use std::{
    collections::HashSet,
    io::Error,
    sync::{MutexGuard, PoisonError},
};

use crate::models::mapper::Page;

pub trait UnwrapPoisonIgnored<'a, T> {
    type Result;
    type PoisonError;

    fn unwrap_ignore_poison(self) -> MutexGuard<'a, T>;
}

impl<'a, T> UnwrapPoisonIgnored<'a, T>
    for Result<MutexGuard<'a, T>, PoisonError<MutexGuard<'a, T>>>
{
    type PoisonError = PoisonError<T>;
    type Result = Result<T, Error>;
    fn unwrap_ignore_poison(self) -> MutexGuard<'a, T> {
        self.unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

pub trait Pagination<T: Clone> {
    fn paginate(self, page: usize, size: usize) -> Page<T>;
}
pub trait HasLen {
    fn len(&self) -> usize;
}

impl<T> HasLen for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<T> HasLen for HashSet<T> {
    fn len(&self) -> usize {
        HashSet::len(self)
    }
}

impl<T, C> Pagination<T> for C
where
    T: Clone,
    C: IntoIterator<Item = T> + Clone + HasLen,
{
    fn paginate(self, page: usize, size: usize) -> Page<T> {
        let start_index: usize = page * size;
        let len = self.len();
        let end_index = std::cmp::min(start_index + size, len);

        Page {
            content: self
                .into_iter()
                .skip(start_index)
                .take(end_index - start_index)
                .collect::<Vec<T>>(),
            total_record: len,
        }
    }
}
