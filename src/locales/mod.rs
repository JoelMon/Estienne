pub mod en_us;
pub mod es_sp;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BibleError {
    #[error("the Bible book, {0}, was not found")]
    BookNotFound(String),
}

pub trait BibleRef {
    fn get_index(book: &str) -> Result<u8, BibleError>;
    fn is_valid(book: &str) -> bool;
}
