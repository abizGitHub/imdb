use serde::{Deserialize, Serialize};

use crate::models::{mapper::FieldSettable, title_basic::TitleBasic};

#[derive(Debug, Default, Clone)]
pub struct TitleRating {
    pub title_id: String,
    pub average_rating: f32,
    pub num_votes: i16,
}

impl FieldSettable for TitleRating {
    fn new() -> Self {
        TitleRating::default()
    }
    fn set_field(&mut self, name: &str, value: &str) {
        match name {
            "tconst" => self.title_id = value.to_string(),
            "averageRating" => self.average_rating = value.parse().unwrap(),
            "numVotes" => self.num_votes = value.parse().unwrap(),
            _ => {
                panic!("no such column![{name}]")
            }
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TitleByYear {
    pub year: i16,
    pub titles: Vec<TitleBasic>,
}
