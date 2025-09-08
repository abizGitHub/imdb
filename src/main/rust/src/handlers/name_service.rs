use crate::{
    handlers::db::{ID_NAME, PRIMARY_NAME},
    models::name_basic::NameBasic,
};

pub fn add(name: NameBasic) {
    ID_NAME.lock().unwrap().insert(name.id.clone(), name);
}

pub fn get_by_id(id: &str) -> Option<NameBasic> {
    match ID_NAME.lock().unwrap().get(id) {
        Some(x) => Some(x.clone()),
        None => None,
    }
}

pub fn get_by_primary_name(id: &str) -> Option<NameBasic> {
    match PRIMARY_NAME.lock().unwrap().get(id) {
        Some(x) => Some(x.clone()),
        None => None,
    }
}
