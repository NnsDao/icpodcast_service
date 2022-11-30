use candid::candid_method;
use std::cell::RefCell;
use std::collections::HashMap;

use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::main::{
    canister_status, deposit_cycles, CanisterIdRecord, CanisterSettings, CanisterStatusResponse,
    CreateCanisterArgument,
};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
use serde::{Deserialize, Serialize};

use owner::*;
use podcast::*;
use types::*;

mod init;
mod owner;
mod podcast;
mod types;
mod tool;

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
pub fn create_base_info(res: SetBaseInfoRes) -> () {
    let info = Info {
        name: res.name,
        icon: res.icon,
        describe: res.describe,
        cover_image: res.cover_image,
        create_at: ic_cdk::api::time(),
        update_at: ic_cdk::api::time(),
    };

    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow_mut().create_info(info));
}

#[update]
#[candid::candid_method(update)]
pub fn update_base_info(res: SetBaseInfoRes) -> () {
    let old_info =
        PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow().get_base_info());
    let info = Info {
        name: res.name,
        icon: res.icon,
        describe: res.describe,
        cover_image: res.cover_image,
        create_at: old_info.create_at,
        update_at: ic_cdk::api::time(),
    };

    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow_mut().create_info(info));
}

#[query]
#[candid::candid_method(query)]
pub fn get_podcast_list() -> HashMap<Id, PodcastIterm> {
    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow().get_podcast_list())
}

#[query]
#[candid::candid_method(query)]
pub fn get_podcast(id: Id) -> Option<PodcastIterm> {
    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow().get_podcast(id))
}

#[update]
#[candid::candid_method(update)]
pub fn create_podcast(podcast: PodcastIterm) -> () {
    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow_mut().create_podcast(podcast))
}

#[update]
#[candid::candid_method(update)]
pub fn update_podcast(id: Id, podcast: PodcastIterm) -> Result<(), String> {
    PODCAST_DATA_STATE
        .with(|podcast_service| podcast_service.borrow_mut().update_podcast(id, podcast))
}

#[query]
#[candid::candid_method(query)]
pub fn get_social_link() -> SocialLink {
    PODCAST_DATA_STATE.with(|podcast_service| podcast_service.borrow().get_social_link())
}

#[update]
#[candid::candid_method(update)]
pub fn set_social_link(social_link: SocialLink) -> () {
    PODCAST_DATA_STATE
        .with(|podcast_service| podcast_service.borrow_mut().set_social_link(social_link))
}

/////////////
//canister//
////////////
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
