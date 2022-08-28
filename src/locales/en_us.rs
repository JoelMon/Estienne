type Book = &'static str;
type Abbr = &'static str;

pub fn book(book: Book) -> Option<(Book, Abbr)> {
const books:[(Book, Abbr);2] = [("Matthew", "Mat"), ("John", "Jon")];

if books.contains(&book.0) {
    Some((Book, Abbr))
} else {
    None
}

}

