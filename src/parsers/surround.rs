use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::{Borrow, Cow};

type ScriptSlice = (Start, End);
type Start = usize;
type End = usize;

// lazy_static insures that the regex is compiled only once.
lazy_static! {
    static ref RE: regex::Regex = Regex::new(r"(?:[1234]\s?)?([a-zA-Z]+)(\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\s*\d+[—–-]\d+|,\s*\d+)*(?:;\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*|;))*)?)").expect("error while compiling the regex in surround");
}

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
    // Find all scriptures in a line and return the beginning index and length.
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

    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.elements.prefix = Some(prefix);
        self
    }

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
    pub(crate) fn surround(mut self) -> Self {
        // .rev method is used to avoid dealing with the changing size of the string.
        for item in self.slice.iter().rev() {
            self.text.insert_str(
                item.0 + (item.1 - item.0),
                self.elements.postfix.map_or("", |v| v),
            );

            self.text
                .insert_str(item.0, self.elements.prefix.map_or("", |v| v));

            dbg!(&self.text);
        }

        self
    }

    /// Returns the text field of the Script struct.
    pub(crate) fn get_text(self) -> String {
        self.text
    }

    /// Returns a list of scriptures.
    pub(crate) fn get_scripture(self) -> Vec<String> {
        let mut scripture_list: Vec<String> = Vec::new();

        for i in self.slice.iter() {
            scripture_list.push(self.text.get(i.0..i.1).unwrap().to_string());
        }

        scripture_list
    }
}

// ########################################################################################################################
// ###################################################### UNIT TESTS ######################################################
// ########################################################################################################################

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
        let result: String = Script::new(text).prefix("[prefix]").surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_add_element_postfix_single() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16[postfix], it's quoted often.";
        let result: String = Script::new(text).postfix("[postfix]").surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_add_element_postfix_multi() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Mathew 24:14. They are quoted often.";
        let expect: &str = "Two popular scriptures are John 3:16[postfix] and Mathew 24:14[postfix]. They are quoted often.";
        let result: String = Script::new(text).postfix("[postfix]").surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_get_scripture() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: Vec<&str> = vec!["John 3:16", "Matthew 24:14"];
        let result: Vec<String> = Script::new(text).get_scripture();
        assert_eq!(result, expect)
    }
}
