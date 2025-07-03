mod escape;
mod indent;

use escape::escape_attribute_set_key;
use indent::indent;
use serde_json::Value;

#[derive(Debug)]
pub struct Json2NixConfig {
    pub initial_indentation: usize,
    pub indentation_increment: usize,
}

impl Json2NixConfig {
    pub fn new(initial_indentation: usize, indentation_increment: usize) -> Self {
        Self {
            initial_indentation,
            indentation_increment,
        }
    }
}

pub fn json2nix(input: &str, config: &Json2NixConfig) -> Result<String, String> {
    let json: Value = match serde_json::from_str(input) {
        Ok(value) => value,
        Err(err) => {
            return Err(format!("Could not parse the input as JSON: {}", err.to_string()));
        }
    };

    Ok(indent(&to_nix(&json, config.initial_indentation, config.indentation_increment), config.initial_indentation))
}

fn to_nix(value: &Value, indentation: usize, indentation_increment: usize) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(bool) => bool.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(string) => format!(r#""{}""#, string),
        Value::Array(array) => match array.len() {
            0 => "[ ]".to_string(),
            _ => {
                let mut formatted_elements = Vec::with_capacity(array.len());
                for element in array {
                    formatted_elements.push(to_nix(element, indentation + indentation_increment, indentation_increment));
                }
                format!(
                    "[\n{}\n{}",
                    formatted_elements
                        .iter()
                        .map(|fe| indent(fe, indentation + indentation_increment))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    indent("]", indentation)
                )
            }
        },
        Value::Object(object) => match object.len() {
            0 => "{ }".to_string(),
            _ => {
                let mut formatted_elements = Vec::with_capacity(object.len());
                for (key, value) in object {
                    formatted_elements.push(format!(
                        "{} = {};",
                        escape_attribute_set_key(key),
                        to_nix(value, indentation + indentation_increment, indentation_increment)
                    ));
                }
                format!(
                    "{{\n{}\n{}",
                    formatted_elements
                        .iter()
                        .map(|fe| indent(fe, indentation + indentation_increment))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    indent("}", indentation)
                )
            }
        },
    }
}

#[cfg(test)]
mod test {
    use std::ops::Not;

    use super::*;
    use serde_json::{json, value::Number, Map, Value};

    fn trim_indent(input: &str) -> String {
        let common_indent = input
            .lines()
            .filter(|line| line.is_empty().not())
            .map(|line| line.chars().take_while(|c| *c == ' ').count())
            .min()
            .unwrap_or(0);

        input
            .lines()
            .map(|line| line.chars().skip(common_indent).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    #[test]
    fn null() {
        assert_eq!("null", to_nix(&Value::Null, 0, 0));
    }

    #[test]
    fn bool_false() {
        assert_eq!("false", to_nix(&Value::Bool(false), 0, 0));
    }

    #[test]
    fn bool_true() {
        assert_eq!("true", to_nix(&Value::Bool(true), 0, 0));
    }

    #[test]
    fn number_positive_integer() {
        let input = Value::Number(Number::from(123));
        assert_eq!("123", to_nix(&input, 0, 0));
    }

    #[test]
    fn number_negative_integer() {
        let input = Value::Number(Number::from(-123));
        assert_eq!("-123", to_nix(&input, 0, 0));
    }

    #[test]
    fn number_positive_float() {
        let input = Value::Number(Number::from_f64(123.5505).unwrap());
        assert_eq!("123.5505", to_nix(&input, 0, 0));
    }

    #[test]
    fn number_negative_float() {
        let input = Value::Number(Number::from_f64(-123.5505).unwrap());
        assert_eq!("-123.5505", to_nix(&input, 0, 0));
    }

    #[test]
    fn string_empty() {
        assert_eq!(r#""""#, to_nix(&Value::String("".to_string()), 0, 0));
    }

    #[test]
    fn string_simple() {
        let input = Value::String("Hello, world!".to_string());
        let expected = r#""Hello, world!""#;
        assert_eq!(expected, to_nix(&input, 0, 0));
    }

    #[test]
    fn array_empty() {
        let input = Value::Array(vec![]);
        let expected = "[ ]";
        assert_eq!(expected, to_nix(&input, 0, 0));
    }

    #[test]
    fn array_single_bool() {
        let input = Value::Array(vec![Value::Bool(true)]);
        let expected = "[\n  true\n]";
        assert_eq!(expected, to_nix(&input, 0, 2));
    }

    #[test]
    fn array_multiple_bool() {
        let input = json!([true, false]);
        let expected = "[\n  true\n  false\n]";
        assert_eq!(expected, to_nix(&input, 0, 2));
    }

    #[test]
    fn object_empty() {
        let input = Value::Object(Map::new());
        let expected = "{ }";
        assert_eq!(expected, to_nix(&input, 0, 0));
    }

    #[test]
    fn object_single_key() {
        let input = json!({ "key": "value" });
        let expected = "{\n  key = \"value\";\n}";
        assert_eq!(expected, to_nix(&input, 0, 2));
    }

    #[test]
    fn complex_object() {
        let input = json!({
            "a": 123,
            "hello-world": "!",
            "1": 1,
            "obj1": {
                "key1": true,
                "null": null,
                "arr1": [
                    false,
                    "abc",
                    {
                        "xyz1": {
                            "xyz2": {
                                "elemenope": 5.55
                            }
                        }
                    }
                ]
            }
        });
        let expected = r#"
          {
            a = 123;
            hello-world = "!";
            "1" = 1;
            obj1 = {
              key1 = true;
              "null" = null;
              arr1 = [
                false
                "abc"
                {
                  xyz1 = {
                    xyz2 = {
                      elemenope = 5.55;
                    };
                  };
                }
              ];
            };
          }"#;
        assert_eq!(trim_indent(expected), to_nix(&input, 0, 2));
    }

    #[test]
    fn complex_array() {
        let input = json!([
            true,
            false,
            "hello",
            123,
            123.456,
            [
                null,
                {
                    "a_b_c": "abc"
                }
            ]
        ]);
        let expected = r#"
          [
            true
            false
            "hello"
            123
            123.456
            [
              null
              {
                a_b_c = "abc";
              }
            ]
          ]"#;
        assert_eq!(trim_indent(expected), to_nix(&input, 0, 2));
    }
}
