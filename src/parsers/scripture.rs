use crate::locales::Book;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: regex::Regex =
        // Regular expression matches the pattern for the name of the book or letter, chapter, and verse. Single verse at the moment,
        // ranges are not taken into account.
        Regex::new(r"(?P<book>([1234]\s)?[a-zA-Z]+)(?:\s+)(?P<chapter>\d+)(:)(?P<verse>\d+)").expect("error while compiling the FIND_BOOK regex in scripture");
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A scripture struct.
pub struct Scripture<'a> {
    book: &'a str,
    book_num: String,
    chapter: &'a str,
    verse: &'a str,
}

#[allow(unused)]
impl<'a> Scripture<'a> {
    /// Creates a Bible with a single scripture.
    pub(crate) fn single_scripture(
        book: &'a str,
        chapter: &'a str,
        verse: &'a str,
    ) -> Scripture<'a> {
        Scripture {
            book,
            chapter,
            verse,
            book_num: Default::default(),
        }
    }

    /// Returns the book stored within the `Scripture` struct.
    pub(crate) fn get_book(&self) -> &'a str {
        self.book
    }

    /// Checks to see if the book name stored within the `Scripture` struct is valid.
    pub(crate) fn valid_book(&self) -> bool {
        Book::is_valid(self.book)
    }

    /// Returns the chapter number stored within the `Scripture` struct.
    pub(crate) fn get_chapter(&self) -> &'a str {
        self.chapter
    }

    /// Returns the verse number stored within the `Scripture` struct.
    pub(crate) fn get_verse(&self) -> &'a str {
        self.verse
    }

    /// Returns the index number of the book stored within the `Scripture` struct.
    pub(crate) fn get_index(&self) -> u8 {
        Book::get_index(self.book).expect("expected a valid book")
    }

    /// Takes a potential scripture that matched the regular expression and return a `Scripture` type if it's valid otherwise returns `None`.
    pub(crate) fn parse(found_script: &'a str) -> Option<Scripture> {
        let re: &RE = &RE;

        // Determine if scripture pattern found has a valid Bible name listed in `locales/`.
        // `Book::is_valid()` takes care to determine which locales language needs to be checked.
        let scripture: Option<regex::Captures> = match Book::is_valid(
            re.captures(found_script)
                .unwrap()
                .name("book")
                .unwrap()
                .as_str(),
        ) {
            true => Some(re.captures(found_script).unwrap()),
            false => None,
        };

        // If a valid Bible name is stored in `scripture` then create a `Scripture` struct and return it to the caller.
        match scripture.is_some() {
            true => {
                let scripture: regex::Captures = scripture.unwrap();
                Some(Self {
                    book: scripture.name("book").unwrap().as_str(),
                    //TODO Fix this part so LOCALES can work.
                    book_num: Book::get_index(scripture.name("book").unwrap().as_str())
                        .unwrap()
                        .to_string(),
                    chapter: scripture.name("chapter").unwrap().as_str(),
                    verse: scripture.name("verse").unwrap().as_str(),
                })
            }
            false => None,
        }
    }
}

// ########################################################################################################################
// ###################################################### UNIT TESTS ######################################################
// ########################################################################################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::locales::LocaleLang;

    // Setup locale for testing.
    fn setup_locale_en_us() {
        match LocaleLang::set(LocaleLang::en_us) {
            Ok(_) => (),
            Err(_) => LocaleLang::swap(LocaleLang::en_us),
        };
    }

    #[test]
    #[should_panic]
    // Should panic because this book should have been filtered out
    fn t_non_existing_book() {
        setup_locale_en_us();
        let input: &str = "Mary 3:16";
        let expect: Scripture = Scripture {
            book: "Mary",
            chapter: "3",
            verse: "16",
            book_num: "0".into(),
        };
        let got: Scripture = Scripture::parse(input).unwrap();
        assert_eq!(got, expect);
    }

    #[test]
    fn t_find_book() {
        setup_locale_en_us();
        let input: &str = "John 3:16";
        let expect: Scripture = Scripture {
            book: "John",
            chapter: "3",
            verse: "16",
            book_num: "43".into(),
        };
        let result: Scripture = Scripture::parse(input).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    // Test whether a space in the book name affects capture: ie "1 Timothy"
    fn t_find_book_with_space() {
        setup_locale_en_us();
        let input: &str = "1 Timothy 3:16";
        let expect: Scripture = Scripture {
            book: "1 Timothy",
            chapter: "3",
            verse: "16",
            book_num: "54".into(),
        };
        let result: Scripture = Scripture::parse(input).unwrap();
        assert_eq!(result, expect);
    }
}
