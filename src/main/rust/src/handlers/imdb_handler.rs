use std::collections::{HashMap, HashSet};

use crate::{
    handlers::{
        db::{CREW, NAME_PRINCIPAL},
        name_service, title_service,
    },
    models::{
        mapper::Page,
        name_basic::NameBasic,
        title_basic::TitleBasic,
        title_crew::TitleCrew,
        title_principal::TitlePrincipal,
        title_rating::{TitleByYear, TitleRating},
    },
    utils::Pagination,
};

pub fn add_title_basics(title: TitleBasic) {
    title_service::add(title);
}

pub fn add_title_rating(rating: TitleRating) {
    title_service::add_title_rating(rating);
}

pub fn add_title_crew(crew: TitleCrew) {
    CREW.lock().unwrap().push(crew);
}

pub fn add_title_principal(principal: TitlePrincipal) {
    let mut name_principal = NAME_PRINCIPAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match name_principal.get_mut(&principal.name_id) {
        Some(list) => {
            list.push(principal);
        }
        None => {
            name_principal.insert(principal.name_id.clone(), vec![principal]);
        }
    };
}

pub fn add_name_basics(name: NameBasic) {
    name_service::add(name);
}

pub fn titles_with_same_crew_and_alive(size: usize, page: usize) -> Page<TitleBasic> {
    let titles: HashSet<TitleBasic> = CREW
        .lock()
        .unwrap()
        .iter()
        .filter(|&c| {
            let same_crew = c.same_director_and_writer();
            if same_crew.is_empty() {
                false
            } else {
                same_crew.iter().any(|c| match name_service::get_by_id(c) {
                    Some(n) => n.death_year.is_none(),
                    None => false,
                })
            }
        })
        .filter_map(|c| title_service::get_by_id(c.title_id.as_str()))
        .collect();

    titles.paginate(page, size)
}

pub fn common_titles(actor1: String, actor2: String, size: usize, page: usize) -> Page<TitleBasic> {
    let name_principal = NAME_PRINCIPAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let principal1 = name_principal.get(
        name_service::get_by_primary_name(&actor1)
            .expect(format!("actor1[{actor1}] doesn't exist!").as_str())
            .id
            .as_str(),
    );

    let principal2 = name_principal.get(
        name_service::get_by_primary_name(&actor2)
            .expect(format!("actor2[{actor2}] doesn't exist!").as_str())
            .id
            .as_str(),
    );

    if principal1.is_none() || principal2.is_none() {
        return Page::empty();
    }

    let extract_titles = |principal: Option<&Vec<TitlePrincipal>>| -> HashSet<String> {
        principal
            .unwrap()
            .iter()
            .filter(|tp| tp.is_actor())
            .map(|tp| tp.title_id.clone())
            .collect()
    };

    let titles1: HashSet<String> = extract_titles(principal1);
    let titles2: HashSet<String> = extract_titles(principal2);

    let shared_titles: HashSet<TitleBasic> = titles1
        .intersection(&titles2)
        .filter_map(|t| title_service::get_by_id(t))
        .collect();

    shared_titles.paginate(page, size)
}

pub fn rating_by_genre(genre: String, size: usize, page: usize) -> Page<TitleByYear> {
    let titles = title_service::get_by_genre(&genre, size, page);
    let mut title_by_year: HashMap<i16, Vec<TitleBasic>> = HashMap::new();

    titles.content.iter().for_each(|t| {
        title_by_year
            .entry(t.start_year.clone())
            .or_insert_with(|| Vec::new())
            .push(t.clone());
    });

    let years: Vec<i16> = title_by_year.keys().map(|y| *y).collect();

    Page {
        content: years
            .iter()
            .map(|y| TitleByYear {
                year: *y,
                titles: title_by_year.remove(y).unwrap_or_default(),
            })
            .collect(),
        total_record: titles.total_record,
    }
}
