// #![allow(dead_code, unused_variables)]

use lazy_static::lazy_static;
use regex::Regex;

type ScriptSlice = (Start, End);
type Start = usize;
type End = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scripture {
    name: String,
    chapter: u8,
    verse: u8,
}

// lazy_static insures that the regex is compiled only once.
lazy_static! {
    static ref RE: regex::Regex = Regex::new(r"/(?:[1234]\s?)?([a-zA-Z]+)(\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\s*\d+[—–-]\d+|,\s*\d+)*(?:;\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*|;))*)?)/ig").expect("error while running regex");
}

pub fn find_scriptures(line: &str) -> Vec<ScriptSlice> {
    let mut scrip_slices: Vec<ScriptSlice> = Vec::new();

    let re: &regex::Regex = &RE;

    dbg!("About to run");

    for script in re.find_iter(&line) {
        dbg!("Running... ");
        dbg!(&script.start());

        scrip_slices.push((script.start(), script.end()));
    }

    scrip_slices
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_find_scriptures() {
        let line = r"John 3:16 and Matthew 24:14";
        let result: Vec<ScriptSlice> = find_scriptures(line);

        assert_eq!(result, vec![(0, 9), (14, 27)])
    }
}
