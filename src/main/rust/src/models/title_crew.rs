use crate::models::mapper::FieldSettable;


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
