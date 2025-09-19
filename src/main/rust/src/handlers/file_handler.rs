use crate::errors::MyError;
use crate::handlers::imdb_handler;
use crate::models::mapper::TSVMapper;
use crate::models::name_basic::NameBasic;
use crate::models::title_basic::TitleBasic;
use crate::models::title_crew::TitleCrew;
use crate::models::title_principal::TitlePrincipal;
use crate::models::title_rating::TitleRating;

pub async fn save(file_name: &str, content: &str) -> Result<usize, MyError> {
    match file_name {
        "title.basics.tsv" => {
            Ok(TSVMapper::<TitleBasic>::new(content).write_to(imdb_handler::add_title_basics))
        }
        "title.crew.tsv" => {
            Ok(TSVMapper::<TitleCrew>::new(content).write_to(imdb_handler::add_title_crew))
        }
        "title.principals.tsv" => {
            Ok(TSVMapper::<TitlePrincipal>::new(content)
                .write_to(imdb_handler::add_title_principal))
        }
        "name.basics.tsv" => {
            Ok(TSVMapper::<NameBasic>::new(content).write_to(imdb_handler::add_name_basics))
        }
        "title.ratings.tsv" => {
            Ok(TSVMapper::<TitleRating>::new(content).write_to(imdb_handler::add_title_rating))
        }
        _ => Err(MyError::InvalidFileName {
            file_name: file_name.to_string(),
        }),
    }
}
