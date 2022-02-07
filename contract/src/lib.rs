/*
 *   near_NFT_app WIP
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption};

use near_contract_standards::non_fungible_token::metadata::{
     NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
 };
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::{
     env, near_bindgen, AccountId, BorshStorageKey, Promise, PromiseOrValue,
 };

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

const FIXED_PRICE: u128 = 2900000000000000000000000; // 2.9 NEAR

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

impl Default for Contract{
    fn default() -> Self {
      Self {
          tokens: NonFungibleToken::new(
              StorageKey::NonFungibleToken,
              env::current_account_id(),
              Some(StorageKey::TokenMetadata),
              Some(StorageKey::Enumeration),
              Some(StorageKey::Approval),
          ),
          metadata: LazyOption::new(StorageKey::Metadata, Some(&(NFTContractMetadata {
          spec: NFT_METADATA_SPEC.to_string(),
          name: "Example NEAR non-fungible token".to_string(),
          symbol: "EXAMPLE".to_string(),
          icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
          base_uri: None,
          reference: None,
          reference_hash: None,
      }))),
      }
  }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn near_transfer(&mut self, to: AccountId, amount: String) -> Promise {
       let balance: u128 = env::attached_deposit();

       assert!(
           env::is_valid_account_id(to.as_bytes()),
           "Account @{} is invalid", to
       );
       let amount_u128: u128 =
           match amount.parse() {
               Ok(value) => value,
               _ => panic!("Amount value error")
           };
       assert!(amount_u128 == FIXED_PRICE);
       assert_eq!(
           amount_u128,
           balance,
           "Not enough tokens (Supplied: {}. Demand: {})",
           amount_u128,
           balance
       );

       Promise::new(to)
           .transfer(amount_u128.clone())
           .into()

       }

    #[payable]
    pub fn mint_now(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }
    // view methods
    pub fn get_price(&self) -> String {
        FIXED_PRICE.to_string()
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
