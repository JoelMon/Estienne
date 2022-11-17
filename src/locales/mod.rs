pub mod en_us;
pub mod es_es;

use arc_swap::{ArcSwap, ArcSwapAny};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use thiserror::Error;

use crate::locales;

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BibleError {
    #[error("the Bible book, {0}, was not found")]
    BookNotFound(String),
    #[error("the locale has to be set before use.")]
    LocaleNeedsSet(),
    #[error("the locale can only be set once and it has already been set.")]
    LocaleAlreadySet(),
}

// Initialize a safe global variable.
// Can be only set once but read many times.
static LOCALE: OnceCell<ArcSwap<LocaleLang>> = OnceCell::new();

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Locale is an enum that lists all of the languages supported by the library.
pub enum LocaleLang {
    /// American English
    en_us,
    /// Spain Spanish
    es_es,
}

#[allow(unused_must_use)]
impl LocaleLang {
    /// Sets the value of `LOCALE`. Can only set once.
    pub fn set(loc: LocaleLang) -> Result<(), BibleError> {
        let lang = Arc::new(loc);
        let arc_swap = ArcSwap::from(lang);

        match LOCALE.set(arc_swap) {
            Ok(_) => Ok(()),
            Err(_) => Err(BibleError::LocaleAlreadySet()),
        }
    }

    /// Retrieves the value of `LOCALE`, may retrieve an arbitrary number of times.
    pub fn get() -> Result<&'static ArcSwapAny<Arc<LocaleLang>>, BibleError> {
        LOCALE.get().ok_or(BibleError::LocaleNeedsSet())
    }

    /// Swaps the value of `LOCALE`. Used only for testing.
    pub(crate) fn swap(loc: LocaleLang) {
        LOCALE.get().unwrap().store(Arc::new(loc));
    }
}

#[allow(non_snake_case)]
trait Bible {
    fn get_index(book: &str) -> Result<u8, BibleError>;
    fn is_valid(book: &str) -> bool;
    fn str_to_BookMap(book: &str) -> Option<&Book>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Book {
    Genesis = 1,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
}

impl Book {
    /// Returns whether `book` is a valid _book_ in the Bible for the a language determined by `LocaleLang`.
    pub(crate) fn is_valid(book: impl Into<String>) -> bool {
        let book: String = book.into().to_lowercase();
        let book = book.as_str();
        let loc = LocaleLang::get().expect("LOCALE needs to be initialized");
        match **loc.load() {
            LocaleLang::en_us => locales::en_us::en_us::is_valid(book),
            LocaleLang::es_es => locales::es_es::es_es::is_valid(book),
        }
    }

    /// Returns the index for a valid book in the Bible. For example, Genesis would return an index of `"1"` since it is the first book of the Bible.
    pub(crate) fn get_index(book: impl Into<String>) -> Result<u8, BibleError> {
        let book: String = book.into().to_lowercase();
        let book = book.as_str();
        let loc = LocaleLang::get().expect("LOCALE needs to be initialized");
        match **loc.load() {
            LocaleLang::en_us => locales::en_us::en_us::get_index(book),
            LocaleLang::es_es => locales::es_es::es_es::get_index(book),
        }
    }
}

// ########################################################################################################################
// ###################################################### UNIT TESTS ######################################################
// ########################################################################################################################

#[cfg(test)]
mod test {
    use super::*;

    // Setup `en_us` locale for testing.
    fn setup_locale_en() {
        match LocaleLang::set(LocaleLang::en_us) {
            Ok(_) => (),
            Err(_) => LocaleLang::swap(LocaleLang::en_us),
        };
    }

    // Setup `es_es` locale for testing.
    fn setup_locale_es() {
        match LocaleLang::set(LocaleLang::es_es) {
            Ok(_) => (),
            Err(_) => LocaleLang::swap(LocaleLang::es_es),
        };
    }

    #[test]
    fn t_is_valid_en_us() {
        setup_locale_en();
        let got: bool = Book::is_valid("Matthew");
        assert!(got);
    }

    #[test]
    fn t_is_valid_en_us_false() {
        setup_locale_en();
        let got: bool = Book::is_valid("Mary");
        assert!(!got);
    }

    #[test]
    fn t_is_valid_es_es() {
        setup_locale_es();
        let got: bool = Book::is_valid("mateo");
        assert!(got);
    }

    #[test]
    fn t_is_valid_es_es_false() {
        setup_locale_es();
        let got: bool = Book::is_valid("Maria");
        assert!(!got);
    }
}
