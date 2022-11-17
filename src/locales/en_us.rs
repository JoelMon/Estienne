use super::BibleError;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{Bible, Book};
use crate::url::Url;

#[allow(non_camel_case_types)]
pub(crate) struct en_us;

lazy_static! {
    static ref BOOKMAP: HashMap<&'static str, Book> = HashMap::from([
        ("genesis", Book::Genesis),
        ("exodus", Book::Exodus),
        ("leviticus", Book::Leviticus),
        ("numbers", Book::Numbers),
        ("deuteronomy", Book::Deuteronomy),
        ("joshua", Book::Joshua),
        ("judges", Book::Judges),
        ("ruth", Book::Ruth),
        ("1 samuel", Book::FirstSamuel),
        ("2 samuel", Book::SecondSamuel),
        ("1 kings", Book::FirstKings),
        ("2 kings", Book::SecondKings),
        ("1 chronicles", Book::FirstChronicles),
        ("2 chronicles", Book::SecondChronicles),
        ("ezra", Book::Ezra),
        ("nehemiah", Book::Nehemiah),
        ("esther", Book::Esther),
        ("job", Book::Job),
        ("psalms", Book::Psalms),
        ("proverbs", Book::Proverbs),
        ("ecclesiastes", Book::Ecclesiastes),
        ("songOfSolomon", Book::SongOfSolomon),
        ("isaiah", Book::Isaiah),
        ("jeremiah", Book::Jeremiah),
        ("lamentations", Book::Lamentations),
        ("ezekiel", Book::Ezekiel),
        ("daniel", Book::Daniel),
        ("hosea", Book::Hosea),
        ("joel", Book::Joel),
        ("amos", Book::Amos),
        ("obadiah", Book::Obadiah),
        ("jonah", Book::Jonah),
        ("micah", Book::Micah),
        ("nahum", Book::Nahum),
        ("habakkuk", Book::Habakkuk),
        ("zephaniah", Book::Zephaniah),
        ("haggai", Book::Haggai),
        ("zechariah", Book::Zechariah),
        ("malachi", Book::Malachi),
        ("matthew", Book::Matthew),
        ("mark", Book::Mark),
        ("luke", Book::Luke),
        ("john", Book::John),
        ("acts", Book::Acts),
        ("romans", Book::Romans),
        ("1 corinthians", Book::FirstCorinthians),
        ("2 corinthians", Book::SecondCorinthians),
        ("galatians", Book::Galatians),
        ("ephesians", Book::Ephesians),
        ("philippians", Book::Philippians),
        ("colossians", Book::Colossians),
        ("1 thessalonians", Book::FirstThessalonians),
        ("2 thessalonians", Book::SecondThessalonians),
        ("1 timothy", Book::FirstTimothy),
        ("2 timothy", Book::SecondTimothy),
        ("titus", Book::Titus),
        ("philemon", Book::Philemon),
        ("hebrews", Book::Hebrews),
        ("james", Book::James),
        ("1 peter", Book::FirstPeter),
        ("2 peter", Book::SecondPeter),
        ("1 john", Book::FirstJohn),
        ("2 john", Book::SecondJohn),
        ("3 john", Book::ThirdJohn),
        ("jude", Book::Jude),
        ("revelation", Book::Revelation),
    ]);
}

lazy_static! {
    // ALTMAP contains all alternatives to BOOKMAP, this includes abbreviations or alternative spellings such as in languages that use accented letters.
    static ref ALTMAP: HashMap<&'static str, Book> = HashMap::from([
        ("ge", Book::Genesis),
        ("ex", Book::Exodus),
        ("le", Book::Leviticus),
        ("nu", Book::Numbers),
        ("de", Book::Deuteronomy),
        ("jos", Book::Joshua),
        ("jg", Book::Judges),
        ("ru", Book::Ruth),
        ("1sa", Book::FirstSamuel),
        ("1 sa", Book::FirstSamuel),
        ("2sa", Book::SecondSamuel),
        ("2 sa", Book::SecondSamuel),
        ("1ki", Book::FirstKings),
        ("1 ki", Book::FirstKings),
        ("2ki", Book::SecondKings),
        ("2 ki", Book::SecondKings),
        ("1ch", Book::FirstChronicles),
        ("1 ch", Book::FirstChronicles),
        ("2ch", Book::SecondChronicles),
        ("2 ch", Book::SecondChronicles),
        ("ezr", Book::Ezra),
        ("ne", Book::Nehemiah),
        ("es", Book::Esther),
        ("job", Book::Job),
        ("ps", Book::Psalms),
        ("pr", Book::Proverbs),
        ("ec", Book::Ecclesiastes),
        ("ca", Book::SongOfSolomon),
        ("isa", Book::Isaiah),
        ("jer", Book::Jeremiah),
        ("la", Book::Lamentations),
        ("eze", Book::Ezekiel),
        ("da", Book::Daniel),
        ("ho", Book::Hosea),
        ("joe", Book::Joel),
        ("am", Book::Amos),
        ("ob", Book::Obadiah),
        ("jon", Book::Jonah),
        ("mic", Book::Micah),
        ("na", Book::Nahum),
        ("hab", Book::Habakkuk),
        ("zep", Book::Zephaniah),
        ("hag", Book::Haggai),
        ("zec", Book::Zechariah),
        ("mal", Book::Malachi),
        ("mat", Book::Matthew),
        ("mr", Book::Mark),
        ("lu", Book::Luke),
        ("joh", Book::John),
        ("ac", Book::Acts),
        ("ro", Book::Romans),
        ("1 co", Book::FirstCorinthians),
        ("1co", Book::FirstCorinthians),
        ("2 co", Book::SecondCorinthians),
        ("2co", Book::SecondCorinthians),
        ("ga", Book::Galatians),
        ("eph", Book::Ephesians),
        ("php", Book::Philippians),
        ("col", Book::Colossians),
        ("1 th", Book::FirstThessalonians),
        ("1th", Book::FirstThessalonians),
        ("2 th", Book::SecondThessalonians),
        ("2th", Book::SecondThessalonians),
        ("1 ti", Book::FirstTimothy),
        ("1ti", Book::FirstTimothy),
        ("2 ti", Book::SecondTimothy),
        ("2ti", Book::SecondTimothy),
        ("tit", Book::Titus),
        ("phm", Book::Philemon),
        ("heb", Book::Hebrews),
        ("jas", Book::James),
        ("1 pe", Book::FirstPeter),
        ("1pe", Book::FirstPeter),
        ("2 pe", Book::SecondPeter),
        ("2pe", Book::SecondPeter),
        ("1 jo", Book::FirstJohn),
        ("1jo", Book::FirstJohn),
        ("2 jo", Book::SecondJohn),
        ("2jo", Book::SecondJohn),
        ("3 jo", Book::ThirdJohn),
        ("3jo", Book::ThirdJohn),
        ("ju", Book::Jude),
        ("re", Book::Revelation),
    ]);
}

impl Bible for en_us {
    fn get_index(book: &str) -> Result<u8, BibleError> {
        let i: Option<&Book> = match BOOKMAP.get(&book).is_some() {
            true => BOOKMAP.get(&book),
            false => ALTMAP.get(&book),
        };

        match i {
            Some(book) => Ok(*book as u8),
            None => Err(super::BibleError::BookNotFound(book.to_string())),
        }
    }

    fn is_valid(book: &str) -> bool {
        BOOKMAP.contains_key(&book) || ALTMAP.contains_key(&book)
    }

    fn str_to_BookMap(book: &str) -> Option<&Book> {
        BOOKMAP.get(book)
    }
}

/// All websites supported for the en_us language.
pub enum Site {
    JwOrg,
}

impl Url for Site {
    // TODO: Instead of working with a template, the correct URL should just be passed back.
    fn get_template(&self) -> String {
        match self {
                Site::JwOrg => "https://www.jw.org/en/library/bible/study-bible/books/{BOOKNAME}/{CHAPTER}/#v{BOOKNUM}{CHAPTER}{VERSE}".into(),
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
    fn setup_locale(){
        match LocaleLang::set(LocaleLang::en_us){
            Ok(_) => (),
            Err(_) => LocaleLang::swap(LocaleLang::en_us),
        };
    }

    #[test]
    fn test_genesis_enum() {
        setup_locale();
        let expect = 1;
        let got = Book::get_index("genesis").unwrap();
        assert_eq!(got, expect);
    }

    #[test]
    fn test_genesis_abbr_enum() {
        setup_locale();
        let expect = 1;
        let got = Book::get_index("ge").unwrap();
        assert_eq!(got, expect);
    }

    #[test]
    fn test_matthew_index() {
        setup_locale();
        let expect: u8 = 40;
        let result: u8 = Book::get_index("Matthew").unwrap();
        assert_eq!(result as u8, expect);
    }

    #[test]
    fn test_john_abbr() {
        setup_locale();
        let expect: u8 = 43;
        let result: u8 = Book::get_index("joh").unwrap();
        assert_eq!(result as u8, expect);
    }
}
