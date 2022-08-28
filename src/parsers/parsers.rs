#![allow(dead_code)]
use crate::locales::en_us::book;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scripture {
    name: String,
    chapter: u8,
    verse: u8,
}

fn find(line: &str) -> Option<Vec<Scripture>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_find() {
        assert!(true);
    }
}
