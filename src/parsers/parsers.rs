#![allow(dead_code)]
use crate::locales::en_us::book;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scripture {
    name: String,
    chapter: u8,
    verse: u8,
}

// lazy_static insures that the regex is compiled only once.
lazy_static! {
    // For testing the regex expression: https://regex101.com/r/fS3wA0/1
   static ref RE: Regex = Regex::new(r"/((?:[1234]\s?)?[a-zA-Z]+)(\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*(?:;\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*|;))*)?)/ig").expect("Something went wrong during use of regex");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_find() {
        let result = RE.find("Matthew 24:14").unwrap();
        dbg!(result);
        assert_eq!(result.start(), 0);
        assert_eq!(result.end(), 5);
    }
}
