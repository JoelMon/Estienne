

mod parsers;

pub fn surround(text: String, prefix: Option<String>, postfix: Option<String>) -> String {

    parsers::souround::Scribe::new(text, prefix, postfix)
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
        let result = surround(
            input.clone(),
            Some("**".to_string()),
            Some("]]".to_string()),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi() {
        let input:String = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.".to_string();
        let expected:String = "Other popular scriptures include **John 3:16]], **Matthew 24:14]], and **Psalm 83:18]], they are quoted often.".to_string();

        let result = surround(
            input.clone(),
            Some("**".to_string()),
            Some("]]".to_string()),
        );
        assert_eq!(result, expected);
    }
}
