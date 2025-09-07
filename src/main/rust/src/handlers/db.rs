use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

use crate::models::imdb::{TitleBasics, TitleCrew};

pub static ID_TITLE: Lazy<Mutex<HashMap<String, TitleBasics>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CREW: Lazy<Mutex<Vec<TitleCrew>>> = Lazy::new(|| Mutex::new(Vec::new()));
