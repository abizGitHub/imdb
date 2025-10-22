use std::cmp::Ordering;

use crate::{
    errors::MyError,
    handlers::db::{GENRE_TITLE, ID_RATING, ID_TITLE},
    models::{mapper::Page, title_basic::TitleBasic, title_rating::TitleRating},
    utils::{Pagination, UnwrapPoisonIgnored},
};

pub fn add(title: TitleBasic) {
    ID_TITLE
        .lock()
        .unwrap_ignore_poison()
        .insert(title.id.clone(), title.clone());
    let mut gt = GENRE_TITLE.lock().unwrap_ignore_poison();

    title.genres.iter().for_each(|g| {
        gt.entry(g.clone())
            .or_insert(Vec::new())
            .push(title.clone());
    });
}

pub fn get_by_id(id: &str) -> Option<TitleBasic> {
    ID_TITLE.lock().unwrap_ignore_poison().get(id)
}

pub fn add_title_rating(rating: TitleRating) {
    ID_RATING
        .lock()
        .unwrap_ignore_poison()
        .insert(rating.title_id.clone(), rating);
}

pub fn get_by_genre(genre: &str, size: usize, page: usize) -> Result<Page<TitleBasic>, MyError> {
    let mut titles = match GENRE_TITLE.lock().unwrap_ignore_poison().get(genre) {
        Some(x) => x.clone(),
        None => {
            return Err(MyError::GenreNotFound {
                genre: genre.to_string(),
            })
        }
    };
    let rating_comp = |a: &TitleRating, b: &TitleRating| {
        if a.average_rating > b.average_rating {
            Ordering::Greater
        } else if a.average_rating < b.average_rating {
            Ordering::Less
        } else {
            Ordering::Less
        }
    };
    let id_rating = ID_RATING.lock().unwrap_ignore_poison();
    titles.sort_by(|t1, t2| {
        let r1 = id_rating.get(&t1.id);
        let r2 = id_rating.get(&t2.id);
        if r1.is_none() {
            Ordering::Less
        } else if r2.is_none() {
            Ordering::Greater
        } else {
            rating_comp(r1.unwrap(), r2.unwrap())
        }
    });

    Ok(titles.paginate(page, size))
}
