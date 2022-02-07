/*
 *   near_NFT_app WIP
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {

}

impl Default for Contract {
  fn default() -> Self {
    Self {

    }
  }
}

#[near_bindgen]
impl Contract {

    // add methods
}
