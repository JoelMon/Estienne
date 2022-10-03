//!# Estienne
//!
//!<img src="https://user-images.githubusercontent.com/6587811/190536556-aec1ba71-0aef-4878-9c1f-a9727e647083.png" alt="A digital image generated by an AI of a bearded man reading from a large book in the style of medieval painting." width=200 align=left hspace="20" vspace="20">
//!
//![![Rust](https://github.com/JoelMon/Estienne/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JoelMon/Estienne/actions/workflows/rust.yml)
//![![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
//!
//!Estienne is a simple to use library for interacting with Biblical verses from text. 
//!The focus of Estienne is to provide a library for building applications that need to interact with files that contain Biblical scriptures such as lecture notes.
//!No special syntax needs to be used in conjunction with scriptures in order for Estienne to parse the text successfully. 
//!
//!Estienne is still in its initial stages and not ready for use.
//!
//!## The Name
//!The library is named after Robert Estienne, a French theologian of the early Christian era. 
//!He is best remembered for being the first to print the New Testament divided with numbered verses. 
//![Read More](https://www.jw.org/finder?wtlocale=E&docid=2016167&srctype=wol&srcid=share&par=14)
//!
//!## The Purpose
//!The purpose of the library is to provide a simple way to manipulate Biblical verses within a line of text. 
//!The API will allow for easy manipulations of the verses, such as:
//!- Returning a list of verses found in a line of text
//!- Allowing a verses to be:
//!   - prefixed with arbitrary text 
//!   - suffixed with arbitrary text
//!   - etc.
//!
//!## Contributing
//!Contributions are welcomed, but please be aware that the project is still in its prototype phase and large portions of code might change at any moment. 
//!Feel free to open an issue if you have any questions or suggestions.


mod parsers;

/// The markup formats supported.
pub enum Markup {
    Markdown,
}

/// Adds the prefix and postfix around each scripture found in the `text`.
pub fn surround<'a, S>(text: S, prefix: &'a str, postfix: &'a str) -> String
where
    S: Into<String> + Clone,
{
    parsers::surround::Script::new(text)
        .prefix(prefix)
        .postfix(postfix)
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
        let result = surround(input, "**", "]]");
        assert_eq!(result, expected);
    }

    #[test]
    // Test if a String can be passed in as input.
    fn t_add_element_prefix_single_to_string() {
        let input: String = "Another popular scripture is John 3:16, it's quoted often.".into();
        let expected: &str = "Another popular scripture is **John 3:16]], it's quoted often.";
        let result = surround(input, "**", "]]");
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi() {
        let input:&str = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.";
        let expected:&str = "Other popular scriptures include **John 3:16]], **Matthew 24:14]], and **Psalm 83:18]], they are quoted often.";
        let result = surround(input, "**", "]]");
        assert_eq!(result, expected);
    }

    #[test]
    fn t_add_element_prefix_multi_no_scriptures() {
        let input: &str = "There are not scriptures in this line.";
        let expected: &str = "There are not scriptures in this line.";
        let result = surround(input, "**", "]]");
        assert_eq!(result, expected);
    }
}
