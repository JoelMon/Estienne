use crate::locales::{en_us::Book, BibleRef};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: regex::Regex =
        // Regular expression matches the pattern for the name of the book or letter, chapter, and verse.
                Regex::new(r"(?<book>(?:[1234]\s?)?[a-zA-Z]+)\s*(?<chapter>\d+)(?::(?<verse>\d+(?:[—–-]\d+)?(?:,\s*\d+(?:[—–-]\d+)?)*(?:;\s*\d+(?::\d+(?:[—–-]\d+)?(?:,\s*\d+(?:[—–-]\d+)?)*))*)?)").expect("error while compiling the FIND_BOOK regex in scripture");
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bible<'a> {
    book: &'a str,
    booknum: String,
    chapter: &'a str,
    verse: &'a str,
}

#[allow(unused)]
impl<'a> Bible<'a> {
    /// Creates a Bible with a single scripture.
    pub(crate) fn single_scripture(book: &'a str, chapter: &'a str, verse: &'a str) -> Bible<'a> {
        Bible {
            book,
            chapter,
            verse,
            booknum: Default::default(),
        }
    }

    pub(crate) fn get_book(&self) -> &'a str {
        self.book
    }

    pub(crate) fn get_chapter(&self) -> &'a str {
        self.chapter
    }

    pub(crate) fn get_verse(&self) -> &'a str {
        self.verse
    }

    pub(crate) fn get_idx(&self) -> u8 {
        Book::get_index(self.book).expect("expected a valid book")
    }

    pub(crate) fn is_range(&self) -> bool {
        match self.get_verse().contains('-') {
            true => true,
            false => false,
        }
    }

    pub(crate) fn parse(scripture: &'a str) -> Bible {
        let re: &RE = &RE;

        let scripture: regex::Captures = match Book::is_valid(
            RE.captures(scripture)
                .unwrap()
                .name("book")
                .unwrap()
                .as_str(),
        ) {
            true => RE.captures(scripture).unwrap(),

            false => {
                //TODO: Improve this with Errors
                panic!();
            }
        };

        Self {
            book: scripture.name("book").unwrap().as_str(),
            //TODO Fix this part so LOCALES can work.
            booknum: Book::get_index(scripture.name("book").unwrap().as_str())
                .unwrap()
                .to_string(),
            chapter: scripture.name("chapter").unwrap().as_str(),
            verse: scripture.name("verse").unwrap().as_str(),
        }
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    // Should panic because this book should have been filtered out
    fn t_non_existing_book() {
        let input: &str = "Mary 3:16";
        let expect: Bible = Bible {
            book: "Mary",
            chapter: "3",
            verse: "16",
            booknum: "0".into(),
        };
        let got: Bible = Bible::parse(input);
        assert_eq!(got, expect);
    }

    #[test]
    fn t_find_book() {
        let input: &str = "John 3:16";
        let expect: Bible = Bible {
            book: "John",
            chapter: "3",
            verse: "16",
            booknum: "43".into(),
        };
        let result: Bible = Bible::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    fn t_find_book_space() {
        let input: &str = "1 Timothy 3:16";
        let expect: Bible = Bible {
            book: "1 Timothy",
            chapter: "3",
            verse: "16",
            booknum: "54".into(),
        };
        let result: Bible = Bible::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    fn t_find_book_ranged() {
        let input: &str = "1 Timothy 3:16-20";
        let expect: Bible = Bible {
            book: "1 Timothy",
            chapter: "3",
            verse: "16-20",
            booknum: "54".into(),
        };
        let result: Bible = Bible::parse(input);
        assert_eq!(result, expect);
    }
}
