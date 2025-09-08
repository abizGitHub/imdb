use crate::{handlers::db::ID_TITLE, models::title_basic::TitleBasic};

pub fn add(title: TitleBasic) {
    ID_TITLE.lock().unwrap().insert(title.id.clone(), title);
}

pub fn get_by_id(id: &str) -> Option<TitleBasic> {
    match ID_TITLE.lock().unwrap().get(id) {
       Some(x) => Some(x.clone()),
       None => None   
    }
}

