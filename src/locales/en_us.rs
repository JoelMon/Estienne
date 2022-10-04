use crate::url::Url;

pub const BOOKS: [&'static str; 66] = [
    "genesis",
    "exodus",
    "leviticus",
    "numbers",
    "deuteronomy",
    "joshua",
    "judges",
    "ruth",
    "1 samuel",
    "2 samuel",
    "1 kings",
    "2 kings",
    "1 chronicles",
    "2 chronicles",
    "ezra",
    "nehemiah",
    "esther",
    "job",
    "psalms",
    "proverbs",
    "ecclesiastes",
    "song of solomon",
    "isaiah",
    "jeremiah",
    "lamentations",
    "ezekiel",
    "daniel",
    "hosea",
    "joel",
    "amos",
    "obadiah",
    "jonah",
    "micah",
    "nahum",
    "habakkuk",
    "zephaniah",
    "haggai",
    "zechariah",
    "malachi",
    "matthew",
    "mark",
    "luke",
    "john",
    "acts",
    "romans",
    "1 corinthians",
    "2 corinthians",
    "galatians",
    "ephesians",
    "philippians",
    "colossians",
    "1 thessalonians",
    "2 thessalonians",
    "1 timothy",
    "2 timothy",
    "titus",
    "philemon",
    "hebrews",
    "james",
    "1 peter",
    "2 peter",
    "1 john",
    "2 john",
    "3 john",
    "jude",
    "revelation",
];

pub const fn BOOK_INDEX() -> [(&'static str, usize); 66] {
    let mut count = 0;
    let mut list: [(&'static str, usize); 66] = [("", 0); 66];

    while count < 67 {
        list[count] = (BOOKS[count], count + 1);
        count += 1;
    }
    list
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
