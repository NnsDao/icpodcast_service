#![allow(unused_variables, unused_imports)]
use crate::metadata::use_admin;
use crate::staking::{Staking, StakingID};
use crate::types::{StakingItem, StakingMonth};
use crate::utils::transfer_ndp;
use candid::{candid_method, error, Principal};
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

mod dip20;
mod metadata;
mod staking;
mod types;
mod utils;

thread_local! {
    static NDPSTAKING:RefCell<staking::Staking> = RefCell::default();
    static METADATA:RefCell<metadata::Metadata> = RefCell::default();
}

#[init]
fn init() {
    METADATA.with(|metadata| {
        let mut meta = metadata.borrow_mut();
        meta.owner = Some(ic_cdk::caller());
        meta.canister_id = ic_cdk::api::id().to_text();
    });
}

#[update]
#[candid_method(update)]
async fn add_staking(mut arg: StakingItem) -> Result<(), String> {
    let caller = ic_cdk::caller();
    arg.principal = Some(caller);
    let amount_min = 1000_0000_0000;
    match arg.token {
        types::TokenType::NDP(amount) => {
            if amount < amount_min {
                Err(format!("Minimum quantity is {}", amount_min))?;
            }
            transfer_ndp(caller, amount).await?;
        }
        _ => Err("unimplemented token!".to_string())?,
    };
    NDPSTAKING.with(|staking| staking.borrow_mut().add(arg))
}

#[query]
#[candid_method(query)]
async fn staking_list() -> HashMap<StakingID, StakingItem> {
    NDPSTAKING.with(|staking| staking.borrow().staking_data.clone())
}

// #[query(guard = "use_admin")]
#[query]
#[candid_method(query)]
fn metadata() -> Result<metadata::Metadata, String> {
    Ok(METADATA.with(|metadata| metadata.borrow().clone()))
}

#[query]
#[candid_method(query)]
pub fn system_time() -> u64 {
    ic_cdk::api::time()
}

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((
        NDPSTAKING.with(|ndp| ndp.borrow().clone()),
        METADATA.with(|metadata| metadata.borrow().clone()),
    ))
    .unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let (old_ndp_staking, old_metadata): (Staking, metadata::Metadata) =
        storage::stable_restore().unwrap();

    NDPSTAKING.with(|ndp| *ndp.borrow_mut() = old_ndp_staking);
    METADATA.with(|data| *data.borrow_mut() = old_metadata);
}

// candid interface
candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
