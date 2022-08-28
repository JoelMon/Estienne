mod locales;
mod parsers;

use once_cell::{self, sync::OnceCell};


// Initialize a safe global variable.
// Can be only set once but read many times.
static LOCALE: OnceCell<Locale> = OnceCell::new();

/// Enum to determine the language data set to use when executing.
// Works in conjunction with `impl Locale` to create a global variable
// with the `once_cell` crate.
#[allow(non_camel_case_types)]
pub enum Locale {
    en_us,
    es_sp,
}

#[allow(unused_must_use)]
impl Locale {
    /// Retrieves the value of `LOCALE`, may retrieve an arbitrary number of times.
    pub fn get() -> &'static Locale {
        LOCALE.get().expect("LOCALE was not initialized")
    }
    /// Sets the value of `LOCALE`, can only set once.
    //TODO: Return error if specific `locale` does not exist
    pub fn new(locale: Locale) {
        LOCALE.set(locale);
    }
}

// TODO: unignore doc test
/// Returns all scriptures in the passed in &str.
///
/// # Examples
/// ```rust;ignore
/// use estienne as est;
///
/// let line = "One of the most commonly cited scripture is John 3:16.";
///
/// let result = est::scripts(line: &str);
/// assert_eq!(result, Some(Vec["John 3:16"]));
/// ```
pub fn scripts(line: &str) -> Option<Vec<&str>> {
    todo!()
}

/// Returns true if the beginning of the &str is a valid scripture.
///
/// # Examples
/// ```rust
/// use est::Locale;
///
///
/// let script1 = "John 3:16";
/// let script2 = "Robert 3:16"; // an invalid scripture
///
/// est::Locale::new(Locale::en_us);
/// let result1 = est::is_scripture(script1);
/// assert_eq!(result1, true); // John 3:16 is valid
///
/// let result2 = est::is_scripture(script2);
/// assert_eq!(result2, false); // Robert 3:16 is not valid
/// ```
// pub fn is_scripture(input: &str) -> bool {
//     parsers::parsers::is_scripture(input)
// }

#[cfg(test)]

mod test {
    use crate::*;

    // #[test]
    /// Test whether a scripture is returned when it is found within a &str.
    fn t_return_scripts() {
        let input: &str = "We can see God's love for people at John 3:16.";
        let result = scripts(input);

        assert_eq!(result, Some(vec!["John 3:16"]));
    }
}
