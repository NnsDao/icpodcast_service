use ic_cdk::export::candid::Principal;
use ic_ledger_types::{AccountIdentifier, Subaccount};

pub fn get_account(canister: Principal, sub: Subaccount) -> String {
    AccountIdentifier::new(&canister, &sub).to_string()
}
