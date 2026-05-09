#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum DataKey {
    Beneficiary,
    Token,
    StartTime,
    CliffTime,
    EndTime,
    TotalAmount,
    Released,
}

#[contract]
pub struct Vesting;

#[contractimpl]
impl Vesting {
    pub fn initialize(
        env: Env,
        beneficiary: Address,
        token: Address,
        start: u64,
        cliff: u64,
        end: u64,
        amount: i128,
    ) {
        if env.storage().instance().has(&DataKey::Beneficiary) {
            panic!("already initialized");
        }
        assert!(cliff >= start && end > cliff, "invalid schedule");
        env.storage().instance().set(&DataKey::Beneficiary, &beneficiary);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::StartTime, &start);
        env.storage().instance().set(&DataKey::CliffTime, &cliff);
        env.storage().instance().set(&DataKey::EndTime, &end);
        env.storage().instance().set(&DataKey::TotalAmount, &amount);
        env.storage().instance().set(&DataKey::Released, &0i128);
    }
}

// Vesting release logic is tracked in issue #9.
