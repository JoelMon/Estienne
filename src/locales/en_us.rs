use super::{BibleError, BibleRef};
use crate::url::Url;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl TryFrom<&str> for Book {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "genesis" => Ok(Book::Genesis),
            "exodus" => Ok(Book::Exodus),
            "leviticus" => Ok(Book::Leviticus),
            "numbers" => Ok(Book::Numbers),
            "deuteronomy" => Ok(Book::Deuteronomy),
            "joshua" => Ok(Book::Joshua),
            "judges" => Ok(Book::Judges),
            "ruth" => Ok(Book::Ruth),
            "1 samuel" => Ok(Book::FirstSamuel),
            "2 samuel" => Ok(Book::SecondSamuel),
            "1 kings" => Ok(Book::FirstKings),
            "2 kings" => Ok(Book::SecondKings),
            "1 chronicles" => Ok(Book::FirstChronicles),
            "2 chronicles" => Ok(Book::SecondChronicles),
            "ezra" => Ok(Book::Ezra),
            "nehemiah" => Ok(Book::Nehemiah),
            "esther" => Ok(Book::Esther),
            "job" => Ok(Book::Job),
            "psalms" => Ok(Book::Psalms),
            "proverbs" => Ok(Book::Proverbs),
            "ecclesiastes" => Ok(Book::Ecclesiastes),
            "song of solomon" => Ok(Book::SongOfSolomon),
            "isaiah" => Ok(Book::Isaiah),
            "jeremiah" => Ok(Book::Jeremiah),
            "lamentations" => Ok(Book::Lamentations),
            "ezekiel" => Ok(Book::Ezekiel),
            "daniel" => Ok(Book::Daniel),
            "hosea" => Ok(Book::Hosea),
            "joel" => Ok(Book::Joel),
            "amos" => Ok(Book::Amos),
            "obadiah" => Ok(Book::Obadiah),
            "jonah" => Ok(Book::Jonah),
            "micah" => Ok(Book::Micah),
            "nahum" => Ok(Book::Nahum),
            "habakkuk" => Ok(Book::Habakkuk),
            "zephaniah" => Ok(Book::Zephaniah),
            "haggai" => Ok(Book::Haggai),
            "zechariah" => Ok(Book::Zechariah),
            "malachi" => Ok(Book::Malachi),
            "matthew" => Ok(Book::Matthew),
            "mark" => Ok(Book::Mark),
            "luke" => Ok(Book::Luke),
            "john" => Ok(Book::John),
            "acts" => Ok(Book::Acts),
            "romans" => Ok(Book::Romans),
            "1 corinthians" => Ok(Book::FirstCorinthians),
            "2 corinthians" => Ok(Book::SecondCorinthians),
            "galatians" => Ok(Book::Galatians),
            "ephesians" => Ok(Book::Ephesians),
            "philippians" => Ok(Book::Philippians),
            "colossians" => Ok(Book::Colossians),
            "1 thessalonians" => Ok(Book::FirstThessalonians),
            "2 thessalonians" => Ok(Book::SecondThessalonians),
            "1 timothy" => Ok(Book::FirstTimothy),
            "2 timothy" => Ok(Book::SecondTimothy),
            "titus" => Ok(Book::Titus),
            "philemon" => Ok(Book::Philemon),
            "hebrews" => Ok(Book::Hebrews),
            "james" => Ok(Book::James),
            "1 peter" => Ok(Book::FirstPeter),
            "2 peter" => Ok(Book::SecondPeter),
            "1 john" => Ok(Book::FirstJohn),
            "2 john" => Ok(Book::SecondJohn),
            "3 john" => Ok(Book::ThirdJohn),
            "jude" => Ok(Book::Jude),
            "revelation" => Ok(Book::Revelation),
            _ => Err(value.to_string()),
        }
    }
}

impl BibleRef for Book {
    fn get_index(book: &str) -> Result<u8, BibleError> {
        let i: Result<Book, String> = book.try_into();

        match i {
            Ok(book) => Ok(book as u8),
            Err(e) => Err(BibleError::BookNotFound(e)),
        }
    }

    fn is_valid(book: &str) -> bool {
        let v: Result<Book, String> = book.try_into();

        match v {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

/// All websites supported for the en_us language.
pub enum Site {
    JwOrg,
}

impl Url for Site {
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

    #[test]
    fn test_genesis_enum() {
        let expect = 1;
        let got = Book::get_index("genesis").unwrap();
        assert_eq!(got, expect);
    }

    #[test]
    fn test_matthew_index() {
        let expect: u8 = 40;
        let result: u8 = Book::get_index("Matthew").unwrap();
        assert_eq!(result as u8, expect);
    }

    #[test]
    #[should_panic = "error"]
    fn test_error() {
        let expect: u8 = 0;
        let result: Book = "Mary".try_into().expect("error");
        assert_eq!(result as u8, expect);
    }
}
