pub fn indent(value: &str, indentation: usize) -> String {
    let mut result = String::with_capacity(value.len() + indentation);

    for _ in 0..indentation {
        result.push(' ');
    }

    result.push_str(value);

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_value_0_indentation() {
        let input = "";
        let expected = "";
        assert_eq!(expected, indent(input, 0));
    }

    #[test]
    fn simple_value_1_indentation() {
        let input = "abc";
        let expected = " abc";
        assert_eq!(expected, indent(input, 1));
    }

    #[test]
    fn already_indented_value_1_indentation() {
        let input = " abc";
        let expected = "  abc";
        assert_eq!(expected, indent(input, 1));
    }

    #[test]
    fn complexer_input_4_indentation() {
        let input = "hello, indentation - 4  #  ";
        let expected = "    hello, indentation - 4  #  ";
        assert_eq!(expected, indent(input, 4));
    }
}
