use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;

const BUILT_IN: [&str; 41] = [
    "assert",
    "async_hooks",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "diagnostics_channel",
    "dns",
    "domain",
    "events",
    "fs",
    "http",
    "http2",
    "https",
    "inspector",
    "module",
    "net",
    "os",
    "path",
    "perf_hooks",
    "process",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "timers",
    "tls",
    "trace_events",
    "tty",
    "url",
    "util",
    "v8",
    "vm",
    "wasi",
    "worker_threads",
    "zlib",
];

const NODE_PROTOCOL: &str = "node:";

pub fn remove_first_and_last_chars(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn is_npm_dep(dependency_string: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9@]+").unwrap();
    re.is_match(dependency_string) && !is_built_in_dep(dependency_string)
}

pub fn is_built_in_dep(dependency_string: &str) -> bool {
    let mut module_name = String::from(dependency_string);

    if module_name.starts_with(NODE_PROTOCOL) {
        module_name = module_name.trim_start_matches(NODE_PROTOCOL).to_string();
    }

    if let Some(slash_index) = module_name.find('/') {
        if slash_index != module_name.len() - 1 {
            module_name = module_name[..slash_index].to_string();
        }
    }

    BUILT_IN.contains(&&*module_name)
}

pub fn crop_dep_only(dependency: String) -> String {
    let split_dep: Vec<_> = dependency.split('/').collect();

    match split_dep.len().cmp(&1) {
        Ordering::Equal => String::from(split_dep[0]),
        Ordering::Greater => split_dep[0].to_owned() + "/" + split_dep[1],
        Ordering::Less => String::new(),
    }
}

pub fn filtered_and_cropped_deps(dependencies: HashSet<String>) -> HashSet<String> {
    dependencies
        .into_iter()
        .filter(|dependency| is_npm_dep(dependency))
        .map(crop_dep_only)
        .collect()
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
mod is_npm_dep_tests {
    use crate::ast_browser::utils::is_npm_dep;

    #[test]
    fn is_npm_dep_should_returns_false_with_local_dep_test() {
        let result = is_npm_dep(&String::from("./aah"));
        assert!(!result);
    }

    #[test]
    fn is_npm_dep_should_returns_false_with_local_remote_dep_test() {
        let result = is_npm_dep(&String::from("../../aah"));
        assert!(!result);
    }

    #[test]
    fn is_npm_dep_should_returns_true_with_external_dep_with_at_test() {
        let result = is_npm_dep(&String::from("@angular/core"));
        assert!(result);
    }

    #[test]
    fn is_npm_dep_should_returns_true_with_external_dep_without_at_test() {
        let result = is_npm_dep(&String::from("rxjs"));
        assert!(result);
    }

    #[test]
    fn is_npm_dep_should_returns_false_with_local_dep_with_hashtag_test() {
        let result = is_npm_dep(&String::from("#mymodule"));
        assert!(!result);
    }

    #[test]
    fn is_npm_built_in_without_protocol() {
        let result = is_npm_dep(&String::from("fs"));
        assert!(!result);
    }

    #[test]
    fn is_npm_built_in_with_protocol() {
        let result = is_npm_dep(&String::from("node:child_process"));
        assert!(!result);
    }

    #[test]
    fn is_npm_built_in_without_protocol_slash() {
        let result = is_npm_dep(&String::from("fs/promises"));
        assert!(!result);
    }

    #[test]
    fn is_npm_built_in_with_protocol_slash() {
        let result = is_npm_dep(&String::from("node:fs/promises"));
        assert!(!result);
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
    use crate::ast_browser::utils::filtered_and_cropped_deps;
    use std::collections::HashSet;

    #[test]
    fn should_returns_same_test() {
        let mut base_deps: HashSet<String> = HashSet::new();
        let external_dep: String = String::from("@angular/core");
        base_deps.insert(external_dep);
        let result = filtered_and_cropped_deps(base_deps);
        assert_eq!(result.len(), 1);
        assert!(result.contains("@angular/core"));
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
        assert!(result.contains("@angular/core"));
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
        assert!(result.contains("node"));
        assert!(result.contains("@angular/core"));
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
        assert!(result.contains("node"));
        assert!(result.contains("@angular/core"));
        assert!(result.contains("@something/utils"));
    }
}
