use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

use crate::OWNER_DATA_STATE;

pub fn is_admin() -> Result<(), String> {
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow().is_admin(ic_cdk::caller()))
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct OwnerService {
    pub admin: Option<Principal>,
}

impl OwnerService {
    pub fn init_admin(&mut self, admin: Principal) -> () {
        self.admin = Some(admin)
    }

    pub fn change_admin(&mut self, admin: Principal) -> () {
        self.admin = Some(admin)
    }

    pub fn get_admin(&self) -> Option<Principal> {
        self.admin.clone()
    }

    pub fn is_admin(&self, caller: Principal) -> Result<(), String> {
        if self.admin == Some(caller) {
            return Ok(());
        }
        Err("no auth".to_owned())
    }
}
