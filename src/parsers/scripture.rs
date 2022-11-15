use crate::locales::en_us::Book;
use crate::locales::BibleRef;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: regex::Regex =
        // Regular expression matches the pattern for the name of the book or letter, chapter, and verse. Single verse at the moment,
        // ranges are not taken into account.
        Regex::new(r"(?P<book>([1234]\s)?[a-zA-Z]+)(?:\s+)(?P<chapter>\d+)(:)(?P<verse>\d+)").expect("error while compiling the FIND_BOOK regex in scripture");
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

    // TODO: Might need to be vecs
    pub(crate) fn parse(scripture: Vec<&'a str>) -> Vec<Bible> {
        let re: &RE = &RE;

        let book_list = ["Matthew"];

        // Find Books from list and build a vec of valid scriptures
        let all_scriptures: Vec<Bible> = scripture
            .iter()
            .map(|scripture| re.captures(scripture).unwrap())
            // Filter out anything that had the pattern of a scripture but is not contained in locales' books().
            .filter(|captures| Book::is_valid(captures.name("book").unwrap().as_str())) //Improve this
            .map(|scripture| -> Bible {
                Bible {
                    book: scripture.name("book").unwrap().as_str(),
                    booknum: Book::get_index(scripture.name("book").unwrap().as_str())
                        .unwrap()
                        .to_string(),
                    chapter: scripture.name("chapter").unwrap().as_str(),
                    verse: scripture.name("verse").unwrap().as_str(),
                }
            })
            .collect();

        all_scriptures
    }
}

// ########################################################################################################################
// ###################################################### UNIT TESTS ######################################################
// ########################################################################################################################

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    // Should panic because this book should have been filtered out
    fn t_non_existing_book() {
        let input: Vec<&str> = vec!["Mary 3:16"];
        let expect: Vec<Bible> = vec![Bible {
            book: "Mary",
            chapter: "3",
            verse: "16",
            booknum: "0".into(),
        }];
        let got: Vec<Bible> = Bible::parse(input);
        assert_eq!(got, expect);
    }

    #[test]
    fn t_find_book() {
        let input: Vec<&str> = vec!["John 3:16"];
        let expect: Vec<Bible> = vec![Bible {
            book: "John",
            chapter: "3",
            verse: "16",
            booknum: "43".into(),
        }];
        let result: Vec<Bible> = Bible::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    fn t_find_book_space() {
        let input: Vec<&str> = vec!["1 Timothy 3:16"];
        let expect: Vec<Bible> = vec![Bible {
            book: "1 Timothy",
            chapter: "3",
            verse: "16",
            booknum: "54".into(),
        }];
        let result: Vec<Bible> = Bible::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    // Two scriptures, letter to Timothy is prefixed with a number
    fn t_find_book_two() {
        let input: Vec<&str> = vec!["Matthew 24:14", "Psalms 83:18"];
        let expect: Vec<Bible> = vec![
            Bible {
                book: "Matthew",
                chapter: "24",
                verse: "14",
                booknum: "40".into(),
            },
            Bible {
                book: "Psalms",
                chapter: "83",
                verse: "18",
                booknum: "19".into(),
            },
        ];
        let result: Vec<Bible> = Bible::parse(input);
        assert_eq!(result, expect);
    }
}
