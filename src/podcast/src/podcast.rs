use std::collections::HashMap;

use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

pub type Id = u64;

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct PodcastService {
    pub id: Id,
    pub list: Hashmap::Hashmap<Id, PodcastIterm>,
    pub play_record: Hashmap::Hashmap<Id, u64>,
    pub info: Info,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct PodcastIterm {
    pub title: String,
    pub describe: String,
    pub link: String,
    pub author: Vec<Principal>,
    pub status: bool,
    pub create_at: u64,
    pub update_at: u64,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct Info {
    pub name: String,
    pub describe: String,
    pub create_at: u64,
}

impl Podcast {
    pub fn create(&mut self, iterm: PodcastIterm) {}

    pub fn generate_id(&mut self) -> Id {
        self.id += 1;
        self.id
    }
}
