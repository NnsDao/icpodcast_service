use std::collections::HashMap;

use crate::types::StakingItem;
use crate::{dip20, types};
use candid::{CandidType, Deserialize, Nat, Principal};

pub type StakingID = u64;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Staking {
    pub staking_id: StakingID,
    pub staking_data: HashMap<StakingID, StakingItem>,
    pub pending_list: Vec<StakingItem>,
}

impl Staking {
    pub fn add(&mut self, mut staking_item: StakingItem) -> Result<(), String> {
        staking_item.start_time = Some(ic_cdk::api::time());

        let item = StakingItem { ..staking_item };
        self.staking_id += 1;
        self.staking_data.insert(self.staking_id, item);
        Ok(())
    }
}
