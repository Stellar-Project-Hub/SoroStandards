#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
pub enum DataKey {
    Signers,
    Threshold,
    Nonce,
}

#[contract]
pub struct Multisig;

#[contractimpl]
impl Multisig {
    pub fn initialize(env: Env, signers: Vec<Address>, threshold: u32) {
        if env.storage().instance().has(&DataKey::Threshold) {
            panic!("already initialized");
        }
        assert!(threshold as usize <= signers.len() as usize, "threshold > signers");
        env.storage().instance().set(&DataKey::Signers, &signers);
        env.storage().instance().set(&DataKey::Threshold, &threshold);
        env.storage().instance().set(&DataKey::Nonce, &0u64);
    }
}

// Full multisig execution logic is tracked in issue #7.
