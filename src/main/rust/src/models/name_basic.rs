use serde::{Deserialize, Serialize};

use crate::models::mapper::FieldSettable;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameBasic {
    pub id: String,
    pub primary_name: String,
    pub birth_year: Option<i16>,
    pub death_year: Option<i16>,
    pub primary_profession: Vec<String>,
    pub known_for_titles: Vec<String>,
}

impl FieldSettable for NameBasic {
    fn new() -> Self {
        NameBasic::default()
    }
    fn set_field(&mut self, name: &str, value: &str) {
        match name {
            "nconst" => self.id = value.to_string(),
            "primaryName" => self.primary_name = value.to_string(),
            "birthYear" => self.birth_year = value.parse().ok(),
            "deathYear" => self.death_year = value.parse().ok(),
            "primaryProfession" => value.trim().split(',').for_each(|g| {
                self.primary_profession.push(g.trim().to_string());
            }),
            "knownForTitles" => value.trim().split(',').for_each(|g| {
                self.known_for_titles.push(g.trim().to_string());
            }),
            _ => {
                panic!("no such column![{name}]")
            }
        }
    }
}
