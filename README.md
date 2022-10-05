# NEAR vs Soroban: first impressions

The code in this codebase has similar functionality to [Soroban's quick start contract](https://soroban.stellar.org/docs/getting-started/quick-start), so I could compare contract sizes using NEAR's suggested defaults vs Soroban's.

Here's their code:

```rust
#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

#[cfg(test)]
mod test {
    use super::{Contract, ContractClient};
    use soroban_sdk::{symbol, vec, BytesN, Env};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&env, &contract_id);

        let words = client.hello(&symbol!("Dev"));
        assert_eq!(
            words,
            vec![&env, symbol!("Hello"), symbol!("Dev"),]
        );
    }
}
```

Here's the code from [src/lib.rs](./src/lib.rs) in this project:

```rust
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
```

# The Results: Holy contract size, Batman!

Soroban's contract size:

<img width="1081" alt="soroban contract size: 3.3 kilobytes!" src="https://user-images.githubusercontent.com/221614/193348126-cc5c5221-0afd-47b3-9e34-cc9d5a98b15b.png">

NEAR's contract size:

<img width="1049" alt="near contract size: 99 kilobytes!" src="https://user-images.githubusercontent.com/221614/193348205-217c1a83-dc87-4470-ac24-f4ec3db3e3b3.png">

Soroban contracts are 3% the size!

NEAR smart contracts built with Rust are big for two main reasons:

- `std`, the standard library, which Soroban requires you skip (see that `#![no_std]`?)
- JSON serialization for the function arguments, which uses [serde_json](https://docs.rs/serde_json/latest/serde_json/)

I know there are ways to avoid both of these with NEAR smart contracts, but the only people I've known to attempt it are Rust experts. Soroban seems like they're trying to give all developers this superpower by default.

# But it doesn't end there!

With just some [build process changes](https://soroban.stellar.org/docs/tutorials/build-optimized), you can build an _optimized_ version of your Soroban smart contract:

<img width="1186" alt="soroban contract optimized: 477 bytes!" src="https://user-images.githubusercontent.com/221614/193348778-b008f5f2-5ce9-40e0-b381-b7eb4de70e49.png">

3.3 kilobytes wasn't small enough! Had to make it about 10x smaller yet.

Damn, y'all. Settle down.

# Is there any future for Stellar, though?

I met [someone](https://twitter.com/tyvdh) from the Stellar Developer Relations team while at [NEARCON](https://nearcon.org/). When I mentioned Stellar to a NEAR maxi, he said something like "Stellar's not going anywhere, though."

I mean, maybe? 

Stuff I like about Stellar:

- long track record (started in 2013, before Ethereum)
- focused on safety, scalability and global financial inclusion
- have done lots of ([sloppy](https://www.coindesk.com/business/2019/12/13/stellar-tried-to-give-away-2b-xlm-tokens-on-keybase-then-the-spammers-came/)) token giveaways and learned interesting lessons about proving unique humanity & [sybil resistance](https://en.wikipedia.org/wiki/Sybil_attack)

Stuff that's interesting about Soroban, their new smart contract platform:

- uses Wasm

- focused on Rust

- creator of Rust on the team??

   <img width="1294" alt="graydon hoare created rust" src="https://user-images.githubusercontent.com/221614/193350533-87b8d0fc-13b9-47db-a28a-50245eecdbfa.png">
   
   (also, yes, that's a screenshot of [an issue](https://github.com/stellar/rs-soroban-sdk/issues/683) where they're considering switching to WIT, inspired by our work!)

- contract interface / ABI added to the Wasm custom section, [RAEN](https://raen.dev/)-style, by default, so that Every. Single. Contract. will have fully-typed interfaces discoverable by all developers on the platform, right out the gate

- sometimes being late to the game is an asset: can learn from existing Rust & Wasm blockchains, taking what works and tweaking what doesn't

- those contract sizes!

So I dunno, I don't see any particular reason to dunk on them. 
