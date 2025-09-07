use crate::handlers::imdb_handler;
use crate::models::mapper::TSVMapper;
use crate::models::imdb::*;

pub async fn save(file_name: &str, content: &str) -> usize {
    match file_name {
        "title.basics.tsv" => {
            TSVMapper::<TitleBasics>::new(content).write_to(imdb_handler::add_title_basics)
        }
        "title.crew.tsv" => {
            TSVMapper::<TitleCrew>::new(content).write_to(imdb_handler::add_title_crew)
        }
        _ => 0
    }
}
