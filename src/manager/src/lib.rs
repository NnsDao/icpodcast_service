#![allow(unused_variables, unused_imports)]

use candid::{candid_method, Error};
use std::cell::RefCell;

use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::api::management_canister::main::*;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
use ic_ledger_types::{
    account_balance, transfer, AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Tokens,
    TransferArgs, DEFAULT_FEE, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};
use ic_cdk::api::call::RejectionCode::Unknown;
use manager::*;
use owner::*;
use serde::{Deserialize, Serialize};

mod ic_wallet;
mod init;
mod manager;
mod owner;

thread_local! {
    static OWNER_DATA_STATE: RefCell<OwnerService>  = RefCell::default();
    static MANAGER_DATA_SERVICE: RefCell<ManagerService> = RefCell::default();
}

pub const PAYMENT_BALANCE: u64 = 50000000;
const MANGER_CANISTER: &str = "c526v-pnjpe-x57vs-xe3qb-idgh7-xre3a-jdzef-l654c-5sg4x-5iigp-xae";

pub const WASM: &[u8] = include_bytes!("podcast/podcast.wasm");

/////////
//user//
////////

#[query]
#[candid::candid_method(query)]
pub fn get_podcast_canister() -> Vec<Principal> {
    MANAGER_DATA_SERVICE.with(|s| s.borrow_mut().get_podcast_canister())
}

//////////
//admin//
////////

// #[update(guard="is_admin")]
#[update]
#[candid::candid_method(update)]
pub fn notify_upgrade() -> () {
    MANAGER_DATA_SERVICE.with(|s| s.borrow_mut().upgrade_podcast())
}

#[query]
#[candid::candid_method(query)]
pub fn need_upgrade(canister_id: Principal) -> bool {
    MANAGER_DATA_SERVICE.with(|s| s.borrow().need_upgrade(canister_id))
}

#[update]
#[candid::candid_method(update)]
pub async fn upgrade_podcast(canister_id: Principal) -> CallResult<()> {
    stop_canister(CanisterIdRecord { canister_id }).await?;
    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        canister_id,
        wasm_module: WASM.to_owned(),
        arg: vec![],
    })
    .await?;
    start_canister(CanisterIdRecord { canister_id }).await?;
    MANAGER_DATA_SERVICE.with(|s| s.borrow_mut().upgrade_canister(canister_id));
    Ok(())
}

////////////
//canister//
///////////

#[update]
#[candid::candid_method(update)]
pub async fn get_canister_status(canister_id: Principal) -> CallResult<(CanisterStatusResponse,)> {
    canister_status(CanisterIdRecord { canister_id }).await
}

#[update]
#[candid::candid_method(update)]
pub async fn deposit(canister_id: Principal, cycles: u128) -> CallResult<()> {
    deposit_cycles(CanisterIdRecord { canister_id }, cycles).await
}

#[update]
#[candid::candid_method(update)]
pub async fn canister_stop(canister_id: Principal) -> CallResult<()>  {
    let caller = ic_cdk::caller();
    if !MANAGER_DATA_SERVICE.with(|s| s.borrow().owner_canister(caller, canister_id.clone())) {
        return Err((Unknown, String::from("canister is not exist")));
    }
    stop_canister(CanisterIdRecord{canister_id}).await
}

#[update]
#[candid::candid_method(update)]
pub async fn canister_start(canister_id: Principal) -> CallResult<()> {
    let caller = ic_cdk::caller();
    if !MANAGER_DATA_SERVICE.with(|s| s.borrow().owner_canister(caller, canister_id.clone())) {
        return Err((Unknown, String::from("canister is not exist")));
    }

    start_canister(CanisterIdRecord{canister_id}).await
}

#[update]
#[candid::candid_method(update)]
pub async fn create_podcast_canister() -> CallResult<()> {
    let caller = ic_cdk::caller();
    let canister = ic_cdk::id();

    let opt_sub_account = MANAGER_DATA_SERVICE.with(|s| s.borrow().get_sub_account(caller.clone()));
    if opt_sub_account.is_none() {
        return Err((
            RejectionCode::CanisterReject,
            String::from("Payment was not detected"),
        ));
    }
    let balance = account_balance(
        MAINNET_LEDGER_CANISTER_ID,
        AccountBalanceArgs {
            account: AccountIdentifier::new(&canister.clone(), &opt_sub_account.clone().unwrap()),
        },
    )
    .await;

    match balance {
        Ok(token) => {
            if token.e8s() == PAYMENT_BALANCE {
                MANAGER_DATA_SERVICE.with(|s| s.borrow_mut().del_sub_account(caller.clone()));
            }
        }
        Err(_) => {
            return Err((
                RejectionCode::CanisterReject,
                String::from("Payment was not detected"),
            ))
        }
    }

    transfer(
        MAINNET_LEDGER_CANISTER_ID,
        TransferArgs {
            memo: Memo(0),
            amount: Tokens::from_e8s(50000000 - 10_000),
            fee: DEFAULT_FEE,
            from_subaccount: opt_sub_account,
            to: AccountIdentifier::new(
                &Principal::from_text(MANGER_CANISTER).unwrap(),
                &DEFAULT_SUBACCOUNT,
            ),
            created_at_time: None,
        },
    )
    .await
    .expect("call to ledger failed")
    .expect("transfer failed");

    let canister = create_canister_with_extra_cycles(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![canister.clone(), caller.clone()]),
                compute_allocation: None,
                /// Must be a number between 0 and 2^48^ (i.e 256TB), inclusively.
                memory_allocation: None,
                /// Must be a number between 0 and 2^64^-1, inclusively, and indicates a length of time in seconds.
                freezing_threshold: None,
            }),
        },
        1_000_000_000_000,
    )
    .await?
    .0;

    MANAGER_DATA_SERVICE.with(|s| {
        s.borrow_mut()
            .add_canister(caller, canister.canister_id.clone())
    });

    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister.canister_id,
        wasm_module: WASM.to_owned(),
        arg: vec![],
    })
    .await
}

#[update]
#[candid::candid_method(update)]
fn get_address() -> String {
    let canister = ic_cdk::id();
    let caller = ic_cdk::caller();
    MANAGER_DATA_SERVICE.with(|s| s.borrow_mut().advance_payment_addr(canister, caller))
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable_state_owner = OWNER_DATA_STATE.with(|s| s.take());
    let stable_state_manager = MANAGER_DATA_SERVICE.with(|s| s.take());
    ic_cdk::storage::stable_save((stable_state_owner, stable_state_manager))
        .expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (stable_state_owner, stable_state_manager): (OwnerService, ManagerService) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");

    OWNER_DATA_STATE.with(|s| {
        s.replace(stable_state_owner);
    });
    MANAGER_DATA_SERVICE.with(|s| {
        s.replace(stable_state_manager);
    });
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        println!("{:?}", dir);
        let dir = dir.parent().unwrap().parent().unwrap().join("candid");
        println!("{:?}", dir);
        write(dir.join("manager.did"), export_candid()).expect("Write failed.");
    }
}