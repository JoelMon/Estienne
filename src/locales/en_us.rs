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
            "genesis" | "gn" => Ok(Book::Genesis),
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
            "john" | "joh" => Ok(Book::John),
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
            "revelation" | "rev" => Ok(Book::Revelation),
            _ => Err(value.to_string()),
        }
    }
}

impl From<Book> for &str {
    fn from(value: Book) -> Self {
        match value {
            Book::Genesis => "genesis",
            Book::Exodus => "exodus",
            Book::Leviticus => "leviticus",
            Book::Numbers => "numbers",
            Book::Deuteronomy => "deuteronomy",
            Book::Joshua => "joshua",
            Book::Judges => "judges",
            Book::Ruth => "ruth",
            Book::FirstSamuel => "1 samuel",
            Book::SecondSamuel => "2 samuel",
            Book::FirstKings => "1 kings",
            Book::SecondKings => "2 kings",
            Book::FirstChronicles => "1 chronicles",
            Book::SecondChronicles => "2 chronicles",
            Book::Ezra => "ezra",
            Book::Nehemiah => "nehemiah",
            Book::Esther => "esther",
            Book::Job => "job",
            Book::Psalms => "psalms",
            Book::Proverbs => "proverbs",
            Book::Ecclesiastes => "ecclesiastes",
            Book::SongOfSolomon => "song of solomon",
            Book::Isaiah => "isaiah",
            Book::Jeremiah => "jeremiah",
            Book::Lamentations => "lamentations",
            Book::Ezekiel => "ezekiel",
            Book::Daniel => "daniel",
            Book::Hosea => "hosea",
            Book::Joel => "joel",
            Book::Amos => "amos",
            Book::Obadiah => "obadiah",
            Book::Jonah => "jonah",
            Book::Micah => "micah",
            Book::Nahum => "nahum",
            Book::Habakkuk => "habakkuk",
            Book::Zephaniah => "zephaniah",
            Book::Haggai => "haggai",
            Book::Zechariah => "zechariah",
            Book::Malachi => "malachi",
            Book::Matthew => "matthew",
            Book::Mark => "mark",
            Book::Luke => "luke",
            Book::John => "john",
            Book::Acts => "acts",
            Book::Romans => "romans",
            Book::FirstCorinthians => "1 corinthians",
            Book::SecondCorinthians => "2 corinthians",
            Book::Galatians => "galatians",
            Book::Ephesians => "ephesians",
            Book::Philippians => "philippians",
            Book::Colossians => "colossians",
            Book::FirstThessalonians => "1 thessalonians",
            Book::SecondThessalonians => "2 thessalonians",
            Book::FirstTimothy => "1 timothy",
            Book::SecondTimothy => "2 timothy",
            Book::Titus => "titus",
            Book::Philemon => "philemon",
            Book::Hebrews => "hebrews",
            Book::James => "james",
            Book::FirstPeter => "1 peter",
            Book::SecondPeter => "2 peter",
            Book::FirstJohn => "1 john",
            Book::SecondJohn => "2 john",
            Book::ThirdJohn => "3 john",
            Book::Jude => "jude",
            Book::Revelation => "revelation",
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

    /// True if `&str` is a valid book of the Bible.
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
    fn test_genesis_abbr_enum() {
        let expect = 1;
        let got = Book::get_index("gn").unwrap();
        assert_eq!(got, expect);
    }

    #[test]
    fn test_matthew_index() {
        let expect: u8 = 40;
        let result: u8 = Book::get_index("Matthew").unwrap();
        assert_eq!(result as u8, expect);
    }

    #[test]
    fn test_john_abbr() {
        let expect: u8 = 43;
        let result: u8 = Book::get_index("joh").unwrap();
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
