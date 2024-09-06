#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::*;
use concordium_cis2::*;
mod techfiestaToken;

// Import the token contract
use crate::techfiestaToken::techFiestaToken::{ContractTokenId, ContractTokenAmount, State as TokenState};

// Constants
const TOKEN_CONTRACT_NAME: &str = "techFiestaToken";
const TOKEN_ID: ContractTokenId = ContractTokenId::from(0); // Assuming token ID 0

// Types
type ContractTokenPrice = u64;

#[derive(Serial, Deserial, SchemaType)]
pub(crate) struct TransferParams {
    /// Address of the CIS2 Contract. Contract containing token to be transferred.
    pub cis_contract_address: ContractAddress,

    /// Token ID of the token to be transferred.
    pub token_id: ContractTokenId,

    /// Address of the receiver of the token.
    pub to: AccountAddress,

    /// Current owner of the Token.
    pub owner: AccountAddress,

    /// Quantity of the token to be transferred.
    pub quantity: ContractTokenAmount,
}

// Events
#[derive(Serialize)]
enum DexEvent {
    TokensPurchased {
        buyer: AccountAddress,
        ccd_amount: Amount,
        tokens_amount: ContractTokenAmount,
    },
    TokensSold {
        seller: AccountAddress,
        ccd_amount: Amount,
        tokens_amount: ContractTokenAmount,
    },
}

// Contract state
#[derive(Serial, Deserial)]
pub struct State {
    token_price: ContractTokenPrice,
    token_balance: ContractTokenAmount,
}

// Contract errors
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum DexError {
    InsufficientCCD,
    InsufficientTokens,
    InvalidTokenId,
    TokenTransferFailed,
    Unauthorized,
    ParseParams
}

// Init function
#[init(contract = "techFiestaDex", parameter = "ContractTokenPrice")]
fn dex_init(ctx: &InitContext, state_builder: &mut StateBuilder) -> InitResult<State> {
    let initial_price: ContractTokenPrice = ctx.parameter_cursor().get()?;
    Ok(State {
        token_price: initial_price,
        token_balance: ContractTokenAmount::from(0),
    })
}

// Buy tokens
#[receive(
    contract = "techFiestaDex",
    name = "buyTokens",
    parameter = "TransferParams",
    payable,
    error = "DexError",
    enable_logger,
    mutable
)]
fn buy_tokens(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    amount: Amount,
    logger: &mut Logger,
) -> Result<(), DexError> {
    let state = host.state_mut();

    let params: TransferParams = ctx
    .parameter_cursor()
    .get()
    .map_err(|_e| DexError::ParseParams)?;

    
    let tokens_to_buy = ContractTokenAmount::from(amount.micro_ccd() / state.token_price);
    ensure!(tokens_to_buy > 0.into(), DexError::InsufficientCCD);

    // Transfer tokens from the DEX contract to the buyer
    let transfer_params = Transfer {
        from: Address::Contract(ctx.self_address()),
        to: Receiver::Account(params.to),
        token_id: params.token_id,
        amount: params.quantity,
        data: AdditionalData::empty(),
    };

    let token_contract = ContractAddress::new(0, TOKEN_CONTRACT_NAME.parse().unwrap());
    host.invoke_contract(
        &token_contract,
        &transfer_params,
        EntrypointName::new("transfer").unwrap(),
        Amount::zero(),
    )
    .map_err(|_| DexError::TokenTransferFailed)?;

    // Update state
    state.token_balance -= tokens_to_buy;

    // Log event
    logger.log(&DexEvent::TokensPurchased {
        buyer: params.to,
        ccd_amount: amount,
        tokens_amount: tokens_to_buy,
    });

    Ok(())
}

// Sell tokens
#[receive(
    contract = "techFiestaDex",
    name = "sellTokens",
    parameter = "TransferParams",
    error = "DexError",
    enable_logger,
    mutable
)]
fn sell_tokens(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> Result<(), DexError> {
    let state = host.state_mut();
    //let tokens_to_sell: ContractTokenAmount = ctx.parameter_cursor().get()?;
    let params : TransferParams = ctx.parameter_cursor().get().map_err(|_e| DexError::ParseParams)?;
    
    let ccd_to_receive = Amount::from_micro_ccd(params.quantity.0 * state.token_price);
    ensure!(host.self_balance() >= ccd_to_receive, DexError::InsufficientCCD);

    // Transfer tokens from the seller to the DEX contract
    let transfer_params = Transfer {
        from: ctx.sender(),
        to: Receiver::Contract(ctx.self_address(), OwnedEntrypointName::as_entrypoint_name("receiveTokens".to_string())),
        token_id: params.token_id,
        amount: params.quantity,
        data: AdditionalData::empty(),
    };

    let token_contract = ContractAddress::new(0, TOKEN_CONTRACT_NAME.parse().unwrap());
    host.invoke_contract(
        &token_contract,
        &transfer_params,
        EntrypointName::new("transfer").unwrap(),
        Amount::zero(),
    )
    .map_err(|_| DexError::TokenTransferFailed)?;

    // Transfer CCD to the seller
    host.invoke_transfer(&params.to, ccd_to_receive)
        .map_err(|_| DexError::InsufficientCCD)?;

    // Update state
    state.token_balance += params.quantity;

    // Log event
    logger.log(&DexEvent::TokensSold {
        seller: params.owner,
        ccd_amount: ccd_to_receive,
        tokens_amount: params.quantity,
    });

    Ok(())
}

// Receive tokens (called by the token contract during sellTokens)
#[receive(
    contract = "techFiestaDex",
    name = "receiveTokens",
    parameter = "OnReceivingCis2Params<ContractTokenId, ContractTokenAmount>",
    mutable
)]
fn receive_tokens(_ctx: &ReceiveContext, host: &mut Host<State>) -> ReceiveResult<()> {
    let params: OnReceivingCis2Params<ContractTokenId, ContractTokenAmount> = _ctx.parameter_cursor().get()?;
    ensure!(params.token_id == TOKEN_ID, DexError::InvalidTokenId.into());
    host.state_mut().token_balance += params.amount;    
    Ok(())
}

// View function
#[receive(contract = "techFiestaDex", name = "view", return_value = "State")]
fn contract_view(_ctx: &ReceiveContext, host: &Host<State>) -> ReceiveResult<State> {
    Ok(*host.state())
}

// Update token price (only by contract owner)
#[receive(
    contract = "techFiestaDex",
    name = "updatePrice",
    parameter = "ContractTokenPrice",
    mutable
)]
fn update_price(ctx: &ReceiveContext, host: &mut Host<State>) -> ReceiveResult<()> {
    ensure!(ctx.sender().matches_account(&ctx.owner()), DexError::Unauthorized.into());
    let new_price: ContractTokenPrice = ctx.parameter_cursor().get()?;
    host.state_mut().token_price = new_price;
    Ok(())
}