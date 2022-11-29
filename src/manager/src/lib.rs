#![allow(unused_variables, unused_imports)]

use candid::candid_method;
use std::cell::RefCell;

use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::main::*;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
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
pub async fn create_podcast_canister() -> CallResult<()> {
    let caller = ic_cdk::caller();
    let canister = create_canister_with_extra_cycles(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id(), caller.clone()]),
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
