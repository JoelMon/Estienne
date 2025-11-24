use std::borrow::{Borrow, Cow};
use crate::parsers::scripture::RE;

use crate::{
    locales::{nwt_en::Site, BibleError},
    url::Url,
};

use super::scripture::Bible;

/// _ScriptSlice_ type describes as a tuple the begining and ending index plus one for a scripture found in a string.
///
/// Take the following string as an example, "Scripture Mathrew 3:16."
/// The `ScriptSlice` will be as follows:
/// - Start = 10
/// - End = 22 (21 + 1)
pub type ScriptSlice = (Start, End);
type Start = usize;
type End = usize;

/// The _ScriptureCollection_ is a vector contain scruptures. TODO: Fix docs
pub type ScriptureCollection = Vec<String>;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
/// _Elements_ is a struct that descibes the _elements_ that will be placed before and after a scripture.
/// For example, if the elements, `<strong>` and `</strong>` were to surround a scripture then `<strong>` would be
/// the prefix with a prefix length of `5` and `</strong>` would be the postfix with a length of `6`. 
/// The prefix and postfix lengths are important to correctly calculate the insertion of the lements before 
/// and after the scripture(s).
struct Elements<'a> {
    /// The optional prefix string that will be inserted before a scripture.
    prefix: Option<&'a str>,
    /// The lenth of chareters in the prefix.
    prefix_len: Option<usize>,
    /// The optional postfix string that will be inserted after a scripture.
    postfix: Option<&'a str>,
    /// The lenth of chareters in the postfix.
    postfix_len: Option<usize>,
}
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone)]
/// _Locations_ contains the start and end indexes of all the scriptures found in the string.
pub struct Locations{
    /// The start and end indexes of all the scriptures found in the string passed in.
    pub slices: Vec<ScriptSlice>,
    /// The original string that was passed in.
    pub string: String,
}



#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// The _Script_ struct describes properties needed for sucessfully wrapping a scripture with _elements_.
pub struct Script<'a> {
    /// The _elements_ that will wrap around the scripture. i.e. the _prefix_ and _postfix_.
    elements: Elements<'a>,
    /// Both the start and end (+1) index for the scripture(s) within the string to be able to locate where the scriptures are found within the string.
    slices: Vec<ScriptSlice>,
    /// The full string that was passed into the library that contains the scripture.
    string: String,
}

#[allow(unused_variables)]
impl<'a> Script<'a> {
    /// Find all _potential_ scriptures in a string and return the beginning index and length.
    /// Will accept `&str` or `String` types.
    pub(crate) fn new<S>(text: S) -> Self
    where
        S: Into<String> + Clone,
    {
        let re: &regex::Regex = &RE;

        let text_to_str: Cow<str> = Cow::from(text.clone().into());
        let matches = re.find_iter(Cow::borrow(&text_to_str));

        let mut scrip_slices: Vec<ScriptSlice> = Vec::new();
        for script in matches {
            scrip_slices.push((script.start(), script.end()));
        }

        Self {
            string: text.into(),
            slices: scrip_slices,
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
        for item in self.slices.iter().rev() {
            self.string.insert_str(
                item.0 + (item.1 - item.0),
                self.elements
                    .postfix
                    .map_or("", |postfix_value| postfix_value),
            );

            self.string.insert_str(
                item.0,
                self.elements.prefix.map_or("", |prefix_value| prefix_value),
            );
        }

        self
    }

    /// Returns the original string with URL markup for all scriptures.
    pub(crate) fn url(mut self, site: &Site) -> Result<Self, BibleError> {
        // .rev() method is used to avoid dealing with the changing size of the string as new characters are added.
        for (start, end) in self.slices.iter().rev() {
            let verse_slice: String = self.get_from_slice(&(*start, *end));
            let bible: Bible = Bible::parse(verse_slice.as_str())?;
            let url: String = site.get_url(&bible)?;

            self.string
                .insert_str(*start + (*end - *start), format!("]({})", url).as_str());

            self.string.insert(*start, '[');
        }

        Ok(self)
    }

    /// Returns the text field of the Script struct.
    pub(crate) fn get_text(self) -> String {
        self.string
    }

    // Returns a `String` to avoid borrowing headaches
    pub(crate) fn get_from_slice(&self, slice: &(usize, usize)) -> String {
        let clone_obj: Script<'a> = self.clone();
        let text: String = clone_obj.get_text();

        text[slice.0..slice.1].to_owned()
    }

    /// Returns a collection of scriptures found in the string passed in.
    pub(crate) fn get_scriptures(&self) -> Result<ScriptureCollection, BibleError> {
        let mut scripture_list:Vec<String>  = Vec::new();

        for i in self.slices.iter() {
            let scripture_str: &str = self.string.get(i.0..i.1).unwrap();

            // We need to validate if the found slice contains a valid Bible book name.
            if Bible::parse(scripture_str).is_ok() {
                scripture_list.push(scripture_str.to_string());
            }
        }

        Ok(scripture_list)
    }

    /// Returns the index of the start and end of each scripture found and also the original string.
    pub(crate) fn get_locations(&self) -> Locations {
        Locations { slices: self.slices.clone(), string: self.string.clone() }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_prefix() {
        let text: &str = "Testing";
        let result_true: Script = Script::new(text).prefix("true");
        let result_false: Script = Script::new(text); // Defaults to `None`.

        assert!(result_true.is_prefix());
        assert!(!result_false.is_prefix());
    }

    #[test]
    fn is_postfix() {
        let text: &str = "Testing";
        let result_true: Script = Script::new(text).postfix("true");
        let result_false: Script = Script::new(text); // Defaults to `None`.

        assert!(result_true.is_postfix());
        assert!(!result_false.is_postfix());
    }

    #[test]
    fn find_slice_1() {
        let text: &str = "A popular scripture is John 3:16.";
        let expect: Vec<(usize, usize)> = vec![(23, 32)];
        let result: Script = Script::new(text);
        assert_eq!(result.slices, expect);
    }

    #[test]
    fn find_slice_2() {
        let text: &str = "John 3:16 and Matthew 24:14";
        let expect: Vec<(usize, usize)> = vec![(0, 9), (14, 27)];
        let result: Script = Script::new(text);
        assert_eq!(result.slices, expect);
    }

    #[test]
    fn find_slice_3() {
        let text: &str = "John 3:16, Mathew 24:14, and Psalms 83:18 are commonly used.";
        let result: Script = Script::new(text);
        assert_eq!(result.slices, vec![(0, 9), (11, 23), (29, 41)]);
    }

    #[test]
    fn single_scripture() {
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
    fn add_element_prefix_single_none() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let result: String = Script::new(text).surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn add_element_prefix_single() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is [John 3:16, it's quoted often.";
        let result: String = Script::new(text).prefix("[").surround().get_text();
        assert_eq!(result, expect);
    }

    #[test]
    fn add_element_prefix_single_to_string() {
        let text: String = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let expect: &str = "Another popular scripture is [John 3:16, it's quoted often.";
        let result: String = Script::new(text).prefix("[").surround().get_text();
        assert_eq!(result, expect);
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn add_element_prefix_multi() {
        let text: &str =
            "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: &str = "Two popular scripture are [prefix]John 3:16 and [prefix]Matthew 24:14. They are quoted often.";
        let got: String = Script::new(text).prefix("[prefix]").surround().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn add_element_postfix_single() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16[postfix], it's quoted often.";
        let got: String = Script::new(text).postfix("[postfix]").surround().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn add_element_postfix_multi() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Mathew 24:14. They are quoted often.";
        let expect: &str = "Two popular scriptures are John 3:16[postfix] and Mathew 24:14[postfix]. They are quoted often.";
        let got: String = Script::new(text).postfix("[postfix]").surround().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn single_url() {
        let text: &str = "A popular scriptures is John 3:16. It is quoted often.";
        let expect: String = "A popular scriptures is [John 3:16](https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016). It is quoted often.".to_string();
        let got: String = Script::new(text).url(&Site::JwOrg).unwrap().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn single_url_abbr() {
        let text: &str = "A popular scriptures is Joh 3:16. It is quoted often.";
        let expect: String = "A popular scriptures is [Joh 3:16](https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016). It is quoted often.".to_string();
        let got: String = Script::new(text).url(&Site::JwOrg).unwrap().get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn get_scripture() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: Vec<&str> = vec!["John 3:16", "Matthew 24:14"];
        let got: Vec<String> = Script::new(text).get_scriptures().unwrap();
        assert_eq!(got, expect)
    }

    #[test]
    fn get_from_slice() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect = "popular".to_string();
        let got: String = Script::new(text).get_from_slice(&(4, 11));
        assert_eq!(got, expect)
    }

    #[test]
    fn get_locations1(){
        let text: &str = "John 3:16 is well known.";
        let expect: Locations = Locations{ slices: vec![(0, 9)], string: text.into() };
        let got: Locations = Script::new(text).get_locations();
        assert_eq!(got, expect);
    }

    #[test]
    fn get_locations2(){
        let text: &str = "John 3:16 is well known and if you know it, then it's easy to remember Timothy 3:16, another important scripture.";
        let expect: Locations = Locations{ slices: vec![(0, 9), (71,83)], string: text.into() };
        let got: Locations = Script::new(text).get_locations();
        assert_eq!(got, expect);
    }
}
