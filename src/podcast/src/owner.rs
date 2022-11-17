use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use crate::OWNER_DATA_STATE;

pub fn is_owner() -> Result<(), String> {
    OWNER_DATA_STATE.with(|owner_service|owner_service.borrow().is_owner(ic_cdk::caller()))
}

pub fn is_admin() -> Result<(), String> {
    OWNER_DATA_STATE.with(|owner_service|owner_service.borrow().is_admin(ic_cdk::caller()))
}


#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct OwnerService {
    pub owners: Vec<Principal>,
    pub admin: Option<Principal>,
}

impl OwnerService {

    pub fn init_admin(&mut self, admin : Principal) -> () {
        self.admin = Some(admin)
    }

    pub fn change_admin(&mut self, admin : Principal) -> () {
        self.admin = Some(admin)
    }

    pub fn is_admin(&self, caller: Principal) -> Result<(), String> {
        if self.admin == Some(caller) {
            return Ok(());
        }
        Err("no auth".to_owned())
    }

    pub fn add_owner(&mut self, principal: Principal) -> () {
        self.owners.push(principal)
    }

    pub fn get_owners(&self) -> Vec<Principal> {
        self.owners.clone()
    }

    pub fn is_owner(&self, caller: Principal) -> Result<(), String> {
        for owner in self.owners.clone() {
            if owner == caller {
                return Ok(());
            }
        }

        Err("no auth".to_owned())
    }

    pub fn delete_owner(&mut self, person: Principal) -> () {
        self.owners.retain(|owner|*owner != person);
    }
}
