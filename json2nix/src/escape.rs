use std::ops::Not;

pub fn escape_attribute_set_key(value: &str) -> String {
    if needs_escape(&value) {
        return format!("\"{}\"", value.replace("\"", "\\\""));
    } else {
        return value.to_string();
    }
}

const KEYWORDS: [&str; 13] = [
    "true", "false", "null", "import", "inherit", "rec", "with", "assert", "if", "then", "else", "let", "in",
];

fn needs_escape(value: &str) -> bool {
    value.is_empty()
        || KEYWORDS.contains(&value)
        || value.chars().take(1).all(|c| c.is_ascii_alphabetic().not())
        || value
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '-' || c == '_')
            .not()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn needs_no_escape() {
        let values = ["abc", "a-b", "aa-bb-cc", "AA_BB_CC", "a1", "A1-3"];
        for value in values {
            assert_eq!(value, escape_attribute_set_key(value));
        }
    }

    #[test]
    fn keywords() {
        for keyword in KEYWORDS {
            let expected = format!("\"{}\"", keyword);
            assert_eq!(expected, escape_attribute_set_key(keyword));
        }
    }

    #[test]
    fn simple_escapes() {
        let values = ["", " ", "   ", ".", "/", "1", "1a", "1a-", "-", "-a", "_", "_a", "a b"];
        for value in values {
            let expected = format!("\"{}\"", value);
            assert_eq!(expected, escape_attribute_set_key(value));
        }
    }

    #[test]
    fn escapes_with_quotes() {
        let values_and_expected = [("\"", r#""\"""#), ("\"hello\"", r#""\"hello\"""#)];
        for (value, expected) in values_and_expected {
            assert_eq!(expected, escape_attribute_set_key(value));
        }
    }
}
