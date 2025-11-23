use std::borrow::{Borrow, Cow};
use crate::parsers::scripture::RE;

use crate::{
    locales::{nwt_en::Site, BibleError},
    url::Url,
};

use super::scripture::Bible;

type ScriptSlice = (Start, End);
type Start = usize;
type End = usize;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Elements<'a> {
    prefix: Option<&'a str>,
    prefix_len: Option<usize>,
    postfix: Option<&'a str>,
    postfix_len: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Script<'a> {
    elements: Elements<'a>,
    slice: Vec<ScriptSlice>,
    text: String,
}

#[allow(unused_variables)]
impl<'a> Script<'a> {
    /// Find all _potential_ scriptures in a line and return the beginning index and length.
    pub(crate) fn new<S>(text: S) -> Self
    where
        S: Into<String> + Clone,
    {
        let re: &regex::Regex = &RE;

        // TODO: Fix this section of code. Too much cloning, find more concise exp.
        //       The use of Cow is due to regex requiring a &str but it is possible that
        //       `text` is `String` or `&str`, eliminating the use of the `as_str()` method.
        let text_copy: S = text.clone();
        let text1: Cow<str> = Cow::from(text_copy.into());
        let matches = re.find_iter(Cow::borrow(&text1));

        let mut scrip_slices: Vec<ScriptSlice> = Vec::new();
        for script in matches {
            scrip_slices.push((script.start(), script.end()));
        }

        Self {
            text: text.into(),
            slice: scrip_slices,
            elements: Elements {
                ..Default::default()
            },
        }
    }

    /// The prefix to be added before the scripture.
    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.elements.prefix = Some(prefix);
        self
    }

    /// The postfix to be added after the scripture.
    pub fn postfix(mut self, postfix: &'a str) -> Self {
        self.elements.postfix = Some(postfix);
        self
    }

    #[allow(dead_code)]
    fn is_prefix(&self) -> bool {
        self.elements.prefix.is_some()
    }

    #[allow(dead_code)]
    fn is_postfix(&self) -> bool {
        self.elements.postfix.is_some()
    }

    /// The surround method adds a prefix and postfix when the corresponding methods are used.
    /// `surround()` does not verify if a captured _scripture_ is valid.
    pub(crate) fn surround(mut self) -> Self {
        // .rev method is used to avoid dealing with the changing size of the string.
        for item in self.slice.iter().rev() {
            self.text.insert_str(
                item.0 + (item.1 - item.0),
                self.elements
                    .postfix
                    .map_or("", |postfix_value| postfix_value),
            );

            self.text.insert_str(
                item.0,
                self.elements.prefix.map_or("", |prefix_value| prefix_value),
            );
        }

        self
    }

    /// Returns the original string with URL markup for all scriptures.
    pub(crate) fn url(mut self, site: &Site) -> Result<Self, BibleError> {
        // .rev() method is used to avoid dealing with the changing size of the string as new characters are added.
        for (start, end) in self.slice.iter().rev() {
            let verse_slice: String = self.get_from_slice(&(*start, *end));
            let bible: Bible = Bible::parse(verse_slice.as_str())?;
            let url: String = site.get_url(&bible)?;

            self.text
                .insert_str(*start + (*end - *start), format!("]({})", url).as_str());

            self.text.insert(*start, '[');
        }

        Ok(self)
    }

    /// Returns the text field of the Script struct.
    pub(crate) fn get_text(self) -> String {
        self.text
    }

    // Returns a `String` to avoid borrowing headaches
    pub(crate) fn get_from_slice(&self, slice: &(usize, usize)) -> String {
        let clone_obj: Script<'a> = self.clone();
        let text: String = clone_obj.get_text();

        text[slice.0..slice.1].to_owned()
    }

    /// Returns a list of scriptures.
    pub(crate) fn get_scriptures(&self) -> Result<Vec<String>, BibleError> {
        let mut scripture_list: Vec<String> = Vec::new();

        for i in self.slice.iter() {
            let scripture_str = self.text.get(i.0..i.1).unwrap();
            // We need to validate if the found slice is a valid scripture
            if Bible::parse(scripture_str).is_ok() {
                scripture_list.push(scripture_str.to_string());
            }
        }

        Ok(scripture_list)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn t_is_prefix() {
        let text: &str = "Testing";
        let result_true: Script = Script::new(text).prefix("true");
        let result_false: Script = Script::new(text); // Defaults to `None`.

        assert!(result_true.is_prefix());
        assert!(!result_false.is_prefix());
    }

    #[test]
    fn t_is_postfix() {
        let text: &str = "Testing";
        let result_true: Script = Script::new(text).postfix("true");
        let result_false: Script = Script::new(text); // Defaults to `None`.

        assert!(result_true.is_postfix());
        assert!(!result_false.is_postfix());
    }

    #[test]
    fn t_find_slice_1() {
        let text: &str = "A popular scripture is John 3:16.";
        let expect: Vec<(usize, usize)> = vec![(23, 32)];
        let result: Script = Script::new(text);
        assert_eq!(result.slice, expect);
    }

    #[test]
    fn t_find_slice_2() {
        let text: &str = "John 3:16 and Matthew 24:14";
        let expect: Vec<(usize, usize)> = vec![(0, 9), (14, 27)];
        let result: Script = Script::new(text);
        assert_eq!(result.slice, expect);
    }

    #[test]
    fn t_find_slice_3() {
        let text: &str = "John 3:16, Mathew 24:14, and Psalms 83:18 are commonly used.";
        let result: Script = Script::new(text);
        assert_eq!(result.slice, vec![(0, 9), (11, 23), (29, 41)]);
    }

    #[test]
    fn t_single_scripture() {
        let text: &str = "John 3:16";
        let expect: &str = "[John 3:16]";
        let got: String = Script::new(text)
            .prefix("[")
            .postfix("]")
            .surround()
            .get_text();
        assert_eq!(got, expect);
    }

    #[test]
    // Tests if an empty "" is added if prefix is `None`.
    fn t_add_element_prefix_single_none() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let result: String = Script::new(text).surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_add_element_prefix_single() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is [John 3:16, it's quoted often.";
        let result: String = Script::new(text).prefix("[").surround().get_text();
        assert_eq!(result, expect);
    }

    #[test]
    fn t_add_element_prefix_single_to_string() {
        let text: String = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let expect: &str = "Another popular scripture is [John 3:16, it's quoted often.";
        let result: String = Script::new(text).prefix("[").surround().get_text();
        assert_eq!(result, expect);
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_prefix_multi() {
        let text: &str =
            "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: &str = "Two popular scripture are [prefix]John 3:16 and [prefix]Matthew 24:14. They are quoted often.";
        let got: String = Script::new(text).prefix("[prefix]").surround().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_add_element_postfix_single() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16[postfix], it's quoted often.";
        let got: String = Script::new(text).postfix("[postfix]").surround().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_add_element_postfix_multi() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Mathew 24:14. They are quoted often.";
        let expect: &str = "Two popular scriptures are John 3:16[postfix] and Mathew 24:14[postfix]. They are quoted often.";
        let got: String = Script::new(text).postfix("[postfix]").surround().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_single_url() {
        let text: &str = "A popular scriptures is John 3:16. It is quoted often.";
        let expect: String = "A popular scriptures is [John 3:16](https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016). It is quoted often.".to_string();
        let got: String = Script::new(text).url(&Site::JwOrg).unwrap().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_single_url_abbr() {
        let text: &str = "A popular scriptures is Joh 3:16. It is quoted often.";
        let expect: String = "A popular scriptures is [Joh 3:16](https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016). It is quoted often.".to_string();
        let got: String = Script::new(text).url(&Site::JwOrg).unwrap().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_get_scripture() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: Vec<&str> = vec!["John 3:16", "Matthew 24:14"];
        let got: Vec<String> = Script::new(text).get_scriptures().unwrap();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_get_from_slice() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect = "popular".to_string();
        let got: String = Script::new(text).get_from_slice(&(4, 11));
        assert_eq!(got, expect)
    }
}
