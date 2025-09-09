use crate::{
    handlers::db::{ID_NAME, PRIMARY_NAME},
    models::name_basic::NameBasic,
};

pub fn add(name_basic: NameBasic) {
    ID_NAME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(name_basic.id.clone(), name_basic.clone());

    PRIMARY_NAME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(name_basic.primary_name.clone(), name_basic);
}

pub fn get_by_id(id: &str) -> Option<NameBasic> {
    match ID_NAME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(id)
    {
        Some(x) => Some(x.clone()),
        None => None,
    }
}

pub fn get_by_primary_name(name: &str) -> Option<NameBasic> {
    match PRIMARY_NAME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(name)
    {
        Some(x) => Some(x.clone()),
        None => None,
    }
}
