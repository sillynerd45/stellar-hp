#![no_std]
use crate::enums::ContractKey;
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Vec};

#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, stellar_hp_admin: Address) {
        env.storage()
            .instance()
            .set(&ContractKey::Admin, &stellar_hp_admin);
    }

    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }

    pub fn hello_admin(env: Env, to: String) -> Vec<String> {
        let admin: Address = env.storage().instance().get(&ContractKey::Admin).unwrap();
        admin.require_auth();

        vec![&env, String::from_str(&env, "Admin saying hello"), to]
    }
}

mod enums;
mod test;
