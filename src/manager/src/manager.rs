use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct ManagerService {
    pub canister_list: HashMap<Principal, Vec<Principal>>,
}

impl ManagerService {
    pub fn add_canister(&mut self, caller: Principal, canister: Principal) {
        if let Some(list) = self.canister_list.get_mut(&caller) {
            list.push(canister.clone())
        } else {
            self.canister_list.insert(caller.clone(), vec![canister.clone()]),
        }
    }
}
