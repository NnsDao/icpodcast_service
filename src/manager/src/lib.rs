#![allow(unused_variables, unused_imports)]

use candid::candid_method;
use std::cell::RefCell;

use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, deposit_cycles, CanisterIdRecord, CanisterSettings,
    CanisterStatusResponse, CreateCanisterArgument,
};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
use manager::*;
use owner::*;
use serde::{Deserialize, Serialize};

mod init;
mod manager;
mod owner;

thread_local! {
    static OWNER_DATA_STATE: RefCell<OwnerService>  = RefCell::default();
    static MANAGER_DATA_SERVICE: RefCell<ManagerService> = RefCell::default();
}

////////
//user//
///////

//////////
//admin//
////////

#[update]
#[candid::candid_method(update)]
pub async fn get_canister_status(canister_id: Principal) -> CallResult<(CanisterStatusResponse,)> {
    canister_status(CanisterIdRecord { canister_id }).await
}

#[update]
#[candid::candid_method(update)]
pub async fn user_deposit_cycles(canister_id: Principal, cycles: u128) -> CallResult<()> {
    deposit_cycles(CanisterIdRecord { canister_id }, cycles).await
}

#[update]
#[candid::candid_method(update)]
pub async fn create_podcast_canister(canister_id: Principal, cycles: u128) -> CallResult<()> {
    let canister = create_canister(CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(vec![ic_cdk::id(), ic_cdk::caller()]),
            compute_allocation: None,
            /// Must be a number between 0 and 2^48^ (i.e 256TB), inclusively.
            memory_allocation: None,
            /// Must be a number between 0 and 2^64^-1, inclusively, and indicates a length of time in seconds.
            freezing_threshold: None,
        }),
    })
    .await?
    .0;
    let caller = ic_cdk::caller();
    MANAGER_DATA_SERVICE.with(|s| s.borrow_mut().add_canister(caller, canister.canister_id));

    Ok(())
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
