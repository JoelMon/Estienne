mod parsers;

/// The markup formats supported.
pub enum Markup {
    Markdown,
}

/// Adds the prefix and postfix around each scripture.
pub fn surround<S>(text: S, prefix: Option<&'static str>, postfix: Option<&'static str>) -> String
where
    S: Into<String> + Clone,
{
    parsers::surround::Script::new(text, prefix, postfix)
        .surround()
        .get_text()
}

pub fn get_scripture() -> Vec<&'static str> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_add_element_prefix_single() {
        let input: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expected: &str = "Another popular scripture is **John 3:16]], it's quoted often.";
        let result = surround(input, Some("**"), Some("]]"));
        assert_eq!(result, expected);
    }

    #[test]
    // Test if a String can be passed in as input.
    fn t_add_element_prefix_single_to_string() {
        let input: String = "Another popular scripture is John 3:16, it's quoted often.".into();
        let expected: &str = "Another popular scripture is **John 3:16]], it's quoted often.";
        let result = surround(input, Some("**"), Some("]]"));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi() {
        let input:&str = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.";
        let expected:&str = "Other popular scriptures include **John 3:16]], **Matthew 24:14]], and **Psalm 83:18]], they are quoted often.";
        let result = surround(input, Some("**"), Some("]]"));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi_noscripts() {
        let input: &str = "There are not scriptures in this line.";
        let expected: &str = "There are not scriptures in this line.";
        let result = surround(input, Some("**"), Some("]]"));
        assert_eq!(result, expected);
    }
}
