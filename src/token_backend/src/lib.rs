use ic_cdk_macros::{init, update, query};
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use ic_cdk::storage;

#[derive(CandidType, Serialize, Deserialize, Clone)]
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
fn send_token(to: Principal, amount: u64) -> String {
    let mut token: Token = load_token();
    if token.balance < amount {
        return "Insufficient balance".to_string();
    }

    token.balance -= amount;

    let serialized = bincode::serialize(&token).unwrap();
    storage::stable_save((serialized,)).unwrap();

    // Normally, you'd update the recipient's balance in a real-world app.
    // Here, we just log the action.
    format!("Sent {} tokens to {}", amount, to)
}

#[update]
fn mint_token(amount: u64) -> Token {
    let mut token: Token = load_token();
    token.balance += amount;

    let serialized = bincode::serialize(&token).unwrap();
    storage::stable_save((serialized,)).unwrap();
    token
}

#[query]
fn read_token() -> Token {
    load_token()
}

fn load_token() -> Token {
    let (buf,): (Vec<u8>,) = storage::stable_restore().unwrap();
    bincode::deserialize(&buf).unwrap()
}

// Function to fetch the receivers toekn balance by receiver principal
#[query]
fn get_balance(owner: Principal) -> u64 {
    let token: Token = load_token();
    token.balance
}



// Export the Candid interface
ic_cdk::export_candid!();
