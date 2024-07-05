use serde_json::Value;

pub fn json2nix(json: &str) -> Result<String, ()> {
    let parsed: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            return Err(());
        }
    };

    Ok(to_nix(&parsed))
}

fn to_nix(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(bool) => bool.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(string) => format!(r#""{}""#, string),
        Value::Array(array) => match array.len() {
            0 => "[ ]".to_string(),
            1 => format!("[ {} ]", to_nix(&array[0])),
            _ => todo!(),
        },
        Value::Object(object) => match object.len() {
            0 => "{ }".to_string(),
            _ => {
                let mut formatted_elements = Vec::with_capacity(object.len());
                for (key, value) in object {
                    formatted_elements.push(format!("{} = {};", key, to_nix(value)));
                }
                format!(
                    "{{\n{}\n}}",
                    formatted_elements
                        .iter()
                        .map(|fe| format!("  {}", fe))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        },
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{json, value::Number, Map, Value};

    #[test]
    fn null() {
        assert_eq!("null", to_nix(&Value::Null));
    }

    #[test]
    fn bool_false() {
        assert_eq!("false", to_nix(&Value::Bool(false)));
    }

    #[test]
    fn bool_true() {
        assert_eq!("true", to_nix(&Value::Bool(true)));
    }

    #[test]
    fn number_positive_integer() {
        let input = Value::Number(Number::from(123));
        assert_eq!("123", to_nix(&input));
    }

    #[test]
    fn number_negative_integer() {
        let input = Value::Number(Number::from(-123));
        assert_eq!("-123", to_nix(&input));
    }

    #[test]
    fn number_positive_float() {
        let input = Value::Number(Number::from_f64(123.5505).unwrap());
        assert_eq!("123.5505", to_nix(&input));
    }

    #[test]
    fn number_negative_float() {
        let input = Value::Number(Number::from_f64(-123.5505).unwrap());
        assert_eq!("-123.5505", to_nix(&input));
    }

    #[test]
    fn string_empty() {
        assert_eq!(r#""""#, to_nix(&Value::String("".to_string())));
    }

    #[test]
    fn string_simple() {
        let input = Value::String("Hello, world!".to_string());
        let expected = r#""Hello, world!""#;
        assert_eq!(expected, to_nix(&input));
    }

    #[test]
    fn array_empty() {
        let input = Value::Array(vec![]);
        let expected = "[ ]";
        assert_eq!(expected, to_nix(&input));
    }

    #[test]
    fn array_single_bool() {
        let input = Value::Array(vec![Value::Bool(true)]);
        let expected = "[ true ]";
        assert_eq!(expected, to_nix(&input));
    }

    #[test]
    fn object_empty() {
        let input = Value::Object(Map::new());
        let expected = "{ }";
        assert_eq!(expected, to_nix(&input));
    }

    #[test]
    fn object_single_key() {
        let input = json!({ "key": "value" });
        let expected = "{\n  key = \"value\";\n}";
        assert_eq!(expected, to_nix(&input));
    }
}
