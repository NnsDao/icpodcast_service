use crate::ic_wallet::*;
use ic_cdk::api::call::reject;
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use ic_ledger_types::{
    account_balance, AccountBalanceArgs, AccountIdentifier, Subaccount, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct ManagerService {
    pub canister_list: HashMap<Principal, Vec<Principal>>,
    pub old_version_list: Vec<Principal>,
    pub advance_payment: HashMap<Principal, Subaccount>,
    pub sub_account_num: u128,
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

    pub fn advance_payment_addr(&mut self, canister: Principal, caller: Principal) -> String {
        let sub = self.get_transaction_sub_account();
        self.advance_payment.insert(caller, sub);
        get_account(canister, sub)
    }

    fn get_new_sub_account_num(&mut self) -> u128 {
        if self.sub_account_num == u128::MAX {
            self.sub_account_num = 1;
            return 1;
        }
        self.sub_account_num += 1;
        self.sub_account_num
    }

    fn get_transaction_sub_account(&mut self) -> Subaccount {
        let num = self.get_new_sub_account_num();

        let mut default_sub_account = DEFAULT_SUBACCOUNT;
        let num_to_vec = num.to_le_bytes();
        for (index, item) in num_to_vec.iter().enumerate() {
            default_sub_account.0[32 - index - 1] = item.clone()
        }
        default_sub_account
    }

    pub fn get_sub_account(&self, caller: Principal) -> Option<Subaccount> {
        match self.advance_payment.get(&caller) {
            Some(&sub) => return Some(sub.clone()),
            None => return None,
        }
    }

    pub fn del_sub_account(&mut self, caller: Principal) -> () {
        self.advance_payment.remove(&caller);
    }
}
