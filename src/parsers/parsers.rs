#![allow(dead_code)]
use crate::locales::en_us::book;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scripture {
    name: String,
    chapter: u8,
    verse: u8,
}

fn find(line: &str) -> Option<Vec<Scripture>> {

    // lazy_static insures that the regex is compiled only once.
    lazy_static!{
        // For testing the regex expression: https://regex101.com/r/fS3wA0/1
        static ref RE: Regex = Regex::new(r"(?:[1234]\s?)?[a-zA-Z]+)(\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*(?:;\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*|;))*)?").expect("Something went wrong during use of regex");
    }
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
