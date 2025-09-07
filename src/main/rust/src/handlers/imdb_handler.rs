use std::collections::HashSet;

use crate::{
    handlers::{
        db::{CREW, ID_TITLE},
        title_service,
    },
    models::imdb::{Page, TitleBasics, TitleCrew},
};

pub fn add_title_basics(title: TitleBasics) {
    title_service::add(title);
}

pub fn add_title_crew(crew: TitleCrew) {
    CREW.lock().unwrap().push(crew);
}

pub fn titles_with_same_crew_and_alive(size: usize, page: usize) -> Page<TitleBasics> {
    let titles: //Vec<TitleBasics> =     ID_TITLE.lock().unwrap().values().map(|f|{f.clone()}).collect();
    HashSet<TitleBasics> = CREW
        .lock()
        .unwrap()
        .iter()
        .filter(|c| c.same_director_and_writer())
        .map(|c| title_service::get_by_id(c.title_id.as_str()))
        .filter(Option::is_some)
        .map(|t|{t.unwrap()})
        .collect();

    let start_index: usize = page * size;
    let end_index = std::cmp::min(start_index + size, titles.len());

    Page {
        content: titles
            .iter()
            .skip(start_index)
            .take(end_index - start_index)
            .map(|c| c.clone())
            .collect::<Vec<TitleBasics>>(),
        total_record: titles.len(),
    }
}

pub fn common_titles(
    actor1: String,
    actor2: String,
    size: usize,
    page: usize,
) -> Page<TitleBasics> {
    Page::empty()
}
