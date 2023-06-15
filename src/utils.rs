use std::collections::HashSet;
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

pub fn filtered_internal_deps(dependencies: HashSet<String>) -> HashSet<String> {
    let filtered_deps = dependencies.into_iter()
            .filter(|dependency| !is_internal_dep(dependency))
            .collect();
    filtered_deps
}

#[cfg(test)]
mod remove_first_and_last_chars_tests {
    use crate::ast_browser::utils::remove_first_and_last_chars;

    #[test]
    fn remove_first_and_last_chars_test() {
        let result = remove_first_and_last_chars(String::from("AAA"));
        assert_eq!(result, "A");
    }
}

#[cfg(test)]
mod is_internal_dep_tests {
    use crate::ast_browser::utils::{is_internal_dep};

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

#[cfg(test)]
mod filtered_internal_deps_tests {
    use std::collections::HashSet;
    use crate::ast_browser::utils::filtered_internal_deps;

    #[test]
    fn filter_internal_deps_should_returns_same_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let external_dep: String = String::from("@angular/core");
        base_deps.insert(external_dep);
        let result = filtered_internal_deps(base_deps);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn filter_internal_deps_should_returns_filtered_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let external_dep: String = String::from("@angular/core");
        let internal_dep: String = String::from("./aah");
        base_deps.insert(external_dep);
        base_deps.insert(internal_dep);
        let result = filtered_internal_deps(base_deps);
        assert_eq!(result.len(), 1);
    }
}