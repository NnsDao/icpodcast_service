use candid::candid_method;
use std::cell::RefCell;

use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
use serde::{Deserialize, Serialize};

use owner::*;
use podcast::*;

mod init;
mod owner;
mod podcast;

thread_local! {
    static OWNER_DATA_STATE: RefCell<OwnerService>  = RefCell::default();
    static PODCAST_DATA_STATE: RefCell<PodcastService> = RefCell::default();
}

////////
//user//
///////
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

///////////////
//podcastBase//
//////////////

#[query]
#[candid::candid_method(query)]
pub fn get_podcast_base_info() -> Info {
    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow().get_base_info())
}

#[update]
#[candid::candid_method(update)]
pub fn modify_info() -> () {}

#[update]
#[candid::candid_method(update)]
pub fn create() -> () {
    let info = Info {
        name: String::from("aaaaa"),
        describe: String::from("ddddd"),
        create_at: ic_cdk::api::time(),
        update_at: ic_cdk::api::time(),
    };

    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow_mut().create_info(info));
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable_state_owner = OWNER_DATA_STATE.with(|s| s.take());
    let stable_state_podcast = PODCAST_DATA_STATE.with(|s| s.take());
    ic_cdk::storage::stable_save((stable_state_owner, stable_state_podcast))
        .expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (stable_state_owner, stable_state_podcast): (OwnerService, PodcastService) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");
    OWNER_DATA_STATE.with(|s| {
        s.replace(stable_state_owner);
    });

    PODCAST_DATA_STATE.with(|s| {
        s.replace(stable_state_podcast);
    });
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
