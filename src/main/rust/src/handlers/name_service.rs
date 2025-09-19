use crate::{
    errors::MyError,
    handlers::db::{ID_NAME, PRIMARY_NAME},
    models::name_basic::NameBasic,
    utils::UnwrapPoisonIgnored,
};

pub fn add(name_basic: NameBasic) {
    ID_NAME
        .lock()
        .unwrap_ignore_poison()
        .insert(name_basic.id.clone(), name_basic.clone());

    PRIMARY_NAME
        .lock()
        .unwrap_ignore_poison()
        .insert(name_basic.primary_name.clone(), name_basic);
}

pub fn get_by_id(id: &str) -> Option<NameBasic> {
    match ID_NAME.lock().unwrap_ignore_poison().get(id) {
        Some(x) => Some(x.clone()),
        None => None,
    }
}

pub fn get_by_primary_name(name: &str) -> Result<NameBasic, MyError> {
    match PRIMARY_NAME.lock().unwrap_ignore_poison().get(name) {
        Some(x) => Ok(x.clone()),
        None => Err(MyError::ActorNameNotFound {
            primary_name: name.to_string(),
        }),
    }
}
