pub mod bible_books {
    use nom::branch::alt;
    use nom::bytes::complete::tag_no_case;
    use nom::IResult;

    // All the books and letters of the Bible as they appear in the the New World Translation
    //TODO: Finish adding non-accented alternatives
    pub fn parse_book(input: &str) -> IResult<&str, &str> {
        let books = alt((
            alt((
                alt((tag_no_case("Génesis"), tag_no_case("Genesis"))),
                alt((tag_no_case("Éxodo"), tag_no_case("Exodo"))),
                alt((tag_no_case("Levítico"), tag_no_case("Levitico"))),
                alt((tag_no_case("Números"), tag_no_case("Numberos"))),
                tag_no_case("Deuteronomio"),
                tag_no_case("Josué"),
                tag_no_case("Jueces"),
                tag_no_case("Rut"),
                tag_no_case("1 Samuel"),
                tag_no_case("2 Samuel"),
                tag_no_case("1 Reyes"),
                tag_no_case("2 Reyes"),
                tag_no_case("1 Crónicas"),
                tag_no_case("2 Crónicas"),
                tag_no_case("Esdras"),
                tag_no_case("Nehemías"),
                tag_no_case("Ester"),
                tag_no_case("Job"),
                tag_no_case("Salmos"),
                tag_no_case("Proverbios"),
            )),
            alt((
                tag_no_case("Eclesiastés"),
                tag_no_case("El Cantar de los Cantares"),
                tag_no_case("Isaías"),
                tag_no_case("Jeremías"),
                tag_no_case("Lamentaciones"),
                tag_no_case("Ezequiel"),
                tag_no_case("Daniel"),
                tag_no_case("Oseas"),
                tag_no_case("Joel"),
                tag_no_case("Amós"),
                tag_no_case("Abdías"),
                tag_no_case("Jonás"),
                tag_no_case("Miqueas"),
                tag_no_case("Nahúm"),
                tag_no_case("Habacuc"),
                tag_no_case("Sofonías"),
                tag_no_case("Ageo"),
                tag_no_case("Zacarías"),
                tag_no_case("Malaquías"),
                tag_no_case("Mateo"),
                tag_no_case("Marcos"),
            )),
            alt((
                tag_no_case("Lucas"),
                tag_no_case("Juan"),
                tag_no_case("Hechos"),
                tag_no_case("Romanos"),
                tag_no_case("1 Corintios"),
                tag_no_case("2 Corintios"),
                tag_no_case("Gálatas"),
                tag_no_case("Efesios"),
                tag_no_case("Filipenses"),
                tag_no_case("Colosenses"),
                tag_no_case("1 Tesalonicenses"),
                tag_no_case("2 Tesalonicenses"),
                tag_no_case("1 Timoteo"),
                tag_no_case("2 Timoteo"),
                tag_no_case("Tito"),
                tag_no_case("Filemón"),
                tag_no_case("Hebreos"),
                tag_no_case("Santiago"),
                tag_no_case("1 Pedro"),
                tag_no_case("2 Pedro"),
                tag_no_case("1 Juan"),
            )),
            alt((
                tag_no_case("2 Juan"),
                tag_no_case("3 Juan"),
                tag_no_case("Judas"),
                tag_no_case("Apocalipsis"),
            )),
        ));

        let mut parser = books;
        parser(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::locales::es_sp::bible_books::*;

    /// Test the parsing of a book name containing a single word.
    #[test]
    fn t_parse_book_single() {
        let input = "Lucas es un libro de la biblia.";
        let result = parse_book(input);
        assert_eq!(result, Ok((" es un libro de la biblia.", "Lucas")));
    }

    /// Test the parsing of a letter name containing a name prefixed by a number.
    #[test]
    fn t_parse_book_double() {
        let input = "1 Tesalonicenses es una carta y no un libro de la biblia.";
        let result = parse_book(input);
        assert_eq!(
            result,
            Ok((
                " es una carta y no un libro de la biblia.",
                "1 Tesalonicenses"
            ))
        );
    }

    /// Test the parsing of a letter name containing a lowercase name prefixed by a number.
    #[test]
    fn t_parse_book_double_lowercase() {
        let input = "1 timoteo es tambien un libro de la biblia.";
        let result = parse_book(input);
        assert_eq!(
            result,
            Ok((" es tambien un libro de la biblia.", "1 timoteo"))
        );
    }

    /// Test the parsing of a book of the Bible without an accent mark.
    #[test]
    fn t_parse_book_no_tilde() {
        let input = "Levitico deve tener un acento: Levítico";
        let result = parse_book(input);
        assert_eq!(result, Ok((" deve tener un acento: Levítico", "Levitico")));
    }
}
