use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct ManagerService {
    pub canister_list: HashMap<Principal, Vec<Principal>>,
    pub old_version_list: Vec<Principal>,
}

impl ManagerService {
    pub fn add_canister(&mut self, caller: Principal, canister: Principal) {
        if let Some(list) = self.canister_list.get_mut(&caller) {
            list.push(canister.clone());
        } else {
            self.canister_list
                .insert(caller.clone(), vec![canister.clone()]);
        }
    }

    pub fn get_podcast_canister(&self) -> Vec<Principal> {
        match self.canister_list.get(&ic_cdk::caller()) {
            Some(list) => list.to_vec().clone(),
            None => vec![],
        }
    }

    pub fn get_old_version_podcast_canister(&self) -> Vec<Principal> {
        self.old_version_list.clone()
    }

    pub fn upgrade_podcast(&mut self) {
        for val in self.canister_list.values() {
            self.old_version_list.append(val.clone().as_mut())
        }
    }

    pub fn need_upgrade(&self, canister_id: Principal) -> bool {
        self.old_version_list.contains(&canister_id)
    }

    pub fn upgrade_canister(&mut self, canister_id: Principal) {
        self.old_version_list
            .retain(|&canister| canister != canister_id);
    }
}
