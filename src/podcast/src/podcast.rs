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
    pub tag: Vec<String>,
    pub social_link: SocialLink,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct PodcastIterm {
    pub title: String,
    pub describe: String,
    pub sub_title: String,
    pub link: String,
    pub author: Vec<Principal>,
    pub categories: Categories,
    pub language: Language,
    pub status: bool,
    pub create_at: u64,
    pub update_at: u64,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct Info {
    pub name: String,
    pub describe: String,
    pub icon: String,
    pub cover_image: String,
    pub create_at: u64,
    pub update_at: u64,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Debug)]
pub enum Categories {
    Default,
}

impl Default for Categories {
    fn default() -> Self {
        Categories::Default
    }
}

#[derive(CandidType, Clone, Deserialize, Serialize, Debug)]
pub enum Language {
    English,
    Chinese,
    Japanese,
    Korean,
    Arabic,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default, Debug)]
pub struct SocialLink {
    pub email: String,
    pub telegram: String,
    pub twitter: String,
    pub github: String,
    pub instagram: String,
    pub openchat: String,
    pub distrikt: String,
    pub dscvr: String,
    pub dmail: String,
}

impl PodcastService {
    pub fn create_podcast(&mut self, podcast: PodcastIterm) {
        let id = self.generate_id();
        self.list.insert(id, podcast);
    }

    pub fn update_podcast(&mut self, id: Id, podcast: PodcastIterm) -> Result<(), String> {
        if let Some(item) = self.list.get(&id) {
            self.list.insert(
                id,
                PodcastIterm {
                    title: podcast.title,
                    describe: podcast.describe,
                    sub_title: podcast.sub_title,
                    link: podcast.link,
                    author: podcast.author,
                    categories: podcast.categories,
                    language: podcast.language,
                    status: podcast.status,
                    create_at: item.create_at,
                    update_at: ic_cdk::api::time(),
                },
            );
        }
        return Err(String::from("id is not exist"));
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

    pub fn get_social_link(&self) -> SocialLink {
        self.social_link.clone()
    }

    pub fn set_social_link(&mut self, social_link: SocialLink) {
        self.social_link = social_link
    }

    pub fn get_podcast_list(&self) -> HashMap<Id, PodcastIterm> {
        self.list.clone()
    }

    pub fn get_podcast(&self, id: Id) -> Option<PodcastIterm> {
        if let Some(item) = self.list.get(&id) {
            return Some(item.clone());
        }
        return None;
    }
}
