#![allow(dead_code)]
use crate::{locales, Locale};
use nom::{
    branch::permutation,
    bytes::complete::{is_a, is_not, tag, take_till, take_until, take_while},
    character::complete::{digit1, space0},
    sequence::{preceded, tuple},
    IResult,
};
use std::error::Error;
use thiserror::Error;

// Finds any occurrence of a book name.
//
// The book and letters used for searching can be found in in the
// locales directory.
//
// # TODO:
// - Support multiple languages
pub(crate) fn parse_book(input: &str) -> IResult<&str, &str> {
    match Locale::get() {
        Locale::en_us => locales::en_us::bible_books::parse_book(input),
        Locale::es_sp => locales::es_sp::bible_books::parse_book(input),
    }
}

pub(crate) fn is_book(input: &str) -> bool {
    match parse_book(input) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}

// Parse the chapter number.
pub(crate) fn parse_chapter(input: &str) -> IResult<&str, &str> {
    let mut parser = preceded(space0, digit1);
    parser(input)
}

// Parse the verse number.
pub(crate) fn parse_verse(input: &str) -> IResult<&str, &str> {
    let mut parser = preceded(tag(":"), digit1);
    parser(input)
}

// Returns a scripture
//
//
// This parser allows for the chance that no space between the book's name and the chapter.
// ie. `John3:16` and `John 3:16` are both valid.
//
// # Notes
// A side effect of this parser is that the information whether a _space_ or _no space_ separating
// a book and its chapter is lost.
//
// Also, at the moment this parser only returns scriptures of a single verse only.
// For example, `Matthew 24:14-17` would only return as `Matthew 24:14`.
//
// # Returned Value
// The scripture is returned as tuple: (<"book/letter's name>", "<chapter>", "<verse>")
//
pub(crate) fn parse_scripture(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let mut parser = tuple((parse_book, parse_chapter, parse_verse));
    parser(input)
}

// Tests whether the &str is a valid scripture or not.
//
// A valid scripture is defined as being in the form of:
// <"book/letter"><" " | ""><n>:<n>
//
// # Todo
// is_scripture() can be improved by matching Err() to `Error {code: Tag}`.
// Any other kind of error should return an `Error` type instead of a bool `false`.
pub(crate) fn is_scripture(input: &str) -> bool {
    match parse_scripture(input) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

// Parse a line until a scripture is found.
pub(crate) fn find_script(input: &str) -> IResult<&str, &str> {
    todo!()
}

// Unit tests for parsers.rs.
#[cfg(test)]
mod test {
    use super::*;

    // Test the parsing of a _valid_ book or letter name.
    #[test]
    fn t_parse_book_valid() {
        Locale::new(Locale::en_us);
        let input = "Matthew is a book in the Bible.";
        let result = parse_book(input);

        assert_eq!(result, Ok((" is a book in the Bible.", "Matthew")));
    }

    // Test the parsing of a single chapter digit.
    #[test]
    fn t_parse_chapter_single_digit() {
        let digit = "1:2";
        let result = parse_chapter(digit);

        assert_eq!(result, Ok((":2", "1")));
    }

    // Test the parsing of a double chapter digit.
    #[test]
    fn t_parse_chapter_double_digit() {
        let digit = "21:13";
        let result = parse_chapter(digit);

        assert_eq!(result, Ok((":13", "21")));
    }

    // Test the parsing of a single verse digit and return everything after it.
    #[test]
    fn t_parse_verse_single_digit() {
        let digit = ":2";
        let result = parse_verse(digit);

        assert_eq!(result, Ok(("", "2")));
    }

    // Test the parsing of a double verse digit and return everything after it.
    #[test]
    fn t_parse_verse_double_digit() {
        let digit = ":13 is a scripture ...";
        let result = parse_verse(digit);

        assert_eq!(result, Ok((" is a scripture ...", "13")));
    }

    // Test if a scripture separated by a space between it's book/letter and chapter is captured
    #[test]
    fn t_parse_scripture_space() {
        let scripture = "1 Timothy 3:16";
        let result = parse_scripture(scripture);

        assert_eq!(result, Ok(("", ("1 Timothy", "3", "16"))));
    }

    // Test if a scripture with no space between it's book/letter and chapter is captured
    #[test]
    fn t_parse_scripture_nospace() {
        let scripture = "John3:16";
        let result = parse_scripture(scripture);

        assert_eq!(result, Ok(("", ("John", "3", "16"))));
    }

    // Test whether the string is a valid scripture or not. Result should be true.
    #[test]
    fn t_is_scripture_true() {
        let input = "Joel 3:2";
        let result = is_scripture(input);

        assert!(result);
    }

    // Test whether the string is a valid scripture or not. Result should be false.
    #[test]
    fn t_is_scripture_false() {
        Locale::new(Locale::en_us);
        let input = "Robert 3:2";
        let result = is_scripture(input);

        assert!(!result);
    }

    // Test whether the string is a valid book of the Bible or not. Result should be true.
    #[test]
    fn t_is_book_true() {
        Locale::new(Locale::en_us);
        let input = "John";
        let result = is_book(input);

        assert!(result);
    }

    // Test whether the string is a valid book of the Bible or not. Result should be false.
    #[test]
    fn t_is_book_false() {
        Locale::new(Locale::en_us);
        let input = "Zach";
        let result = is_book(input);

        assert!(!result);
    }
}
