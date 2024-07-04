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

mod locales;
mod parsers;
mod url;
use locales::en_us::Site;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Locale {
    /// American English
    en_us,
    es_es,
}

/// Adds the prefix and postfix around each scripture found in the `text`.
pub fn surround<'a, S: Into<String> + Clone>(text: S, prefix: &'a str, postfix: &'a str) -> String {
    parsers::surround::Script::new(text)
        .prefix(prefix)
        .postfix(postfix)
        .surround()
        .get_text()
}

/// Links scriptures found to a online Bible.
pub fn url<S: Into<String> + Clone>(site: &Site, text: S) -> String {
    parsers::surround::Script::new(text)
        .url(site)
        .get_text()
}

#[cfg(test)]
mod lib_test {
    use crate::parsers::surround::Script;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn t_single_url() {
        let input: &str = "A popular scriptures is Re 12:12. It is quoted often.";
        let expect: String = "A popular scriptures is [Re 12:12](https://www.jw.org/en/library/bible/study-bible/books/revelation/12/#v66012012). It is quoted often.".to_string();
        let got: String = Script::new(input).url( &Site::JwOrg).get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_single_ranged_url() {
        let input: &str = "A popular scriptures is Job 36:26-28. It is quoted often.";
        let expect: String = "A popular scriptures is [Job 36:26-28](https://www.jw.org/en/library/bible/study-bible/books/job/36/#v18036026-v18036028). It is quoted often.".to_string();
        let got: String = Script::new(input).url( &Site::JwOrg).get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_multipal_url() {
        let input: &str = "Three well-known Bible scriptures are Proverbs 3:5, John 3:16, and Romans 8:28";
        let expect: String = "Three well-known Bible scriptures are [Proverbs 3:5](https://www.jw.org/en/library/bible/study-bible/books/proverbs/3/#v20003005), [John 3:16](https://www.jw.org/en/library/bible/study-bible/books/john/3/#v43003016), and [Romans 8:28](https://www.jw.org/en/library/bible/study-bible/books/romans/8/#v45008028)".to_string();
        let got: String = Script::new(input).url( &Site::JwOrg).get_text();
        assert_eq!(got, expect)
    }

    #[test]
    fn t_add_element_prefix_single() {
        let input: &str = "Another popular scripture is John 3:16, it's quoted often.";
        let expect: &str = "Another popular scripture is **John 3:16]], it's quoted often.";
        let got = surround(input, "**", "]]");
        assert_eq!(got, expect);
    }

    #[test]
    // Test if a String can be passed in as input.
    fn t_add_element_prefix_single_to_string() {
        let input: String = "Another popular scripture is John 3:16, it's quoted often.".into();
        let expect: &str = "Another popular scripture is **John 3:16]], it's quoted often.";
        let got = surround(input, "**", "]]");
        assert_eq!(got, expect);
    }

    #[test]
    fn t_add_element_prefix_multi() {
        let input:&str = "Other popular scriptures include John 3:16, Matthew 24:14, and Psalm 83:18, they are quoted often.";
        let expect:&str = "Other popular scriptures include **John 3:16]], **Matthew 24:14]], and **Psalm 83:18]], they are quoted often.";
        let got = surround(input, "**", "]]");
        assert_eq!(got, expect);
    }

    #[test]
    fn t_add_element_prefix_ranged_multi() {
        let input:&str = "Other popular scriptures include John 3:16, 17, Matthew 24:14-16, and Psalm 83:18, 17-20, they are quoted often.";
        let expect:&str = "Other popular scriptures include **John 3:16, 17]], **Matthew 24:14-16]], and **Psalm 83:18, 17-20]], they are quoted often.";
        let got = surround(input, "**", "]]");
        assert_eq!(got, expect);
    }

    #[test]
    fn t_add_element_prefix_multi_no_scriptures() {
        let input: &str = "There are no scriptures in this line.";
        let expect: &str = "There are no scriptures in this line.";
        let got = surround(input, "**", "]]");
        assert_eq!(got, expect);
    }
}
