use kvds::connector::connector::Connector;

use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{self};

use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use crate::models::{
    name_basic::NameBasic, title_basic::TitleBasic, title_crew::TitleCrew,
    title_principal::TitlePrincipal, title_rating::TitleRating,
};

pub const STORE_INTERNALLY: &str = "STORE-INTERNALLY";

pub const DB_URL: &str = "DB-URL";

pub static ID_TITLE: Lazy<Mutex<MapStorage<TitleBasic>>> =
    Lazy::new(|| Mutex::new(MapStorage::new()));

pub struct MapStorage<T> {
    map: HashMap<String, T>,
    internal: bool,
    type_name: String,
    connector: Option<Box<Connector>>,
}

impl<T> MapStorage<T>
where
    T: serde::de::DeserializeOwned + Clone + Serialize,
{
    pub fn new() -> Self {
        let is_internal = env::var(STORE_INTERNALLY).unwrap() == "true";
        MapStorage {
            map: HashMap::new(),
            internal: is_internal,
            type_name: std::any::type_name::<T>()
                .split("::")
                .last()
                .unwrap()
                .to_string(),
            connector: if is_internal {
                None
            } else {
                Some(Box::new(Connector::with_url(env::var(DB_URL).unwrap().as_str())))
            },
        }
    }

    pub fn get(&self, k: &str) -> Option<T> {
        let key = self.make_key(k);
        match self.internal {
            true => Some(self.map.get(&key)?.clone()),
            false => match &self.connector {
                Some(connector) => serde_json::from_str(connector.get(&key)?.as_str()).ok(),
                None => None,
            },
        }
    }

    pub fn insert(&mut self, k: String, value: T) {
        let key = self.make_key(k.as_str());
        match self.internal {
            true => {
                self.map.insert(key, value);
            }
            false => {
                if let Some(connector) = &self.connector {
                    connector.insert(&key, serde_json::to_string(&value).unwrap().as_str());
                }
            }
        };
    }

    fn make_key(&self, key: &str) -> String {
        format!("{}::{}", self.type_name, key)
    }
}

pub static ID_NAME: Lazy<Mutex<HashMap<String, NameBasic>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static PRIMARY_NAME: Lazy<Mutex<HashMap<String, NameBasic>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CREW: Lazy<Mutex<Vec<TitleCrew>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static NAME_PRINCIPAL: Lazy<Mutex<HashMap<String, Vec<TitlePrincipal>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static ID_RATING: Lazy<Mutex<HashMap<String, TitleRating>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static GENRE_TITLE: Lazy<Mutex<HashMap<String, Vec<TitleBasic>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CALL_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));
