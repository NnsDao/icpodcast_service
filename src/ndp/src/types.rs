use candid::{CandidType, Deserialize, Principal};

pub type StakingMonth = u8;

#[derive(CandidType, Deserialize, Clone)]
pub struct StakingItem {
    pub principal: Option<Principal>,
    pub start_time: Option<u64>,
    pub duration: StakingMonth,
    pub token: TokenType,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct SiteItem {
    pub principal: Option<Principal>,
    pub site_name : String,
    pub site_start_time: Option<u64>,
    pub site_url : String,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct SiteInfoItem {
    pub principal: Option<Principal>,
    pub site_name : String,
    pub site_start_time: Option<u64>,
    pub site_title : String,
    pub site_sub_title : String,
    pub site_cover_img : String,
    pub site_url : String,
    pub site_host : String,
    pub site_guests : String,
    pub site_commant : String,
    pub site_season : String,
    pub site_ep : String,
    pub site_avi_url : String,
    pub site_show_notes : String,
    pub site_view : String,
    pub site_status : String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(CandidType, Deserialize, Clone)]
pub enum TokenType {
    NDP(u64),
}
