use std::marker::PhantomData;

use serde::Serialize;

pub trait FieldSettable {
    fn set_field(&mut self, name: &str, value: &str);
    fn new() -> Self;
}

pub struct TSVMapper<'a, T: FieldSettable> {
    _0: PhantomData<T>,
    headers: Vec<String>,
    content: &'a str,
}

impl<'a, T: FieldSettable> TSVMapper<'a, T> {
    pub fn new(content: &'a str) -> Self {
        TSVMapper::<T> {
            _0: Default::default(),
            headers: Vec::new(),
            content: content,
        }
    }

    pub fn write_to<F>(&mut self, consumer: F) -> usize
    where
        F: Fn(T),
    {
        let mut lines = self.content.lines();
        lines.next().unwrap().split("\t").for_each(|h| {
            self.headers.push(h.to_string());
        });
        let mut count = 0;
        loop {
            match lines.next() {
                Some(row) => {
                    let entity = self.read_and_convert_row(row);
                    count += 1;
                    consumer(entity)
                }
                None => break count,
            }
        }
    }

    pub fn read_and_convert_row(&self, row: &str) -> T {
        let mut entity = T::new();
        let mut col_ix = 0;
        for c in row.split("\t") {
            entity.set_field(self.headers.get(col_ix).unwrap(), c.trim());
            col_ix += 1;
        }
        entity
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub total_record: usize,
}

impl<T> Page<T> {
    pub fn empty() -> Self {
        Page {
            content: Vec::new(),
            total_record: 0,
        }
    }
}
