use crate::handlers::imdb_handler;
use crate::models::mapper::TSVMapper;
use crate::models::name_basic::NameBasic;
use crate::models::title_basic::TitleBasic;
use crate::models::title_crew::TitleCrew;
use crate::models::title_principal::TitlePrincipal;

pub async fn save(file_name: &str, content: &str) -> usize {
    match file_name {
        "title.basics.tsv" => {
            TSVMapper::<TitleBasic>::new(content).write_to(imdb_handler::add_title_basics)
        }
        "title.crew.tsv" => {
            TSVMapper::<TitleCrew>::new(content).write_to(imdb_handler::add_title_crew)
        }
        "title.principals.tsv" => {
            TSVMapper::<TitlePrincipal>::new(content).write_to(imdb_handler::add_title_principal)
        }
        "name.basics.tsv" => {
            TSVMapper::<NameBasic>::new(content).write_to(imdb_handler::add_name_basics)
        }
        _ => 0
    }
}
