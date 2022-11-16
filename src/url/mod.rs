use crate::{parsers::scripture::Bible, locales::en_us::Book};
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

    /// Constructs the proper URL from `url_template`
    fn get_url(&self, scripture: &Bible) -> String {

        let book_name:Book= scripture.get_book().try_into().unwrap();
        let book_name:&str= book_name.into();

        let url: String = crate::url::BOOKNAME
            .replace(&self.get_template(), book_name)
            .into();

        let url: String = crate::url::CHAPTER
            .replace(&url, scripture.get_chapter())
            .into();

        let url: String = crate::url::BOOKNUM
            .replace(&url, format!("{:0<2}", scripture.get_i()))
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
    use pretty_assertions::assert_eq;
    #[test]
    fn test_url_template_jw_org() {
        let site: Site = Site::JwOrg;
        let result: String = site.get_template();
        let expected: String = "https://www.jw.org/en/library/bible/study-bible/books/{BOOKNAME}/{CHAPTER}/#v{BOOKNUM}{CHAPTER}{VERSE}".into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_url_jw_org_matthew() {
        let scripture: Bible = Bible::single_scripture("matthew", "24", "14");
        let site: Site = Site::JwOrg;
        let result: String = site.get_url(&scripture);
        let expected: String =
            "https://www.jw.org/en/library/bible/study-bible/books/matthew/24/#v40024014".into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_url_jw_org_john() {
        let scripture: Bible = Bible::single_scripture("john", "3", "16");
        let site: Site = Site::JwOrg;
        let got: String = site.get_url(&scripture);
        let expect: String =
            "https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016".into();
        assert_eq!(got, expect);
    }

    #[test]
    fn test_get_url_jw_org_john_abbr() {
        let scripture: Bible = Bible::single_scripture("joh", "3", "16");
        let site: Site = Site::JwOrg;
        let got: String = site.get_url(&scripture);
        let expect: String =
            "https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016".into();
        assert_eq!(got, expect);
    }
}
