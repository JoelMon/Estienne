use std::string::String;

use lazy_static::lazy_static;
use regex::Regex;

type ScriptSlice = (Start, End);
type Start = usize;
type End = usize;

// lazy_static insures that the regex is compiled only once.
lazy_static! {
    static ref RE: regex::Regex = Regex::new(r"(?:[1234]\s?)?([a-zA-Z]+)(\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\s*\d+[—–-]\d+|,\s*\d+)*(?:;\s?\d+(?::(?:\d+[—–-]\d+|\d+)(?:,\d+[—–-]\d+|,\d+)*|;))*)?)").expect("error while compiling the regex");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Elements {
    prefix: Option<String>,
    prefix_len: Option<usize>,
    postfix: Option<String>,
    postfix_len: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scribe {
    text: String,
    slice: Vec<ScriptSlice>,
    elements: Elements,
}

impl Scribe {
    // Find all scriptures in a line and return the beginning index and length.
    pub(crate) fn new(text: String, prefix: Option<String>, postfix: Option<String>) -> Self {
        let mut scrip_slices: Vec<ScriptSlice> = Vec::new();
        let re: &regex::Regex = &RE;

        for script in re.find_iter(text.as_str()) {
            scrip_slices.push((script.start(), script.end()));
        }

        let prefix_len = prefix.as_ref().map(String::len);
        let postfix_len = postfix.as_ref().map(String::len);

        Self {
            text,
            slice: scrip_slices,
            elements: Elements { prefix, prefix_len, postfix, postfix_len },
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

    pub(crate) fn surround(mut self) -> Self {
        let post_len = &self.elements.postfix_len.map_or(0, |v| v);

        for item in self.slice.iter().rev() {
            // dbg!(&item);

            dbg!(post_len);
            dbg!(&item.0, &item.1);
            self.text.insert_str(
                item.0 + (item.1-item.0),
                self.elements.postfix.as_ref().map_or("", |v| v.as_str()),
            );

            self.text.insert_str(
                item.0,
                self.elements.prefix.as_ref().map_or("", |v| v.as_str()),
            );
        }

        self
    }

    pub(crate) fn get_text(self) -> String {
        self.text
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_is_prefix() {
        let text: String = "Testing".to_string();
        let prefix: Option<String> = Some("true".to_string());
        let result_true = Scribe::new(text.clone(), prefix, None);
        let result_false = Scribe::new(text, None, None);

        assert!(result_true.is_prefix());
        assert!(!result_false.is_prefix());
    }

    #[test]
    fn t_is_postfix() {
        let text: String = "Testing".to_string();
        let postfix: Option<String> = Some("true".to_string());
        let result_true = Scribe::new(text.clone(), None, postfix);
        let result_false = Scribe::new(text, None, None);

        assert!(result_true.is_postfix());
        assert!(!result_false.is_postfix());
    }

    #[test]
    fn t_find_1() {
        let text = "A popular scripture is John 3:16.".to_string();
        let result = Scribe::new(text, None, None);
        assert_eq!(result.slice, vec![(23, 32)]);
    }

    #[test]
    fn t_find_2() {
        let text = "John 3:16 and Matthew 24:14".to_string();
        let result = Scribe::new(text, None, None);
        assert_eq!(result.slice, vec![(0, 9), (14, 27)]);
    }

    #[test]
    fn t_find_3() {
        let text = "John 3:16, Mathew 24:14, and Psalms 83:18 are commonly used.".to_string();
        let result = Scribe::new(text, None, None);
        assert_eq!(result.slice, vec![(0, 9), (11, 23), (29, 41)]);
    }

    #[test]
    fn t_add_element_prefix_single() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = Scribe::new(text, Some("[".to_string()), None);
        let result = result.surround();
        assert_eq!(
            result.text,
            String::from("Another popular scripture is [John 3:16, it's quoted often.")
        );
    }

    #[test]
    // Tests if an empty "" is added if prefix is `None`.
    fn t_add_element_prefix_single_none() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = Scribe::new(text, None, None);
        let result = result.surround();
        assert_eq!(
            result.text,
            String::from("Another popular scripture is John 3:16, it's quoted often.")
        )
    }

    #[test]
    fn t_add_element_prefix_multi_1() {
        let text = "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often."
            .to_string();
        let result = Scribe::new(text, Some("[".to_string()), None);
        let result = result.surround();
        assert_eq!(
            result.text,
            String::from(
                "Two popular scripture are [John 3:16 and [Matthew 24:14. They are quoted often."
            )
        )
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_prefix_multi_2() {
        let text = "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often."
            .to_string();
        let result = Scribe::new(text, Some("[prefix]".to_string()), None);
        let result = result.surround();

        assert_eq!(
            result.text,
            String::from(
                "Two popular scripture are [prefix]John 3:16 and [prefix]Matthew 24:14. They are quoted often."
            )
        )
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_prefix_multi_3() {
        let text = "Three popular scripture are John 3:16, Matthew 24:14 and also Psalms 83:18 . They are quoted often."
            .to_string();
        let result = Scribe::new(text, Some("=>".to_string()), None);
        let result = result.surround();

        assert_eq!(
            result.text,
            String::from(
                "Three popular scripture are =>John 3:16, =>Matthew 24:14 and also =>Psalms 83:18 . They are quoted often."
            )
        )
    }

    #[test]
    fn t_add_element_postfix() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = Scribe::new(text, None, Some("[postfix]".to_string()));
        let result = result.surround();
        assert_eq!(
            result.text,
            String::from("Another popular scripture is John 3:16[postfix], it's quoted often.")
        )
    }

    #[test]
    // Tests to make sure the prefix len is taken into consideration for each scripture found.
    fn t_add_element_postfix_single_none() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = Scribe::new(text, None, None);
        let result = result.surround();
        assert_eq!(
            result.text,
            String::from("Another popular scripture is John 3:16, it's quoted often.")
        )
    }

    #[test]
    fn t_add_element_postfix_multi() {
        let text = "Two popular scriptures are John 3:16 and Mathew 24:14. They are quoted often."
            .to_string();
        let result = Scribe::new(text, None, Some("[postfix]".to_string()));
        let result = result.surround();

        assert_eq!(
            result.text,
            String::from(
                "Two popular scriptures are John 3:16[postfix] and Mathew 24:14[postfix]. They are quoted often."
            )
        )
    }
}
