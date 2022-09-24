use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: regex::Regex =
        Regex::new(r"([1234]\s)?[a-zA-Z]+").expect("error while compiling the regex in scripture");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Bible {
    num_letter: Option<String>,
    book: &'static str,
    chapter: usize,
    verse: usize,
}

#[allow(unused)]
impl Bible {
    pub(crate) fn parse(scripture: Vec<&'static str>) -> Vec<Bible> {
        let re = &RE;

        //only for quick test
        let book_list: Vec<&str> = vec!["John", "1 Timothy"];

        //Find Books from list and build a vec of valid scriptures
        let cap: Vec<&'static str> = scripture
            .iter()
            .map(|x| re.captures(x).unwrap())
            .filter(|x| book_list.contains(&x.get(0).unwrap().as_str()))
            .map(|x| x.get(0).unwrap().as_str())
            .collect();

        let vec_bible: Vec<Bible> = cap.iter().fold(Vec::new(), |mut acc, book| {
            acc.push(Bible {
                num_letter: None,
                book,
                chapter: 0,
                verse: 0,
            });
            acc
        });

        vec_bible
    }
}

// ########################################################################################################################
// ###################################################### UNIT TESTS ######################################################
// ########################################################################################################################

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_find_book() {
        let input = vec!["John 3:16"];
        let expected = vec![Bible {
            num_letter: None,
            book: "John",
            chapter: 0,
            verse: 0,
        }];
        let result = Bible::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn t_find_book_space() {
        let input = vec!["1 Timothy 3:16"];
        let expected = vec![Bible {
            num_letter: None,
            book: "1 Timothy",
            chapter: 0,
            verse: 0,
        }];
        let result = Bible::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    // Two scriptures, letter to Timothy is prefixed with a number
    fn t_find_book_two_space() {
        let input = vec!["John 3:16", "1 Timothy 3:16"];
        let expected = vec![
            Bible {
                num_letter: None,
                book: "John",
                chapter: 0,
                verse: 0,
            },
            Bible {
                num_letter: None,
                book: "1 Timothy",
                chapter: 0,
                verse: 0,
            },
        ];
        let result = Bible::parse(input);
        assert_eq!(result, expected);
    }
}
