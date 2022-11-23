use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct SetBaseInfoRes {
    pub name: String,
    pub describe: String,
    pub icon: String,
    pub cover_image: String,
}
