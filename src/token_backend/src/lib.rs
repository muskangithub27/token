use ic_cdk_macros::{init, update, query};
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use ic_cdk::storage;

#[derive(CandidType, Serialize, Deserialize, Clone)]  // Added Clone
struct Token {
    id: u32,
    owner: Principal,
    balance: u64,
}

#[init]
fn init() {
    let token = Token {
        id: 1,
        owner: ic_cdk::caller(),
        balance: 1000,
    };

    let serialized = bincode::serialize(&token).unwrap();
    storage::stable_save((serialized,)).unwrap();
}

#[update]
fn update_token(new_balance: u64) -> Token {  // Added return type
    let mut token: Token = load_token();
    token.balance = new_balance;

    let serialized = bincode::serialize(&token).unwrap();
    storage::stable_save((serialized,)).unwrap();
    token  // Return the updated token
}

#[query]
fn read_token() -> Token {
    load_token()
}

fn load_token() -> Token {
    let (buf,): (Vec<u8>,) = storage::stable_restore().unwrap();
    bincode::deserialize(&buf).unwrap()
}

// Required: Export the Candid interface
ic_cdk::export_candid!();