use regex::Regex;

pub fn remove_first_and_last_chars(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn is_internal_dep(dependency_string: &String) -> bool {
    let re = Regex::new(r"^\.+").unwrap();
    re.is_match(dependency_string)
}

#[cfg(test)]
mod utils_tests {
    use crate::ast_browser::utils::{is_internal_dep, remove_first_and_last_chars};

    #[test]
    fn remove_first_and_last_chars_test() {
        let result = remove_first_and_last_chars(String::from("AAA"));
        assert_eq!(result, "A");
    }

    #[test]
    fn is_internal_dep_should_returns_true_with_local_dep_test() {
        let result = is_internal_dep(&String::from("./aah"));
        assert_eq!(result, true);
    }

    #[test]
    fn is_internal_dep_should_returns_true_with_out_dep_test() {
        let result = is_internal_dep(&String::from("../../aah"));
        assert_eq!(result, true);
    }

    #[test]
    fn is_internal_dep_should_returns_false_with_internal_test() {
        let result = is_internal_dep(&String::from("@angular/core"));
        assert_eq!(result, false);
    }
}
