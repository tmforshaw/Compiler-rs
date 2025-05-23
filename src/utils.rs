const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    // Don't allow starting numbers for identifier
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("Expected identifier".to_string())
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_careful(|c| c.is_ascii_digit(), s, "Expected digits".to_string())
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_whitespace_non_empty(s: &str) -> Result<(&str, &str), String> {
    take_while_careful(
        |c| WHITESPACE.contains(&c),
        s,
        "Expected whitespace".to_string(),
    )
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| (!accept(c)).then_some(idx))
        .unwrap_or(s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn take_while_careful(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_message: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if !extracted.is_empty() {
        Ok((remainder, extracted))
    } else {
        Err(error_message)
    }
}

pub(crate) fn tag<'a>(starting_text: &str, s: &'a str) -> Result<&'a str, String> {
    if let Some(s) = s.strip_prefix(starting_text) {
        Ok(s)
    } else {
        Err(format!("Expected \"{starting_text}\""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("Expected digits".to_string()));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("abcdEFG stop"), Ok((" stop", "abcdEFG")));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar1()"), Ok(("()", "foobar1")));
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(
            extract_ident("123abc"),
            Err("Expected identifier".to_string()),
        );
    }

    #[test]
    fn do_not_extract_spaces1_when_input_does_not_start_with_them() {
        assert_eq!(
            extract_whitespace_non_empty("blah"),
            Err("Expected whitespace".to_string()),
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }

    #[test]
    fn extract_newlines_or_spaces() {
        assert_eq!(extract_whitespace(" \n   \n\nabc"), ("abc", " \n   \n\n"));
    }
}
