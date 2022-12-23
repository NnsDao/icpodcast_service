use candid::{CandidType, Deserialize, Principal};

pub type StakingMonth = u8;

#[derive(CandidType, Deserialize, Clone)]
pub struct StakingItem {
    pub principal: Option<Principal>,
    pub start_time: Option<u64>,
    pub duration: StakingMonth,
    pub token: TokenType,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(CandidType, Deserialize, Clone)]
pub enum TokenType {
    NDP(u64),
}
