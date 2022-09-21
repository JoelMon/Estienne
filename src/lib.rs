mod locales;
mod parsers;

pub fn surround(text: String, prefix: &str, postfix: &str) -> String {
    todo!()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_add_element_prefix_single() {
        let text = "Another popular scripture is John 3:16, it's quoted often.".to_string();
        let result = surround(text, "**", "**");
        assert_eq!(
            result,
            String::from(
                "Another popular scripture is this is a prefix **John 3:16** it's quoted often."
            )
        )
    }
}
