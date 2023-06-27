use super::{BibleError, BibleRef};
use crate::url::Url;

enum Translations {
    NWT,
    
}

pub(crate) struct Book;

const NWT: &[(&str, &str)] = &[
    ( "genesis", "gen"),
    ( "exodus", "ex"),

];

/// All translations supported by the en_us language.
pub enum Translation {
    NWT,
}

impl TryFrom<&str> for Translation {
    type Error = BibleError;

    fn try_from(tl: &str) -> Result<Self, BibleError> {
        match tl{
            "nwt" => Ok(Translation::NWT),
            _ => Err(BibleError::TranslationNotSupported(String::from(tl), String::from("us_en"))),
        }
    }
}

impl BibleRef for Book {
    fn is_valid(book_name: &str, tl: &str) -> Result<bool, BibleError> {
        let book_translation = match Translation::try_from(tl){
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let list = match book_translation {
            Translation::NWT => NWT,
        };

        Ok(list.iter().any(|&(bn, abbr)| bn == book_name || abbr == book_name))
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


// Unit tests
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_genesis_enum() {
//         let expect = 1;
//         let got = Book::get_index("genesis").unwrap();
//         assert_eq!(got, expect);
//     }

//     #[test]
//     fn test_genesis_abbr_enum() {
//         let expect = 1;
//         let got = Book::get_index("gn").unwrap();
//         assert_eq!(got, expect);
//     }

//     #[test]
//     fn test_matthew_index() {
//         let expect: u8 = 40;
//         let result: u8 = Book::get_index("Matthew").unwrap();
//         assert_eq!(result as u8, expect);
//     }

//     #[test]
//     fn test_john_abbr() {
//         let expect: u8 = 43;
//         let result: u8 = Book::get_index("joh").unwrap();
//         assert_eq!(result as u8, expect);
//     }

//     #[test]
//     #[should_panic = "error"]
//     fn test_error() {
//         let expect: u8 = 0;
//         let result: Book = "Mary".try_into().expect("error");
//         assert_eq!(result as u8, expect);
//     }
// }
