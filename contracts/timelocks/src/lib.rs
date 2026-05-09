#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum DataKey {
    Token,
    Recipient,
    UnlockTime,
    Amount,
}

#[contract]
pub struct Timelock;

#[contractimpl]
impl Timelock {
    pub fn initialize(env: Env, token: Address, recipient: Address, unlock_time: u64, amount: i128) {
        if env.storage().instance().has(&DataKey::Token) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Recipient, &recipient);
        env.storage().instance().set(&DataKey::UnlockTime, &unlock_time);
        env.storage().instance().set(&DataKey::Amount, &amount);
    }
}

// Timelock claim logic is tracked in issue #6.
