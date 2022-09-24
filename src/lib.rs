mod parsers;

/// Adds the prefix and postfix around each scripture.
pub fn surround(text: String, prefix: Option<String>, postfix: Option<String>) -> String {
    parsers::souround::Scribe::new(text, prefix, postfix)
        .surround()
        .get_text()
}

/// Adds the prefix around each scripture.
pub fn prefix(text: String, prefix: Option<String>) -> String {
    parsers::souround::Scribe::new(text, prefix, None)
        .surround()
        .get_text()
}

/// Adds the postfix around each scripture.
pub fn postfix(text: String, postfix: Option<String>) -> String {
    parsers::souround::Scribe::new(text, None, postfix)
        .surround()
        .get_text()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_add_element_prefix_single() {
        let input: String =
            "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let expected: String =
            "Another popular scripture is **John 3:16]], it's quoted often.".to_string();
        let result = surround(input, Some("**".to_string()), Some("]]".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi() {
        let input:String = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.".to_string();
        let expected:String = "Other popular scriptures include **John 3:16]], **Matthew 24:14]], and **Psalm 83:18]], they are quoted often.".to_string();

        let result = surround(input, Some("**".to_string()), Some("]]".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_prefix_multi_1() {
        let input:String = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.".to_string();
        let expected:String = "Other popular scriptures include **John 3:16, **Matthew 24:14, and **Psalm 83:18, they are quoted often.".to_string();
        let result = prefix(input, Some("**".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_postfix_multi_1() {
        let input:String = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.".to_string();
        let expected:String = "Other popular scriptures include John 3:16**, Matthew 24:14**, and Psalm 83:18**, they are quoted often.".to_string();
        let result = postfix(input, Some("**".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi_noscripts() {
        let input:String = "There are not scriptures in this line.".to_string();
        let expected:String = "There are not scriptures in this line.".to_string();

        let result = surround(input, Some("**".to_string()), Some("]]".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_prefix_multi_1_noscripts() {
        let input:String = "There are not scriptures in this line.".to_string();
        let expected:String = "There are not scriptures in this line.".to_string();
        let result = prefix(input, Some("**".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn t_postfix_multi_1_noscripts() {
        let input:String = "There are not scriptures in this line.".to_string();
        let expected:String = "There are not scriptures in this line.".to_string();
        let result = postfix(input, Some("**".to_string()));
        assert_eq!(result, expected);
    }
}
