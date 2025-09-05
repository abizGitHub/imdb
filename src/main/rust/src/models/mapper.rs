use std::marker::PhantomData;

pub trait FieldSettable {
    fn set_field(&mut self, name: &str, value: String);
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

    pub fn write_to<F>(&mut self, consumer: F)
    where
        F: Fn(T),
    {
        let mut lines = self.content.lines();
        lines.next().unwrap().split("\t").for_each(|h| {
            self.headers.push(h.to_string());
        });
        loop {
            match lines.next() {
                Some(row) => {
                    let entity = self.read_and_convert_row(row);
                    consumer(entity)
                }
                None => break,
            }
        }
    }

    pub fn read_and_convert_row(&self, row: &str) -> T {
        let mut entity = T::new();
        let mut colIx = 0;
        for c in row.split("\t") {
            entity.set_field(self.headers.get(colIx).unwrap(), c.to_string());
            colIx += 1;
        }
        entity
    }
}
