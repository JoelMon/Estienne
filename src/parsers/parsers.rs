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
    todo!
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scripture {
    name: String,
    chapter: u8,
    verse: u8,
}

    // lazy_static insures that the regex is compiled only once.
    lazy_static!{
        // For testing the regex expression: https://regex101.com/r/fS3wA0/1
        static ref RE: Regex = Regex::new(r"\((?:[1234]\s?)?[a-zA-Z]+)(\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*(?:;\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*|;))*)?)\ig").expect("Something went wrong during use of regex");

        //static ref RE: Regex = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").expect("Something went wrong");

    }

fn regex_compilation(line: &str) -> regex::Match {

    let c_regex = &RE;

    let found: regex::Match = c_regex.find(line).unwrap();

    //println!("{}", &found.start());
    //dbg!(&found.end());
    found

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_find() {
       let thing = regex_compilation("phone: 111-222-3333");
       dbg!(&thing);
    println!("THING: {}", &thing.start());

    assert_eq!((thing.start(), thing.end()), (7, 19));
    }
}
