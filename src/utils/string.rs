/// Converts an `Option<String>` to an `Option<&str>`.
#[allow(dead_code)]
pub fn str_option_to_slice(option: &Option<String>) -> Option<&str> {
    option.as_deref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_option_to_slice() {
        let some_string = Some(String::from("test"));
        assert_eq!(str_option_to_slice(&some_string), Some("test"));

        let none_string: Option<String> = None;
        assert_eq!(str_option_to_slice(&none_string), None);
    }
}
