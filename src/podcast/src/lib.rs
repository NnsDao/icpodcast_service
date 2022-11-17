use std::cell::RefCell;

use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
use serde::{Deserialize, Serialize};

use owner::*;

mod init;
mod owner;
mod podcast;

thread_local! {
    static OWNER_DATA_STATE: RefCell<OwnerService>  = RefCell::default();
}

#[query]
#[candid::candid_method(query)]
pub fn get_owner() -> Vec<Principal> {
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow().get_owners())
}

#[query]
#[candid::candid_method(query)]
pub fn get_admin() -> Option<Principal> {
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow().get_admin())
}

// #[update(guard="is_owner")]
#[update]
#[candid::candid_method(update)]
pub fn add_owner(person: Principal) -> () {
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow_mut().add_owner(person))
}

// #[update(guard="is_admin")]
#[update]
#[candid::candid_method(update)]
pub fn change_admin(person: Principal) -> () {
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow_mut().change_admin(person))
}

// #[update(guard="is_admin")]
#[update]
#[candid::candid_method(update)]
pub fn delete_owner(person: Principal) -> () {
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow_mut().delete_owner(person))
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable_state = OWNER_DATA_STATE.with(|s| s.take());
    ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (stable_state,) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");
    OWNER_DATA_STATE.with(|s| {
        s.replace(stable_state);
    });
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
