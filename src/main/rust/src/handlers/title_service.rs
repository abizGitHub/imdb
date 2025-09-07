use crate::handlers::db::{ ID_TITLE};
use crate::models::imdb::{TitleBasics, TitleCrew};

pub fn add(title: TitleBasics) {
    ID_TITLE.lock().unwrap().insert(title.id.clone(), title);
}

pub fn get_by_id(id: &str) -> Option<TitleBasics> {
    match ID_TITLE.lock().unwrap().get(id) {
       Some(x) => Some(x.clone()),
       None => None   
    }
}

