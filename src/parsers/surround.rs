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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Elements {
    prefix: Option<&'static str>,
    prefix_len: Option<usize>,
    postfix: Option<&'static str>,
    postfix_len: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Script {
    elements: Elements,
    slice: Vec<ScriptSlice>,
    text: String,
}

#[allow(unused_variables)]
impl Script {
    // Find all scriptures in a line and return the beginning index and length.
    pub(crate) fn new<S>(
        text: S,
        prefix: Option<&'static str>,
        postfix: Option<&'static str>,
    ) -> Self
    where
        S: Into<String> + Clone,
    {
        let mut scrip_slices: Vec<ScriptSlice> = Vec::new();
        let re: &regex::Regex = &RE;

        // TODO: Fix this section of code. Too much cloning, find more concise exp.
        //       The use of Cow is due to regex requiring a &str but it is possible that
        //       `text` is `String` or `&str`, eliminating the use of the `as_str()` method.
        let text_copy = text.clone();
        let text1: Cow<str> = Cow::from(text_copy.into());
        for script in re.find_iter(Cow::borrow(&text1)) {
            scrip_slices.push((script.start(), script.end()));
        }

        let prefix_len = prefix.map(|x| x.len());
        let postfix_len = postfix.map(|x| x.len());

        Self {
            text: text.into(),
            slice: scrip_slices,
            elements: Elements {
                prefix,
                prefix_len,
                postfix,
                postfix_len,
            },
        }
    }

    #[allow(dead_code)]
    fn is_prefix(&self) -> bool {
        self.elements.prefix.is_some()
    }

    #[allow(dead_code)]
    fn is_postfix(&self) -> bool {
        self.elements.postfix.is_some()
    }

    // TODO: Document this, in a few weeks I'll forget why it's rev.
    pub(crate) fn surround(mut self) -> Self {
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

    /// Returns the text.
    pub(crate) fn get_text(self) -> String {
        self.text
    }

    #[allow(dead_code)]
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

    #[test]
    fn t_is_prefix() {
        let text: &str = "Testing";
        let prefix: Option<&str> = Some("true");
        let result_true: Script = Script::new(text, prefix, None);
        let result_false: Script = Script::new(text, None, None);

        assert!(result_true.is_prefix());
        assert!(!result_false.is_prefix());
    }

    #[test]
    fn t_is_postfix() {
        let text: &str = "Testing";
        let postfix: Option<&str> = Some("true");
        let result_true: Script = Script::new(text, None, postfix);
        let result_false: Script = Script::new(text, None, None);

        assert!(result_true.is_postfix());
        assert!(!result_false.is_postfix());
    }

    #[test]
    fn t_find_1() {
        let text: &str = "A popular scripture is John 3:16.";
        let expect: Vec<(usize, usize)> = vec![(23, 32)];
        let result: Script = Script::new(text, None, None);
        assert_eq!(result.slice, expect);
    }

    #[test]
    fn t_find_2() {
        let text: &str = "John 3:16 and Matthew 24:14";
        let expect: Vec<(usize, usize)> = vec![(0, 9), (14, 27)];
        let result: Script = Script::new(text, None, None);
        assert_eq!(result.slice, expect);
    }

    #[test]
    fn t_find_3() {
        let text: &str = "John 3:16, Mathew 24:14, and Psalms 83:18 are commonly used.";
        let result: Script = Script::new(text, None, None);
        assert_eq!(result.slice, vec![(0, 9), (11, 23), (29, 41)]);
    }

    #[test]
    fn t_add_element_prefix_single() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is [John 3:16, it's quoted often.";
        let result: String = Script::new(text, Some("["), None).surround().get_text();
        assert_eq!(result, expect);
    }

    #[test]
    fn t_add_element_prefix_single_to_string() {
        let text: String = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let expect: &str = "Another popular scripture is [John 3:16, it's quoted often.";
        let result: String = Script::new(text, Some("["), None).surround().get_text();
        assert_eq!(result, expect);
    }

    #[test]
    // Tests if an empty "" is added if prefix is `None`.
    fn t_add_element_prefix_single_none() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let result: String = Script::new(text, None, None).surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_add_element_prefix_multi_1() {
        let text: &str =
            "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: &str =
            "Two popular scripture are [John 3:16 and [Matthew 24:14. They are quoted often.";
        let result: String = Script::new(text, Some("["), None).surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_prefix_multi_2() {
        let text: &str =
            "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: &str = "Two popular scripture are [prefix]John 3:16 and [prefix]Matthew 24:14. They are quoted often.";
        let result: String = Script::new(text, Some("[prefix]"), None)
            .surround()
            .get_text();
        assert_eq!(result, expect)
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_prefix_multi_3() {
        let text: &str = "Three popular scripture are John 3:16, Matthew 24:14 and also Psalms 83:18 . They are quoted often.";
        let expect: &str = "Three popular scripture are =>John 3:16, =>Matthew 24:14 and also =>Psalms 83:18 . They are quoted often.";
        let result: String = Script::new(text, Some("=>"), None).surround().get_text();

        assert_eq!(result, expect)
    }

    #[test]
    fn t_add_element_postfix() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16[postfix], it's quoted often.";
        let result: String = Script::new(text, None, Some("[postfix]"))
            .surround()
            .get_text();
        assert_eq!(result, expect)
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_postfix_single_none() {
        let text: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let result: String = Script::new(text, None, None).surround().get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_add_element_postfix_multi() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Mathew 24:14. They are quoted often.";
        let expect: &str = "Two popular scriptures are John 3:16[postfix] and Mathew 24:14[postfix]. They are quoted often.";
        let result: String = Script::new(text, None, Some("[postfix]"))
            .surround()
            .get_text();
        assert_eq!(result, expect)
    }

    #[test]
    fn t_get_scripture() {
        let text: &str =
            "Two popular scriptures are John 3:16 and Matthew 24:14. They are quoted often.";
        let expect: Vec<&str> = vec!["John 3:16", "Matthew 24:14"];
        let result: Vec<String> = Script::new(text, None, None).get_scripture();
        assert_eq!(result, expect)
    }
}
