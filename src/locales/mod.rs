pub mod en_us;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BibleError {
    #[error("the Bible book, {0}, was not found")]
    BookNotFound(String),
    #[error("failed to parse scripture reference: '{0}'")]
    ParsingError(String),
}

pub trait BibleRef {
    fn get_index(book: &str) -> Result<u8, BibleError>;
    fn is_valid(book: &str) -> bool;
}
