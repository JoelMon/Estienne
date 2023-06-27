pub mod en_us;
pub mod es_es;
use thiserror::Error;

use crate::Locale;

#[derive(Debug, Error)]
pub enum BibleError {
    #[error("the Bible book, {0}, was not found")]
    BookNotFound(String),
    #[error("the translation, {0}, is not supported by the {1} language")]
    TranslationNotSupported(String, String),
}

pub trait BibleRef {
    // fn get_index(book: &str) -> Result<u8, BibleError>;
    fn is_valid(book_name: &str, tl: &str) -> Result<bool, BibleError>;
}


pub fn book() -> impl BibleRef {
   let lang = Locale::get();

    match lang{
        Locale::en_us => en_us::Book,
        Locale::es_es => todo!(),
    }
}


// pub enum Book {
//     Genesis = 1,
//     Exodus,
//     Leviticus,
//     Numbers,
//     Deuteronomy,
//     Joshua,
//     Judges,
//     Ruth,
//     FirstSamuel,
//     SecondSamuel,
//     FirstKings,
//     SecondKings,
//     FirstChronicles,
//     SecondChronicles,
//     Ezra,
//     Nehemiah,
//     Esther,
//     Job,
//     Psalms,
//     Proverbs,
//     Ecclesiastes,
//     SongOfSolomon,
//     Isaiah,
//     Jeremiah,
//     Lamentations,
//     Ezekiel,
//     Daniel,
//     Hosea,
//     Joel,
//     Amos,
//     Obadiah,
//     Jonah,
//     Micah,
//     Nahum,
//     Habakkuk,
//     Zephaniah,
//     Haggai,
//     Zechariah,
//     Malachi,
//     Matthew,
//     Mark,
//     Luke,
//     John,
//     Acts,
//     Romans,
//     FirstCorinthians,
//     SecondCorinthians,
//     Galatians,
//     Ephesians,
//     Philippians,
//     Colossians,
//     FirstThessalonians,
//     SecondThessalonians,
//     FirstTimothy,
//     SecondTimothy,
//     Titus,
//     Philemon,
//     Hebrews,
//     James,
//     FirstPeter,
//     SecondPeter,
//     FirstJohn,
//     SecondJohn,
//     ThirdJohn,
//     Jude,
//     Revelation,
// }