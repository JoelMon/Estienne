#![allow(dead_code, unused_variables)]

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
    postfix: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scripts {
    text: String,
    slice: Vec<ScriptSlice>,
    elements: Elements,
}

impl Scripts {
    // Find all scriptures in a line and return the beginning index and length.
    fn new(text: String, prefix: Option<String>, postfix: Option<String>) -> Self {
        let mut scrip_slices: Vec<ScriptSlice> = Vec::new();
        let re: &regex::Regex = &RE;

        for script in re.find_iter(text.as_str()) {
            scrip_slices.push((script.start(), script.end()));
        }

        Scripts {
            text,
            slice: scrip_slices,
            elements: Elements { prefix, postfix },
        }
    }

    fn add_prefix(mut self) -> Self {
        let prefix_len = self.elements.prefix.as_ref().map_or(0, |v| v.len());

        // After the first loop, the size of the prefix has to be taken into account to avoid
        // having the index off by prfix length. This happens because the index of all the
        // scriptures is taken before text is added to the original text line.
        for (i, scripture_loc) in self.slice.iter().enumerate() {
            if i < 1 {
                self.text.insert_str(
                    scripture_loc.0,
                    self.elements.prefix.as_ref().map_or("", |v| v.as_str()),
                );
            } else {
                self.text.insert_str(
                    scripture_loc.0 + prefix_len,
                    self.elements.prefix.as_ref().map_or("", |v| v.as_str()),
                );
            }
        }

        self
    }

    fn add_postfix(mut self) -> Self {
        let postfix_len = self.elements.postfix.as_ref().map_or(0, |v| v.len());

        // After the first loop, the size of the postfix has to be taken into account to avoid
        // having the index off by postfix length. This happens because the index of all the
        // scriptures is taken before text is added to the original text line.
        for (i, scripture_loc) in self.slice.iter().enumerate() {
            if i < 1 {
                self.text.insert_str(
                    (scripture_loc.0 + scripture_loc.1) - scripture_loc.0,
                    self.elements.postfix.as_ref().map_or("", |v| v.as_str()),
                );
            } else {
                self.text.insert_str(
                    (scripture_loc.0 + scripture_loc.1) - (scripture_loc.0 - postfix_len),
                    self.elements.postfix.as_ref().map_or("", |v| v.as_str()),
                );
            }
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_find_1() {
        let text = "A popular scripture is John 3:16.".to_string();
        let result = Scripts::new(text, None, None);
        assert_eq!(result.slice, vec![(23, 32)])
    }

    #[test]
    fn t_find_2() {
        let text = "John 3:16 and Matthew 24:14".to_string();
        let result = Scripts::new(text, None, None);
        assert_eq!(result.slice, vec![(0, 9), (14, 27)])
    }

    #[test]
    fn t_find_3() {
        let text = "John 3:16, Mathew 24:14, and Psalms 83:18 are commonly used.".to_string();
        let result = Scripts::new(text, None, None);
        assert_eq!(result.slice, vec![(0, 9), (11, 23), (29, 41)])
    }

    #[test]
    fn t_add_element_prefix_single() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = Scripts::new(text, Some("this is a prefix ".to_string()), None);
        let result = result.add_prefix();
        assert_eq!(
            result.text,
            String::from(
                "Another popular scripture is this is a prefix John 3:16, it's quoted often."
            )
        )
    }

    #[test]
    fn t_add_element_prefix_multi_1() {
        let text = "Two popular scripture are John 3:16 and Matthew 24:14. They are quoted often."
            .to_string();
        let result = Scripts::new(text, Some("[".to_string()), None);
        let result = result.add_prefix();
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
        let result = Scripts::new(text, Some("[prefix]".to_string()), None);
        let result = result.add_prefix();

        assert_eq!(
            result.text,
            String::from(
                "Two popular scripture are [prefix]John 3:16 and [prefix]Matthew 24:14. They are quoted often."
            )
        )
    }

    #[test]
    fn t_add_element_postfix() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = Scripts::new(text, None, Some("[postfix]".to_string()));
        let result = result.add_postfix();

        assert_eq!(
            result.text,
            String::from("Another popular scripture is John 3:16[postfix], it's quoted often.")
        )
    }

    #[test]
    fn t_add_element_postfix_multi() {
        let text = "Two popular scriptures are John 3:16 and Mathew 24:14. They are quoted often."
            .to_string();
        let result = Scripts::new(text, None, Some("[postfix]".to_string()));
        let result = result.add_postfix();

        assert_eq!(
            result.text,
            String::from(
                "Two popular scriptures are John 3:16[postfix] and Mathew 24:14[postfix]. They are quoted often."
            )
        )
    }
}
