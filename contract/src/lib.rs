/*
 *   near_NFT_app WIP
 *
 */
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, Promise, PromiseOrValue};

use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata,
    NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;

// define constant values
const GAS: u128             = 200000000000000;
const MINT_DEPOSIT: u128    = 100000000000000000000000; // 0.1 NEAR
const FIXED_PRICE: u128     = 2900000000000000000000000; // 2.9 NEAR

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

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
       assert!(amount_u128 == FIXED_PRICE,
           "The transfer amount {} is different from the price: {}.",
            amount_u128, FIXED_PRICE);
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
        let p: String =  FIXED_PRICE.to_string();
        return p
    }
    pub fn get_mint_deposit(&self) -> String {
        let p: String =  MINT_DEPOSIT.to_string();
        return p
    }
    pub fn get_gas_price(&self) -> String {
        let p: String = GAS.to_string();
        return p
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

// tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    const MINT_DEPOSIT: u128 = 5870000000000000000000;
    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::default();

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_DEPOSIT)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        let token = contract.mint_now(token_id.clone(), accounts(0), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0));
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
    }

    #[test]
    fn test_buy() {
        let mut context = get_context(accounts(1));
        let mut contract = Contract::default();

        let correct_amount = FIXED_PRICE;
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(correct_amount)
            .build());

        contract.near_transfer(accounts(0), correct_amount.to_string());
    }

    #[test]
    #[should_panic]
    fn test_buy_with_less_near() {
        let mut context = get_context(accounts(1));
        let mut contract = Contract::default();

        let correct_amount = FIXED_PRICE - 1;
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(correct_amount)
            .build());

        contract.near_transfer(accounts(0), correct_amount.to_string());
    }

    #[test]
    #[should_panic]
    fn test_buy_with_more_near() {
        let mut context = get_context(accounts(1));
        let mut contract = Contract::default();

        let correct_amount = FIXED_PRICE + 1;
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(correct_amount)
            .build());

        contract.near_transfer(accounts(0), correct_amount.to_string());
    }
    #[test]
    fn test_get_price(){
        let mut context = get_context(accounts(1));
        let contract = Contract::default();
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(0)
            .build());

        assert_eq!(
            contract.get_price(),
            FIXED_PRICE.to_string()
        );
    }

}
