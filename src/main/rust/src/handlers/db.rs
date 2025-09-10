use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::models::{
    name_basic::NameBasic, title_basic::TitleBasic, title_crew::TitleCrew,
    title_principal::TitlePrincipal,
};

pub static ID_TITLE: Lazy<Mutex<HashMap<String, TitleBasic>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static ID_NAME: Lazy<Mutex<HashMap<String, NameBasic>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static PRIMARY_NAME: Lazy<Mutex<HashMap<String, NameBasic>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CREW: Lazy<Mutex<Vec<TitleCrew>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static NAME_PRINCIPAL: Lazy<Mutex<HashMap<String, Vec<TitlePrincipal>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CALL_COUNTER: Lazy<Arc<Mutex<u32>>> = Lazy::new(|| Arc::new(Mutex::new(0)));
