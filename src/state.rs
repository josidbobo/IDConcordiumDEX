
#![cfg_attr(not(feature = "std"), no_std)]

use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_std::*;

#[derive(Clone, Serialize, PartialEq, Eq, Debug)]
pub struct TokenInfo<T: IsTokenId> {
    pub id: T,
    pub address: ContractAddress,
}

#[derive(Clone, Serialize, PartialEq, Eq, Debug)]
pub struct TokenOwnerInfo<T: IsTokenId> {
    pub id: T,
    pub address: ContractAddress,
    pub owner: AccountAddress,
}

impl<T: IsTokenId> TokenOwnerInfo<T> {
    pub fn from(token_info: TokenInfo<T>, owner: &AccountAddress) -> Self {
        TokenOwnerInfo {
            owner: *owner,
            id: token_info.id,
            address: token_info.address,
        }
    }
}

#[derive(Clone, Serialize, Copy, PartialEq, Eq, Debug)]
pub struct TokenPriceState<A: IsTokenAmount> {
    pub quantity: A,
    pub price: Amount,
}

#[derive(Debug, Serialize, SchemaType, PartialEq, Eq, Clone)]
pub struct TokenListItem<T: IsTokenId, A: IsTokenAmount> {
    pub token_id: T,
    pub contract: ContractAddress,
    pub price: Amount,
    pub owner: AccountAddress,
    pub quantity: A,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S: HasStateApi, T: IsTokenId, A: IsTokenAmount + Copy> {
    pub token_prices: StateMap<TokenOwnerInfo<T>, TokenPriceState<A>, S>,
}

impl<S: HasStateApi, T: IsTokenId + Copy, A: IsTokenAmount + Copy + ops::Sub<Output = A>>
    State<S, T, A>
{
    /// Creates a new state with the given tokenPrices.
    pub fn new(state_builder: &mut StateBuilder<S>,) -> Self {
        State {
            token_prices: state_builder.new_map(),
        }
    }

    /// Adds a token to Buyable Token List.
    pub fn list_token(
        &mut self,
        token_info: &TokenInfo<T>,
        owner: &AccountAddress,
        price: Amount,
        royalty: u16,
        quantity: A,
    ) {
        match self.token_prices.get(&TokenOwnerInfo::from(token_info.clone(), owner)) {
            // If the token is already listed, do nothing.
            Some(_) => None,
            // If the token is not listed, add it to the list.
            None => self.token_prices.insert(
                TokenOwnerInfo::from(token_info.clone(), owner),
                TokenPriceState {
                    quantity: quantity,
                    price,
                },
            ),
        };
    }

    pub(crate) fn decrease_listed_quantity(&mut self, token_info: &TokenOwnerInfo<T>, delta: A) {
        if let Some(mut price) = self.token_prices.get_mut(token_info) {
            price.quantity = price.quantity - delta;
        }
    }

    /// Gets a token from the buyable token list.
    pub fn get_token(
        &self,
        token_info: &TokenInfo<T>,
        owner: &AccountAddress,
    ) -> Option<TokenPriceState<A>> {
        match self.token_prices.get(&TokenOwnerInfo::from(token_info.clone(), owner)) {
            None => None,
            Some(r) => Some(*r),
        }
    }

    /// Gets a list of all tokens in the buyable token list.
    pub fn list(&self) -> Vec<TokenListItem<T, A>> {
        self.token_prices
            .iter()
            .filter_map(|p| -> Option<TokenListItem<T, A>> {
                let token_info = TokenInfo {
                    id: p.0.id,
                    address: p.0.address,
                };

                Option::Some(TokenListItem {
                        token_id: token_info.id,
                        contract: token_info.address,
                        price: p.1.price,
                        owner: p.0.owner,
                        quantity: p.1.quantity,
                    })
            })
            .collect()
    }
}