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

pub fn crop_dep_only(dependency: String) -> String {
    let split_dep: Vec<_> = dependency.split("/").collect();

    if split_dep.len() == 1 {
        String::from(split_dep[0])
    } else if split_dep.len() > 1 {
        split_dep[0].to_owned() + "/" + split_dep[1]
    } else {
        String::new()
    }
}

pub fn filtered_and_cropped_deps(dependencies: HashSet<String>) -> HashSet<String> {
    let filtered_deps = dependencies.into_iter()
        .filter(|dependency| !is_internal_dep(dependency))
        .map(|dependency| crop_dep_only(dependency))
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
mod crop_dep_only_tests {
    use crate::ast_browser::utils::crop_dep_only;

    #[test]
    fn simple_dep_test() {
        let result = crop_dep_only(String::from("@angular"));
        assert_eq!(result, "@angular");
    }

    #[test]
    fn namespace_dep_test() {
        let result = crop_dep_only(String::from("@angular/core"));
        assert_eq!(result, "@angular/core");
    }

    #[test]
    fn nested_dep_test() {
        let result = crop_dep_only(String::from("@angular/core/something"));
        assert_eq!(result, "@angular/core");
    }
}

#[cfg(test)]
mod filtered_and_cropped_deps_tests {
    use std::collections::HashSet;
    use crate::ast_browser::utils::filtered_and_cropped_deps;

    #[test]
    fn should_returns_same_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let external_dep: String = String::from("@angular/core");
        base_deps.insert(external_dep);
        let result = filtered_and_cropped_deps(base_deps);
        assert_eq!(result.len(), 1);
        assert_eq!(result.contains("@angular/core"), true);
    }

    #[test]
    fn should_returns_filtered_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let external_dep: String = String::from("@angular/core");
        let internal_dep: String = String::from("./aah");
        base_deps.insert(external_dep);
        base_deps.insert(internal_dep);
        let result = filtered_and_cropped_deps(base_deps);
        assert_eq!(result.len(), 1);
        assert_eq!(result.contains("@angular/core"), true);
    }

    #[test]
    fn should_returns_cropped_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let simple_external_dep: String = String::from("node");
        let nested_external_dep: String = String::from("@angular/core/truc");
        base_deps.insert(simple_external_dep);
        base_deps.insert(nested_external_dep);
        let result = filtered_and_cropped_deps(base_deps);
        assert_eq!(result.len(), 2);
        assert_eq!(result.contains("node"), true);
        assert_eq!(result.contains("@angular/core"), true);
    }

    #[test]
    fn should_returns_filtered_and_cropped_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let simple_external_dep: String = String::from("node");
        let nested_external_dep: String = String::from("@angular/core/truc");
        let internal_dep: String = String::from("./aah");
        let namespace_external_dep: String = String::from("@something/utils");

        base_deps.insert(simple_external_dep);
        base_deps.insert(nested_external_dep);
        base_deps.insert(internal_dep);
        base_deps.insert(namespace_external_dep);

        let result = filtered_and_cropped_deps(base_deps);
        assert_eq!(result.len(), 3);
        assert_eq!(result.contains("node"), true);
        assert_eq!(result.contains("@angular/core"), true);
        assert_eq!(result.contains("@something/utils"), true);
    }
}