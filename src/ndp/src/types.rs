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

#[allow(clippy::upper_case_acronyms)]
#[derive(CandidType, Deserialize, Clone)]
pub enum TokenType {
    NDP(u64),
}
