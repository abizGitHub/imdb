use serde::Serialize;

use crate::models::mapper::FieldSettable;

#[derive(Debug, Default, Clone, Hash, PartialEq, PartialOrd, Eq, Serialize)]
pub struct TitleBasics {
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

impl FieldSettable for TitleBasics {
    fn new() -> Self {
        TitleBasics::default()
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
            "genres" => value.trim().split("\\,").for_each(|g| {
                self.genres.push(g.to_string());
            }),
            _ => {
                panic!("no such column![{name}]")
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TitleCrew {
    pub title_id: String,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
}

impl FieldSettable for TitleCrew {
    fn new() -> Self {
        TitleCrew::default()
    }
    fn set_field(&mut self, name: &str, value: &str) {
        match name {
            "tconst" => self.title_id = value.to_string(),
            "directors" => value.trim().split("\\,").for_each(|g| {
                self.directors.push(g.trim().to_string());
            }),
            "writers" => value.trim().split("\\,").for_each(|g| {
                self.writers.push(g.trim().to_string());
            }),
            _ => {
                panic!("no such column![{name}]")
            }
        }
    }
}

impl TitleCrew {
    pub fn same_director_and_writer(&self) -> bool {
        self.directors.iter().any(|d| self.writers.contains(d))
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total_record: usize,
}

impl<T> Page<T> {
    pub fn empty() -> Self {
        Page {
            content: Vec::new(),
            total_record: 0,
        }
    }
}
