use std::collections::HashSet;

use crate::{
    handlers::{
        db::{CREW, NAME_PRINCIPAL},
        name_service, title_service,
    },
    models::{
        mapper::Page, name_basic::NameBasic, title_basic::TitleBasic, title_crew::TitleCrew,
        title_principal::TitlePrincipal,
    },
};

pub fn add_title_basics(title: TitleBasic) {
    title_service::add(title);
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

    let start_index: usize = page * size;
    let end_index = std::cmp::min(start_index + size, titles.len());

    Page {
        content: titles
            .iter()
            .skip(start_index)
            .take(end_index - start_index)
            .map(|c| c.clone())
            .collect::<Vec<TitleBasic>>(),
        total_record: titles.len(),
    }
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

    let start_index: usize = page * size;
    let end_index = std::cmp::min(start_index + size, shared_titles.len());

    Page {
        content: shared_titles
            .iter()
            .skip(start_index)
            .take(end_index - start_index)
            .map(|c| c.clone())
            .collect::<Vec<TitleBasic>>(),
        total_record: shared_titles.len(),
    }
}
