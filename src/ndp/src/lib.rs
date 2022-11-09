#![allow(unused_variables, unused_imports)]
use crate::metadata::use_admin;
use crate::staking::{Staking, StakingID};
use crate::sitedata::{SiteDataList, SiteKeyType};
use crate::siteinfo::{SiteInfoDataList, SiteInfoKeyType};
use crate::types::{StakingItem,SiteItem, SiteInfoItem, StakingMonth};
use crate::utils::transfer_ndp;
use candid::{candid_method, error, Principal};
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

mod dip20;
mod metadata;
mod staking;
mod sitedata;
mod siteinfo;
mod types;
mod utils;

thread_local! {
    static NDPSTAKING:RefCell<staking::Staking> = RefCell::default();
    static SITEDATAMOD:RefCell<sitedata::SiteDataList> = RefCell::default();
    static SITEINFOMOD:RefCell<siteinfo::SiteInfoDataList> = RefCell::default();
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

#[query]
#[candid_method(query)]
pub fn system_t_time() -> u64 {
    ic_cdk::api::time()
}


#[update]
#[candid_method(update)]
async fn add_site_data(mut arg: SiteItem) -> Result<(), String> {
    let caller = ic_cdk::caller();
    // println!("{:#?}",caller);
    arg.principal = Some(caller);
    // 8个0精度单位 也就是这里是100元
    // let amount_min: u64 = 100_0000_0000;
    SITEDATAMOD.with(|site_data| site_data.borrow_mut().add_func(arg))
    // Err(format!("Minimum quantity is {}", amount_min))
}


#[query]
#[candid_method(query)]
async fn get_site_list() -> HashMap<SiteKeyType, SiteItem> {
    // HashMap<SiteKeyType, SiteItem>
    // Result<SiteDataList, String>
    
    
    // let caller = ic_cdk::caller();
    // arg.principal = Some(caller);
    // 8个0精度单位 也就是这里是100元
    // let amount_min: u64 = 100_0000_0000;
    SITEDATAMOD.with(|site_data| site_data.borrow().site_data.clone())
    // Err(format!("Minimum quantity is {}", amount_min))
}

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((
        NDPSTAKING.with(|ndp| ndp.borrow().clone()),
        SITEDATAMOD.with(|sitedata| sitedata.borrow().clone()),
        METADATA.with(|metadata| metadata.borrow().clone()),
        SITEINFOMOD.with(|siteinfodata| siteinfodata.borrow().clone()),
    ))
    .unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let (old_ndp_staking, old_metadata,old_site_data,old_site_info): (Staking, metadata::Metadata,sitedata::SiteDataList,siteinfo::SiteInfoDataList) =
        storage::stable_restore().unwrap();

    NDPSTAKING.with(|ndp| *ndp.borrow_mut() = old_ndp_staking);
    SITEDATAMOD.with(|sitedata| *sitedata.borrow_mut() = old_site_data);
    METADATA.with(|data| *data.borrow_mut() = old_metadata);
    SITEINFOMOD.with(|siteinfodata| *siteinfodata.borrow_mut() = old_site_info);
}

// candid interface
candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
