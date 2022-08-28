#![allow(dead_code)]

struct Scripture {
    name: String,
    chapter: u8,
    verse: u8,
}

impl Scripture {
    fn find(line: &str) -> Option<Vec<Scripture>> {
        if line.is_empty() {return None};
        todo!()
    }

    fn set(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::locales::en_us::book;

    #[test]
    fn t_find() {
        let result = book("Matthew");
        assert_eq!(result.unwrap(), "Matthew")
    }
}
