use std::collections::HashSet;

use crate::models::mapper::Page;

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
