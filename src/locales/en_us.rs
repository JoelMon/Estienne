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

    // Naming as it appeares on the NWT.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "genesis" | "gn" => Ok(Book::Genesis),
            "exodus" | "ex" => Ok(Book::Exodus),
            "leviticus" | "le" => Ok(Book::Leviticus),
            "numbers" | "nu" => Ok(Book::Numbers),
            "deuteronomy" | "de" => Ok(Book::Deuteronomy),
            "joshua" | "jos" => Ok(Book::Joshua),
            "judges" | "jg" => Ok(Book::Judges),
            "ruth" | "ru" => Ok(Book::Ruth),
            "1 samuel" | "1sa" | "1 sa" => Ok(Book::FirstSamuel),
            "2 samuel" | "2sa" | "2 sa" => Ok(Book::SecondSamuel),
            "1 kings" | "1ki" | "1 ki" => Ok(Book::FirstKings),
            "2 kings" | "2ki" | "2 ki" => Ok(Book::SecondKings),
            "1 chronicles" | "1ch" | "1 ch" => Ok(Book::FirstChronicles),
            "2 chronicles" | "2ch" | "2 ch" => Ok(Book::SecondChronicles),
            "ezra" | "ezr" => Ok(Book::Ezra),
            "nehemiah" | "ne" => Ok(Book::Nehemiah),
            "esther" | "es" => Ok(Book::Esther),
            "job" => Ok(Book::Job),
            "psalms" | "ps" => Ok(Book::Psalms),
            "proverbs" | "pr" => Ok(Book::Proverbs),
            "ecclesiastes" | "ec" => Ok(Book::Ecclesiastes),
            "song of solomon" | "ca" => Ok(Book::SongOfSolomon),
            "isaiah" | "isa" => Ok(Book::Isaiah),
            "jeremiah" | "jer" => Ok(Book::Jeremiah),
            "lamentations" | "la" => Ok(Book::Lamentations),
            "ezekiel" | "eze" => Ok(Book::Ezekiel),
            "daniel" | "da" => Ok(Book::Daniel),
            "hosea" | "ho" => Ok(Book::Hosea),
            "joel" | "joe" => Ok(Book::Joel),
            "amos" | "am" => Ok(Book::Amos),
            "obadiah" | "ob" => Ok(Book::Obadiah),
            "jonah" | "jon" => Ok(Book::Jonah),
            "micah" | "mic" => Ok(Book::Micah),
            "nahum" | "na" => Ok(Book::Nahum),
            "habakkuk" | "hab" => Ok(Book::Habakkuk),
            "zephaniah" | "zep" => Ok(Book::Zephaniah),
            "haggai" | "hag" => Ok(Book::Haggai),
            "zechariah" | "zec" => Ok(Book::Zechariah),
            "malachi" | "mal" => Ok(Book::Malachi),
            "matthew" | "mt" => Ok(Book::Matthew),
            "mark" | "mr" => Ok(Book::Mark),
            "luke" | "lu" => Ok(Book::Luke),
            "john" | "joh" => Ok(Book::John),
            "acts" | "ac" => Ok(Book::Acts),
            "romans" | "ro" => Ok(Book::Romans),
            "1 corinthians" | "1co" | "1 co" => Ok(Book::FirstCorinthians),
            "2 corinthians" | "2co" | "2 co" => Ok(Book::SecondCorinthians),
            "galatians" | "ga" => Ok(Book::Galatians),
            "ephesians" | "eph" => Ok(Book::Ephesians),
            "philippians" | "php" => Ok(Book::Philippians),
            "colossians" | "col" => Ok(Book::Colossians),
            "1 thessalonians" | "1th" | "1 th" => Ok(Book::FirstThessalonians),
            "2 thessalonians" | "2th" | "2 th" => Ok(Book::SecondThessalonians),
            "1 timothy" | "1ti" | "1 ti" => Ok(Book::FirstTimothy),
            "2 timothy" | "2ti" | "2 ti" => Ok(Book::SecondTimothy),
            "titus" | "tit" => Ok(Book::Titus),
            "philemon" | "phm" => Ok(Book::Philemon),
            "hebrews" | "heb" => Ok(Book::Hebrews),
            "james" | "jas" => Ok(Book::James),
            "1 peter" | "1pe" | "1 pe" => Ok(Book::FirstPeter),
            "2 peter" | "2pe" | "2 pe" => Ok(Book::SecondPeter),
            "1 john" | "1jo" | "1 jo" => Ok(Book::FirstJohn),
            "2 john" | "2jo" | "2 jo" => Ok(Book::SecondJohn),
            "3 john" | "3jo" | "3 jo" => Ok(Book::ThirdJohn),
            "jude" => Ok(Book::Jude),
            "revelation" | "re" => Ok(Book::Revelation),

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
        v.is_ok()
    }
}

/// All websites supported for the en_us language.
pub enum Site {
    JwOrg,
}

/// The UrlTemplate holds the URL structure for single and ranged verse scriptures.
pub(crate) struct UrlTemplate {
    single: String,
    range: String,
}

impl Url for Site {
    fn get_template(&self) -> UrlTemplate {
        match self {
                Site::JwOrg => UrlTemplate {single: "https://www.jw.org/en/library/bible/study-bible/books/{BOOKNAME}/{CHAPTER}/#v{BOOKNUM}{CHAPTER}{VERSE}".into(), 
                                        range:  "https://www.jw.org/en/library/bible/study-bible/books/{BOOKNAME}/{CHAPTER}/#v{BOOKNUM}{CHAPTER}{VERSE}-v{BOOKNUM}{CHAPTER}{VERSE}".into()}
            }
    }
    fn get_single(&self) -> String {
        match self {
            Site::JwOrg => {
                let temp = self.get_template();
                temp.single
            }
        }
    }

    fn get_range(&self) -> String {
        match self {
            Site::JwOrg => {
                let temp = self.get_template();
                temp.range
            }
        }
    }
}

// Unit tests
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
