use std::collections::HashMap;

use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

pub type Id = u64;

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct PodcastService {
    pub info: Info,
    pub id: Id,
    pub list: HashMap<Id, PodcastIterm>,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct PodcastIterm {
    pub title: String,
    pub describe: String,
    pub link: String,
    pub author: Vec<Principal>,
    pub tag: Tag,
    pub status: bool,
    pub create_at: u64,
    pub update_at: u64,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct Info {
    pub name: String,
    pub describe: String,
    pub create_at: u64,
    pub update_at: u64,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Debug)]
pub enum Tag {
    Default,
}

impl Default for Tag {
    fn default() -> Self {
        Tag::Default
    }
}

impl PodcastService {
    pub fn create(&mut self, iterm: PodcastIterm) {
        let id = self.generate_id();
        self.list.insert(id, iterm);
    }

    pub fn create_info(&mut self, info: Info) {
        self.info = info;
    }

    pub fn generate_id(&mut self) -> Id {
        self.id += 1;
        self.id.clone()
    }

    pub fn get_base_info(&self) -> Info {
        self.info.clone()
    }
}
