use std::collections::HashMap;

use crate::types::SiteInfoItem;
use crate::{dip20, types};
use candid::{CandidType, Deserialize, Nat, Principal};

pub type SiteInfoKeyType = u64;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct SiteInfoDataList {
    pub site_id: SiteInfoKeyType,
    pub site_info_data: HashMap<SiteInfoKeyType, SiteInfoItem>,
    pub pending_list: Vec<SiteInfoItem>,
}

impl SiteInfoDataList {
    pub fn add_func(&mut self, mut site_info: SiteInfoItem) -> Result<(), String> {
        site_info.site_start_time = Some(ic_cdk::api::time());

        let item = SiteInfoItem { ..site_info };
        self.site_id += 1;
        self.site_info_data.insert(self.site_id, item);
        Ok(())
    }
}
