use crate::locales::{en_us::Book, BibleError, BibleRef};
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

    pub(crate) fn get_idx(&self) -> Result<u8, BibleError> {
        Book::get_index(self.book)
    }

    pub(crate) fn is_range(&self) -> bool {
        self.get_verse().contains('-')
    }

    pub(crate) fn parse(scripture: &'a str) -> Result<Bible<'a>, BibleError> {
        let caps = RE
            .captures(scripture)
            .ok_or_else(|| BibleError::ParsingError(scripture.to_string()))?;

        let book_name = caps
            .name("book")
            .ok_or_else(|| BibleError::ParsingError(scripture.to_string()))?
            .as_str();

        if !Book::is_valid(book_name) {
            return Err(BibleError::BookNotFound(book_name.to_string()));
        }

        let booknum = Book::get_index(book_name)?.to_string();
        let chapter = caps
            .name("chapter")
            .ok_or_else(|| BibleError::ParsingError(scripture.to_string()))?
            .as_str();
        let verse = caps
            .name("verse")
            .map(|m| m.as_str())
            .unwrap_or_default();

        Ok(Self {
            book: book_name,
            booknum,
            chapter,
            verse,
        })
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn t_non_existing_book() {
        let input: &str = "Mary 3:16";
        let got = Bible::parse(input);
        assert!(got.is_err());
        assert_eq!(
            got.unwrap_err(),
            BibleError::BookNotFound("Mary".to_string())
        );
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
        let result: Bible = Bible::parse(input).unwrap();
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
        let result: Bible = Bible::parse(input).unwrap();
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
        let result: Bible = Bible::parse(input).unwrap();
        assert_eq!(result, expect);
    }
}
