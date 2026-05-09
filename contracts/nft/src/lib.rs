#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
pub enum DataKey {
    Admin,
    TokenUri(u64),
    Owner(u64),
    TotalSupply,
}

#[contract]
pub struct NFT;

#[contractimpl]
impl NFT {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0u64);
    }
}

// Full NFT standard implementation is tracked in issue #5.
