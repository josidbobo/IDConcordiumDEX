//! Marketplace Contract
//! This module provides implementation of the marketplace contract.
//! Marketplace Contract provides following functions
//! - `list` : returns a list of buyable tokens added to the contract instance.
//! - `add` : adds the token to the list of buyable tokens taking the price of
//!   the token as input.
//! - `transfer` : transfer the authority of the input listed token from one
//!   address to another.
//!
mod errors;
mod parameter;
mod state;

use concordium_cis2::*;
use concordium_std::*;
use errors::DexError;
use parameter::{AddParams, InitParams, TokenList};
use state::{ State, TokenInfo, TokenListItem,};

use crate::{parameter::TransferParams, state::TokenOwnerInfo};

type ContractResult<A> = Result<A, DexError>;

/// Type of token Id used by the CIS2 contract.
type ContractTokenId = TokenIdU8;

/// Type of Token Amount used by the CIS2 contract.
type ContractTokenAmount = TokenAmountU64;

/// Type of state.
type ContractState<S> = State<S, ContractTokenId, ContractTokenAmount>;
type Cis2ClientResult<T> = Result<T, concordium_cis2::Cis2ClientError<()>>;

/// Initializes a new Exchange Contract
///
/// This function can be called by using InitParams.
#[init(contract = "RagnarDEX", parameter = "InitParams")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S, ContractTokenId, ContractTokenAmount>> {
    let params: InitParams = ctx
        .parameter_cursor()
        .get()
        .map_err(|_e| DexError::ParseParams)?;

    ensure!(
        params.amount > 0,
        DexError::InvalidTokenQuantity.into()
    );

    Ok(State::new(state_builder))
}

#[receive(
    contract = "RagnarDEX",
    name = "add",
    parameter = "AddParams",
    mutable
)]
fn add<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<ContractState<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: AddParams = ctx
        .parameter_cursor()
        .get()
        .map_err(|_e| DexError::ParseParams)?;

    let sender_account_address: AccountAddress = match ctx.sender() {
        Address::Account(account_address) => account_address,
        Address::Contract(_) => bail!(DexError::CalledByAContract),
    };

    let token_info = TokenInfo {
        address: params.cis_contract_address,
        id: params.token_id,
    };

    ensure_supports_cis2(host, &params.cis_contract_address)?;
    ensure_is_operator(host, ctx, &params.cis_contract_address)?;
    ensure_balance(
        host,
        params.token_id,
        &params.cis_contract_address,
        sender_account_address,
        params.quantity,
    )?;

    
    host.state_mut().list_token(
        &token_info,
        &sender_account_address,
        params.price,
        params.quantity,
    );

    Ok(())
}

/// Allows for transferring the token specified by TransferParams.
///
/// This function is the buy function where one
/// account can transfer an Asset by paying a price. The transfer will fail of
/// the Amount paid is < token_quantity * token_price
#[receive(
    contract = "RagnarDEX",
    name = "transfer",
    parameter = "TransferParams",
    mutable,
    payable
)]
fn transfer<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<ContractState<S>, StateApiType = S>,
    amount: Amount,
) -> ContractResult<()> {
    let params: TransferParams = ctx
        .parameter_cursor()
        .get()
        .map_err(|_e| DexError::ParseParams)?;

    let token_info = TokenInfo {
        id: params.token_id,
        address: params.cis_contract_address,
    };

    let mut listed_token = host
        .state_mut()
        .get_token(&token_info, &params.owner)
        .ok_or(DexError::TokenNotListed)?;

    let listed_quantity = listed_token.quantity;
    let price_per_unit = listed_token.price;

    ensure!(
        listed_quantity.cmp(&params.quantity).is_ge(),
        DexError::InvalidTokenQuantity
    );

    let price = price_per_unit * params.quantity.0;
    ensure!(
        amount.cmp(&price).is_ge(),
        DexError::InvalidAmountPaid
    );

    let cis2_client = Cis2Client::new(params.cis_contract_address);
    let res: Cis2ClientResult<SupportResult> = cis2_client.supports_cis2(host);
    let res = match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };
    // Checks if the CIS2 contract supports the CIS2 interface.
    let cis2_contract_address = match res {
        SupportResult::NoSupport => bail!(DexError::CollectionNotCis2),
        SupportResult::Support => params.cis_contract_address,
        SupportResult::SupportBy(contracts) => match contracts.first() {
            Some(c) => *c,
            None => bail!(DexError::CollectionNotCis2),
        },
    };

    let cis2_client = Cis2Client::new(cis2_contract_address);
    let res: Cis2ClientResult<bool> = cis2_client.transfer(
        host,
        Transfer {
            amount: params.quantity,
            from: Address::Contract(ctx.self_address()),
            to: Receiver::Account(params.to),   // User that bought the cis2 token
            token_id: params.token_id,
            data: AdditionalData::empty(),
        },
    );

    match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };

    listed_token.price = listed_token.price + Amount::from_micro_ccd(1);

   
    Ok(())
}

#[receive(
    contract = "RagnarDEX",
    name = "receive_ccd",
    parameter = "TransferParams",
    payable,
    mutable
)]
fn receive_ccd<S: HasStateApi>( _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<ContractState<S>, StateApiType = S>,
    _amount: Amount) -> ContractResult<()>{
    Ok(())
}
/// Allows for transferring selling the Cis2 token specified from the user of the Dex for CCD 
///
/// This function is the function where one
/// account can transfer an Asset by paying a price. The transfer will fail of
/// the Amount paid is < token_quantity * token_price
#[receive(
    contract = "RagnarDEX",
    name = "transfer_cis2",
    parameter = "TransferParams",
    mutable,
)]
fn transfer_cis2<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<ContractState<S>, StateApiType = S>,
) -> ContractResult<()> {
    let params: TransferParams = ctx
        .parameter_cursor()
        .get()
        .map_err(|_e| DexError::ParseParams)?;

    let token_info = TokenInfo {
        id: params.token_id,
        address: params.cis_contract_address,
    };

    let mut listed_token = host
        .state_mut()
        .get_token(&token_info, &params.owner)
        .ok_or(DexError::TokenNotListed)?;

    let listed_quantity = listed_token.quantity;
    let price_per_unit = listed_token.price;

    ensure!(
        listed_quantity.cmp(&params.quantity).is_ge(),
        DexError::InvalidTokenQuantity
    );

    let price = price_per_unit * params.quantity.0;
    ensure!(
        host.self_balance().cmp(&price).is_ge(),
        DexError::InsufficientFunds
    );

    let cis2_client = Cis2Client::new(params.cis_contract_address);
    let res: Cis2ClientResult<SupportResult> = cis2_client.supports_cis2(host);
    let res = match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };
    // Checks if the CIS2 contract supports the CIS2 interface.
    let cis2_contract_address = match res {
        SupportResult::NoSupport => bail!(DexError::CollectionNotCis2),
        SupportResult::Support => params.cis_contract_address,
        SupportResult::SupportBy(contracts) => match contracts.first() {
            Some(c) => *c,
            None => bail!(DexError::CollectionNotCis2),
        },
    };

    let cis2_client = Cis2Client::new(cis2_contract_address);
    let res: Cis2ClientResult<bool> = cis2_client.transfer(
        host,
        Transfer {
            amount: params.quantity,
            from: Address::Account(params.owner),
            to: Receiver::Contract(ctx.self_address(), OwnedEntrypointName::new("receive_ccd".to_string()).unwrap(),
        ),   // User that receives the cis2 token
            token_id: params.token_id,
            data: AdditionalData::empty(),
        },
    );

    match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };

    host.invoke_transfer(&params.owner, price)
        .map_err(|_| DexError::InvokeTransferError)?;

    // distribute_amounts(
    //     host,
    //     amount,
    //     &params.owner,
    //     &ctx.owner(),
    // )?;

    listed_token.price = listed_token.price - Amount::from_micro_ccd(1);
    Ok(())
}

/// Returns a list of Added Cis2 Tokens and the token price
#[receive(contract = "RagnarDEX", name = "list", return_value = "TokenList")]
fn list<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<ContractState<S>, StateApiType = S>,
) -> ContractResult<TokenList> {
    let tokens: Vec<TokenListItem<ContractTokenId, ContractTokenAmount>> = host
        .state()
        .list()
        .iter()
        .filter(|t| t.quantity.cmp(&ContractTokenAmount::from(0)).is_gt())
        .cloned()
        .collect::<Vec<TokenListItem<ContractTokenId, ContractTokenAmount>>>();

    Ok(TokenList(tokens))
}

/// Calls the [supports](https://proposals.concordium.software/CIS/cis-0.html#supports) function of CIS2 contract.
/// Returns error If the contract does not support the standard.
fn ensure_supports_cis2<S: HasStateApi, T: IsTokenId + Copy, A: IsTokenAmount + Copy>(
    host: &mut impl HasHost<State<S, T, A>, StateApiType = S>,
    cis_contract_address: &ContractAddress,
) -> ContractResult<()> {
    let cis2_client = Cis2Client::new(*cis_contract_address);
    let res: Cis2ClientResult<SupportResult> = cis2_client.supports_cis2(host);

    let res = match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };

    match res {
        SupportResult::NoSupport => bail!(DexError::CollectionNotCis2),
        SupportResult::SupportBy(_) => Ok(()),
        SupportResult::Support => Ok(()),
    }
}

/// Calls the [operatorOf](https://proposals.concordium.software/CIS/cis-2.html#operatorof) function of CIS contract.
/// Returns error if Current Contract Address is not an Operator of Transaction
/// Sender.
fn ensure_is_operator<S: HasStateApi, T: IsTokenId + Copy, A: IsTokenAmount + Copy>(
    host: &mut impl HasHost<State<S, T, A>, StateApiType = S>,
    ctx: &impl HasReceiveContext<()>,
    cis_contract_address: &ContractAddress,
) -> ContractResult<()> {
    let cis2_client = Cis2Client::new(*cis_contract_address);
    let res: Cis2ClientResult<bool> =
        cis2_client.operator_of(host, ctx.sender(), Address::Contract(ctx.self_address()));
    let res = match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };
    ensure!(res, DexError::NotOperator);
    Ok(())
}

/// Calls the [balanceOf](https://proposals.concordium.software/CIS/cis-2.html#balanceof) function of the CIS2 contract.
/// Returns error if the returned balance < input balance (balance param).
fn ensure_balance<S: HasStateApi, T: IsTokenId + Copy, A: IsTokenAmount + Ord + Copy>(
    host: &mut impl HasHost<State<S, T, A>, StateApiType = S>,
    token_id: T,
    cis_contract_address: &ContractAddress,
    owner: AccountAddress,
    minimum_balance: A,
) -> ContractResult<()> {
    let cis2_client = Cis2Client::new(*cis_contract_address);

    let res: Cis2ClientResult<A> = cis2_client.balance_of(host, token_id, Address::Account(owner));
    let res = match res {
        Ok(res) => res,
        Err(_) => bail!(DexError::Cis2ClientError),
    };
    ensure!(
        res.cmp(&minimum_balance).is_ge(),
        DexError::NoBalance
    );

    Ok(())
}


struct DistributableAmounts {
    to_primary_owner: Amount,
    }

// Distribute the funds to appropriate addresses if needed.
fn distribute_amounts<S: HasStateApi, T: IsTokenId + Copy, A: IsTokenAmount + Copy, E>(
    host: &mut impl HasHost<State<S, T, A>, StateApiType = S, ReturnValueType = E>,
    ctx: &ReceiveContext,
    amount: Amount,
    token_owner: &AccountAddress,
    marketplace_owner: &AccountAddress,
) -> Result<(), DexError> {
    let amounts = calculate_amounts(
        &amount,
    );

    host.invoke_transfer(marketplace_owner, amount)
        .map_err(|_| DexError::InvokeTransferError)?;

    if amounts
        .to_primary_owner
        .cmp(&Amount::from_micro_ccd(0))
        .is_gt()
    {
        host.invoke_transfer(marketplace_owner, amounts.to_primary_owner)
            .map_err(|_| DexError::InvokeTransferError)?;
    }

    if amounts
        .to_primary_owner
        .cmp(&Amount::from_micro_ccd(0))
        .is_gt()
    {
         };

    Ok(())
    
}

/// Calculates the amounts (Commission, Royalty & Selling Price) to be
/// distributed
fn calculate_amounts(
    amount: &Amount,
) -> DistributableAmounts {

    DistributableAmounts {
        to_primary_owner: amount.to_owned()
        }
        
          // distribute_amounts(
    //     host,
    //     amount,
    //     &params.owner,
    //     &ctx.owner(),
    // )?;

    // host.state_mut().decrease_listed_quantity(
    //     &TokenOwnerInfo::from(token_info, &params.owner),
    //     params.quantity,
    // );

}

#[concordium_cfg_test]
mod test {
    use crate::{
        add, calculate_amounts, list,
        parameter::AddParams,
        state::{ State, TokenInfo, TokenListItem, TokenPriceState,},
        ContractState, ContractTokenAmount, ContractTokenId,
    };
    use concordium_cis2::*;

    use concordium_std::{test_infrastructure::*, *};

    const ACCOUNT_0: AccountAddress = AccountAddress([0u8; 32]);
    const ADDRESS_0: Address = Address::Account(ACCOUNT_0);
    const CIS_CONTRACT_ADDRESS: ContractAddress = ContractAddress {
        index: 1,
        subindex: 0,
    };
    const MARKET_CONTRACT_ADDRESS: ContractAddress = ContractAddress {
        index: 2,
        subindex: 0,
    };

    #[concordium_test]
    fn should_add_token() {
        let token_id_1 = ContractTokenId::from(1);
        let token_quantity_1 = ContractTokenAmount::from(1);
        let price = Amount::from_ccd(1);

        let mut ctx = TestReceiveContext::default();
        ctx.set_sender(ADDRESS_0);
        ctx.set_self_address(MARKET_CONTRACT_ADDRESS);

        let add_params = AddParams {
            cis_contract_address: CIS_CONTRACT_ADDRESS,
            price,
            token_id: token_id_1,
            quantity: token_quantity_1,
        };
        let parameter_bytes = to_bytes(&add_params);
        ctx.set_parameter(&parameter_bytes);

        let mut state_builder = TestStateBuilder::new();
        let state = State::new(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        fn mock_supports(
            _p: Parameter,
            _a: Amount,
            _a2: &mut Amount,
            _s: &mut ContractState<TestStateApi>,
        ) -> Result<(bool, SupportsQueryResponse), CallContractError<SupportsQueryResponse>>
        {
            Ok((
                false,
                SupportsQueryResponse {
                    results: vec![SupportResult::Support],
                },
            ))
        }

        fn mock_is_operator_of(
            _p: Parameter,
            _a: Amount,
            _a2: &mut Amount,
            _s: &mut ContractState<TestStateApi>,
        ) -> Result<(bool, OperatorOfQueryResponse), CallContractError<OperatorOfQueryResponse>>
        {
            Ok((false, OperatorOfQueryResponse { 0: vec![true] }))
        }

        fn mock_balance_of(
            _p: Parameter,
            _a: Amount,
            _a2: &mut Amount,
            _s: &mut ContractState<TestStateApi>,
        ) -> Result<
            (bool, BalanceOfQueryResponse<ContractTokenAmount>),
            CallContractError<BalanceOfQueryResponse<ContractTokenAmount>>,
        > {
            Ok((false, BalanceOfQueryResponse(vec![1.into()])))
        }

        TestHost::setup_mock_entrypoint(
            &mut host,
            CIS_CONTRACT_ADDRESS,
            OwnedEntrypointName::new_unchecked("supports".to_string()),
            MockFn::new_v1(mock_supports),
        );

        TestHost::setup_mock_entrypoint(
            &mut host,
            CIS_CONTRACT_ADDRESS,
            OwnedEntrypointName::new_unchecked("operatorOf".to_string()),
            MockFn::new_v1(mock_is_operator_of),
        );

        TestHost::setup_mock_entrypoint(
            &mut host,
            CIS_CONTRACT_ADDRESS,
            OwnedEntrypointName::new_unchecked("balanceOf".to_string()),
            MockFn::new_v1(mock_balance_of),
        );

        let res = add(&ctx, &mut host);

        claim!(res.is_ok(), "Results in rejection");
        claim!(
            host.state().token_prices.iter().count() != 0,
            "Token not added"
        );
        
        

        let token_list_tuple = host
            .state().add(
                &TokenInfo {
                    id: token_id_1,
                    address: CIS_CONTRACT_ADDRESS,
                },
                &ACCOUNT_0,
            )
            .expect("Should not be None");

        
        claim_eq!(
            token_list_tuple.1.to_owned(),
            TokenPriceState {
                price,
                quantity: token_quantity_1
            },
        )
    }
}
