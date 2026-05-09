#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

#[contracttype]
pub enum DataKey {
    Admin,
    Metadata,
}

#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

#[contract]
pub struct FungibleToken;

#[contractimpl]
impl FungibleToken {
    /// Initialize the token with metadata and an admin.
    pub fn initialize(env: Env, admin: Address, metadata: TokenMetadata) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Metadata, &metadata);
    }
}

// SEP-41 interface implementation is tracked in issue #4.
