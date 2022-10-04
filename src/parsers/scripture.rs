use crate::locales::en_us::book;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: regex::Regex =
        // Regular expression matches the pattern for the name of the book or letter, chapter, and verse. Single verse at the moment,
        // ranges are not taken into account.
        Regex::new(r"(?P<book>([1234]\s)?[a-zA-Z]+)(?:\s+)(?P<chapter>\d+)(:)(?P<verse>\d+)").expect("error while compiling the FIND_BOOK regex in scripture");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Bible<'a> {
    book: &'a str,
    chapter: &'a str,
    verse: &'a str,
}

#[allow(unused)]
impl<'a> Bible<'a> {
    pub(crate) fn parse(scripture: Vec<&'a str>) -> Vec<Bible> {
        let re: &RE = &RE;

        let book_list: [&str; 66] = book();

        //Find Books from list and build a vec of valid scriptures
        let cap: Vec<Bible> = scripture
            .iter()
            .map(|scripture| re.captures(scripture).unwrap())
            // Filter out anything that had the pattern of a scripture but is not contained in locales' books().
            .filter(|captures| {
                book_list.contains(
                    &captures
                        .name("book")
                        .unwrap()
                        .as_str()
                        .to_lowercase()
                        .as_str(),
                )
            }) //Improve this
            .map(|book| Bible {
                book: book.name("book").unwrap().as_str(),
                chapter: book.name("chapter").unwrap().as_str(),
                verse: book.name("verse").unwrap().as_str(),
            })
            .collect();

        cap
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
        }];
        let result: Vec<Bible> = Bible::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    fn t_find_book() {
        let input: Vec<&str> = vec!["John 3:16"];
        let expect: Vec<Bible> = vec![Bible {
            book: "John",
            chapter: "3",
            verse: "16",
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
        }];
        let result: Vec<Bible> = Bible::parse(input);
        assert_eq!(result, expect);
    }

    #[test]
    // Two scriptures, letter to Timothy is prefixed with a number
    fn t_find_book_two_space() {
        let input: Vec<&str> = vec!["Matthew 24:14", "Psalms 83:18"];
        let expect: Vec<Bible> = vec![
            Bible {
                book: "Matthew",
                chapter: "24",
                verse: "14",
            },
            Bible {
                book: "Psalms",
                chapter: "83",
                verse: "18",
            },
        ];
        let result: Vec<Bible> = Bible::parse(input);
        assert_eq!(result, expect);
    }
}
