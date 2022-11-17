use super::{
    locales::{Bible, Book},
    BibleError,
};
use crate::url::Url;
use lazy_static::lazy_static;
use std::collections::HashMap;
#[allow(non_camel_case_types)]
pub(crate) struct es_es;

lazy_static! {
    static ref BOOKMAP: HashMap<&'static str, Book> = HashMap::from([
        ("genesis", Book::Genesis),
        ("exodo", Book::Exodus),
        ("levitico", Book::Leviticus),
        ("numeros", Book::Numbers),
        ("deuteronomio", Book::Deuteronomy),
        ("josue", Book::Joshua),
        ("jueces", Book::Judges),
        ("rut", Book::Ruth),
        ("1 samuel", Book::FirstSamuel),
        ("2 samuel", Book::SecondSamuel),
        ("1 reyes", Book::FirstKings),
        ("2 reyes", Book::SecondKings),
        ("1 cronicas", Book::FirstChronicles),
        ("2 cronicas", Book::SecondChronicles),
        ("esdras", Book::Ezra),
        ("nehemias", Book::Nehemiah),
        ("ester", Book::Esther),
        ("job", Book::Job),
        ("salmos", Book::Psalms),
        ("proverbios", Book::Proverbs),
        ("eclesiastes", Book::Ecclesiastes),
        ("el cantar de los cantares", Book::SongOfSolomon),
        ("isaias", Book::Isaiah),
        ("jeremias", Book::Jeremiah),
        ("lamentaciones", Book::Lamentations),
        ("exequiel", Book::Ezekiel),
        ("daniel", Book::Daniel),
        ("oseas", Book::Hosea),
        ("joel", Book::Joel),
        ("amos", Book::Amos),
        ("abdias", Book::Obadiah),
        ("jonas", Book::Jonah),
        ("miqueas", Book::Micah),
        ("nahum", Book::Nahum),
        ("habacuc", Book::Habakkuk),
        ("sofonias", Book::Zephaniah),
        ("ageo", Book::Haggai),
        ("zacarias", Book::Zechariah),
        ("malaquias", Book::Malachi),
        ("mateo", Book::Matthew),
        ("marcos", Book::Mark),
        ("lucas", Book::Luke),
        ("juan", Book::John),
        ("hechos", Book::Acts),
        ("romanos", Book::Romans),
        ("1 corintios", Book::FirstCorinthians),
        ("2 corintios", Book::SecondCorinthians),
        ("galatas", Book::Galatians),
        ("efesios", Book::Ephesians),
        ("filipenses", Book::Philippians),
        ("colosenses", Book::Colossians),
        ("1 tesalonicenses", Book::FirstThessalonians),
        ("2 tesalonicenses", Book::SecondThessalonians),
        ("1 timoteo", Book::FirstTimothy),
        ("2 timoteo", Book::SecondTimothy),
        ("tito", Book::Titus),
        ("filemon", Book::Philemon),
        ("hebreos", Book::Hebrews),
        ("santiago", Book::James),
        ("1 pedro", Book::FirstPeter),
        ("2 pedro", Book::SecondPeter),
        ("1 juan", Book::FirstJohn),
        ("2 juan", Book::SecondJohn),
        ("3 juan", Book::ThirdJohn),
        ("judas", Book::Jude),
        ("apocalipsis", Book::Revelation),
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

impl Bible for es_es {
    fn get_index(book: &str) -> Result<u8, BibleError> {
        let i: Option<&Book> = match BOOKMAP.get(&book).is_some() {
            true => BOOKMAP.get(&book),
            false => ALTMAP.get(&book),
        };

        match i {
            Some(book) => Ok(*book as u8),
            None => Err(BibleError::BookNotFound(book.to_string())),
        }
    }

    fn is_valid(book: &str) -> bool {
        BOOKMAP.contains_key(&book) || ALTMAP.contains_key(&book)
    }

    #[allow(non_snake_case)]
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
                Site::JwOrg => "https://www.jw.org/es/library/bible/study-bible/books/{BOOKNAME}/{CHAPTER}/#v{BOOKNUM}{CHAPTER}{VERSE}".into(),
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

    // Set up LocaleLang
    fn setup_locale() {
        match LocaleLang::set(LocaleLang::es_es) {
            Ok(_) => (),
            Err(_) => LocaleLang::swap(LocaleLang::es_es),
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
        let result: u8 = Book::get_index("mateo").unwrap();
        assert_eq!(result as u8, expect);
    }

     #[test]
    fn test_revelations_index() {
        setup_locale();
        let expect: u8 = 66;
        let result: u8 = Book::get_index("apocalipsis").unwrap();
        assert_eq!(result as u8, expect);
    } 
}
