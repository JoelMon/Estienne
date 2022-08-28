#![allow(dead_code)]
use super::Book;

pub fn book(book: Book) -> Option<Book> {
    const BOOKS: [Book; 66] = [
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

    if BOOKS.contains(&book.to_lowercase().as_str()) {
        Some(book)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_book_titlecase() {
        let result = book("Exodus");
        assert_eq!(result, Some("Exodus"));
    }

    #[test]
    fn t_book_uppercase() {
        let result = book("JOHN");
        assert_eq!(result, Some("JOHN"));
    }

    #[test]
    fn t_book_mixedcase() {
        let result = book("1 TiMoThY");
        assert_eq!(result, Some("1 TiMoThY"));
    }

    #[test]
    fn t_book_lowercase() {
        let result = book("matthew");
        assert_eq!(result, Some("matthew"));
    }

    #[test]
    fn t_book_none() {
        let result = book("Robert");
        assert_eq!(result, None);
    }
}
