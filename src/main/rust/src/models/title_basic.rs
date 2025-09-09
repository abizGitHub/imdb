use serde::Serialize;

use crate::models::mapper::FieldSettable;


#[derive(Debug, Default, Clone, Hash, PartialEq, PartialOrd, Eq, Serialize)]
pub struct TitleBasic {
    pub id: String,
    pub title_type: String,
    pub primary_title: String,
    pub original_title: String,
    pub is_adult: bool,
    pub start_year: i16,
    pub end_year: i16,
    pub runtime_minutes: i16,
    pub genres: Vec<String>,
}

impl FieldSettable for TitleBasic {
    fn new() -> Self {
        TitleBasic::default()
    }
    fn set_field(&mut self, name: &str, value: &str) {
        let str_value = value.to_string();
        match name {
            "tconst" => self.id = str_value,
            "titleType" => self.title_type = str_value,
            "primaryTitle" => self.primary_title = str_value,
            "originalTitle" => self.original_title = str_value,
            "isAdult" => self.is_adult = value == "1",
            "startYear" => self.start_year = value.parse().unwrap(),
            "endYear" => match value.parse() {
                Ok(ey) => self.end_year = ey,
                Err(_) => {}
            },
            "runtimeMinutes" => self.runtime_minutes = value.parse().unwrap(),
            "genres" => value.trim().split(',').for_each(|g| {
                self.genres.push(g.to_string());
            }),
            _ => {
                panic!("no such column![{name}]")
            }
        }
    }
}
