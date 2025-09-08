use serde::Serialize;

use crate::models::mapper::FieldSettable;


#[derive(Debug, Default, Clone, Hash, PartialEq, PartialOrd, Eq, Serialize)]
pub struct TitlePrincipal {
    pub title_id: String,
    pub name_id: String,
    pub ordering: i8,
    pub category: String,
    pub job: String,
    pub characters: String,
}

impl TitlePrincipal {
    pub fn is_actor(&self) -> bool {
        self.job.eq("actor") || self.job.eq("actress")
    }
}

impl FieldSettable for TitlePrincipal {
    fn new() -> Self {
        TitlePrincipal::default()
    }

    fn set_field(&mut self, name: &str, value: &str) {
        match name {
            "tconst" => self.title_id = value.to_string(),
            "nconst" => self.name_id = value.to_string(),
            "ordering" => self.ordering = value.parse().unwrap(),
            "category" => self.category = value.to_owned(),
            "job" => self.job = value.to_owned(),
            "characters" => self.characters = value.to_owned(),
            _ => {
                panic!("no such column![{name}]")
            }
        }
    }
}
