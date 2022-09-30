use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter;

#[near_bindgen]
impl Counter {
    pub fn hello(&self, to: &AccountId) -> (String, AccountId) {
        (String::from("hello"), to.clone())
    }
}

/*
 * The rest of this file sets up unit tests
 * Run with `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        // instantiate a contract variable with the counter at zero
        let contract = Counter;
        let account_id: AccountId = "near".parse().unwrap();
        assert_eq!(
            ("hello".to_string(), account_id.clone()),
            contract.hello(&account_id)
        );
    }
}
