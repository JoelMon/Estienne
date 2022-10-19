use crate::locales::en_us::BOOK_INDEX;
use crate::parsers::scripture::Bible;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref BOOKNAME: regex::Regex =
        Regex::new(r"\{BOOKNAME\}").expect("error while compiling the regex in BOOKNAME");
    pub static ref CHAPTER: regex::Regex =
        Regex::new(r"\{CHAPTER\}").expect("error while compiling the regex in CHAPTER");
    pub static ref BOOKNUM: regex::Regex =
        Regex::new(r"\{BOOKNUM\}").expect("error while compiling the regex in BOOKNUM");
    pub static ref VERSE: regex::Regex =
        Regex::new(r"\{VERSE\}").expect("error while compiling the regex in VERSE");
}

pub trait Url {
    /// Returns the template to the URL.
    fn get_template(&self) -> String;

    // Constructs the proper URL from `url_template`
    fn get_url(&self, scripture: Bible) -> String {
        let url: String = crate::url::BOOKNAME
            .replace(&self.get_template(), scripture.get_book())
            .into();

        let url: String = crate::url::CHAPTER
            .replace(&url, scripture.get_chapter())
            .into();

        let url: String = crate::url::BOOKNUM
            .replace(&url, format!("{:0<2}", "40"))
            .into();

        let url: String = crate::url::CHAPTER
            .replace(&url, format!("{:0>3}", scripture.get_chapter()))
            .into();

        let url: String = crate::url::VERSE
            .replace(&url, format!("{:0>3}", scripture.get_verse()))
            .into();

        url
    }
}

// ########################################################################################################################
// ###################################################### UNIT TESTS ######################################################
// ########################################################################################################################

#[cfg(test)]
mod test {

    use super::*;
    use crate::locales::en_us::Site;
    #[test]
    fn test_url_template_jw_org() {
        let site: Site = Site::JwOrg;
        let result: String = site.get_template();
        let expected: String = "https://www.jw.org/en/library/bible/study-bible/books/{BOOKNAME}/{CHAPTER}/#v{BOOKNUM}{CHAPTER}{VERSE}".into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_url_jw_org_english() {
        let scripture: Bible = Bible::single_scripture("matthew", "24", "14");
        let site: Site = Site::JwOrg;
        let result: String = site.get_url(scripture);
        let expected: String =
            "https://www.jw.org/en/library/bible/study-bible/books/matthew/24/#v40024014".into();
        assert_eq!(result, expected);
    }
}
