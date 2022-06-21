pub mod bible_books {
    use nom::branch::alt;
    use nom::bytes::complete::tag_no_case;
    use nom::IResult;

    // All the books and letters of the Bible as they appear in the the New World Translation
    pub fn parse_book(input: &str) -> IResult<&str, &str> {
        let books = alt((
            alt((
                tag_no_case("Genesis"),
                tag_no_case("Exodus"),
                tag_no_case("Leviticus"),
                tag_no_case("Numbers"),
                tag_no_case("Deuteronomy"),
                tag_no_case("Joshua"),
                tag_no_case("Judges"),
                tag_no_case("Ruth"),
                tag_no_case("1 Samuel"),
                tag_no_case("2 Samuel"),
                tag_no_case("1 Kings"),
                tag_no_case("2 Kings"),
                tag_no_case("1 Chronicles"),
                tag_no_case("2 Chronicles"),
                tag_no_case("Ezra"),
                tag_no_case("Nehemiah"),
                tag_no_case("Esther"),
                tag_no_case("Job"),
                tag_no_case("Psalms"),
                tag_no_case("Proverbs"),
            )),
            alt((
                tag_no_case("Ecclesiastes"),
                tag_no_case("Song of Solomon"),
                tag_no_case("Isaiah"),
                tag_no_case("Jeremiah"),
                tag_no_case("Lamentations"),
                tag_no_case("Ezekiel"),
                tag_no_case("Daniel"),
                tag_no_case("Hosea"),
                tag_no_case("Joel"),
                tag_no_case("Amos"),
                tag_no_case("Obadiah"),
                tag_no_case("Jonah"),
                tag_no_case("Micah"),
                tag_no_case("Nahum"),
                tag_no_case("Habakkuk"),
                tag_no_case("Zephaniah"),
                tag_no_case("Haggai"),
                tag_no_case("Zechariah"),
                tag_no_case("Malachi"),
                tag_no_case("Matthew"),
                tag_no_case("Mark"),
            )),
            alt((
                tag_no_case("Luke"),
                tag_no_case("John"),
                tag_no_case("Acts"),
                tag_no_case("Romans"),
                tag_no_case("1 Corinthians"),
                tag_no_case("2 Corinthians"),
                tag_no_case("Galatians"),
                tag_no_case("Ephesians"),
                tag_no_case("Philippians"),
                tag_no_case("Colossians"),
                tag_no_case("1 Thessalonians"),
                tag_no_case("2 Thessalonians"),
                tag_no_case("1 Timothy"),
                tag_no_case("2 Timothy"),
                tag_no_case("Titus"),
                tag_no_case("Philemon"),
                tag_no_case("Hebrews"),
                tag_no_case("James"),
                tag_no_case("1 Peter"),
                tag_no_case("2 Peter"),
                tag_no_case("1 John"),
            )),
            alt((
                tag_no_case("2 John"),
                tag_no_case("3 John"),
                tag_no_case("Jude"),
                tag_no_case("Revelation"),
            )),
        ));

        let mut parser = books;
        parser(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::locales::en_us::bible_books::*;

    /// Test the parsing of a book name containing a single word.
    #[test]
    fn t_parse_book_single() {
        let input = "Luke is the third book in the Greek scriptures.";
        let result = parse_book(input);
        assert_eq!(
            result,
            Ok((" is the third book in the Greek scriptures.", "Luke"))
        );
    }

    /// Test the parsing of a letter name containing a name prefixed by a number.
    #[test]
    fn t_parse_book_double() {
        let input = "1 Thessalonians is a letter rather than a book.";
        let result = parse_book(input);
        assert_eq!(
            result,
            Ok((" is a letter rather than a book.", "1 Thessalonians"))
        );
    }

    /// Test the parsing of a lowercase book name containing a single word.
    #[test]
    fn t_parse_book_single_lowercase() {
        let input = "luke is the third book in the Greek scriptures.";
        let result = parse_book(input);
        assert_eq!(
            result,
            Ok((" is the third book in the Greek scriptures.", "luke"))
        );
    }

    /// Test the parsing of a letter name containing a lowercase name prefixed by a number.
    #[test]
    fn t_parse_book_double_lowercase() {
        let input = "1 thessalonians is a letter rather than a book.";
        let result = parse_book(input);
        assert_eq!(
            result,
            Ok((" is a letter rather than a book.", "1 thessalonians"))
        );
    }
}
