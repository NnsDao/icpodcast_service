use std::collections::HashMap;

use crate::types::SiteItem;
use crate::{dip20, types};
use candid::{CandidType, Deserialize, Nat, Principal};

pub type SiteKeyType = u64;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct SiteDataList {
    pub site_id: SiteKeyType,
    pub site_data: HashMap<SiteKeyType, SiteItem>,
    pub pending_list: Vec<SiteItem>,
}

impl SiteDataList {
    pub fn add_func(&mut self, mut site_item: SiteItem) -> Result<(), String> {
        site_item.site_start_time = Some(ic_cdk::api::time());

        let item = SiteItem { ..site_item };
        self.site_id += 1;
        self.site_data.insert(self.site_id, item);
        Ok(())
    }
}
