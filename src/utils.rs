use crate::ast_browser::ImportStatement;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

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

    let organization_dependency = Regex::new(r"^@").unwrap();

    match split_dep.len().cmp(&1) {
        Ordering::Equal => String::from(split_dep[0]),
        Ordering::Greater => {
            if organization_dependency.is_match(split_dep[0]) {
                split_dep[0].to_owned() + "/" + split_dep[1]
            } else {
                split_dep[0].to_owned()
            }
        }
        Ordering::Less => String::new(),
    }
}

pub fn filtered_and_cropped_deps(dependencies: &mut HashMap<String, ImportStatement>) {
    let mut keys_to_remove = Vec::new();
    let mut items_to_add: HashMap<String, ImportStatement> = HashMap::new();

    for (key, dependency) in &mut *dependencies {
        if !is_npm_dep(&dependency.name) {
            keys_to_remove.push(key.clone());
        } else {
            let new_name = crop_dep_only(dependency.name.clone());
            if new_name != dependency.name {
                keys_to_remove.push(key.clone());
                items_to_add.insert(new_name.clone(), dependency.clone());
            }
        }
    }
    for key in keys_to_remove {
        dependencies.remove(&key);
    }
    for (key, dependency) in items_to_add {
        dependencies.insert(key, dependency.clone());
    }
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
    fn namespace_dep_test_with_organization() {
        let result = crop_dep_only(String::from("@angular/core"));
        assert_eq!(result, "@angular/core");
    }

    #[test]
    fn nested_dep_test_with_organization() {
        let result = crop_dep_only(String::from("@angular/core/something"));
        assert_eq!(result, "@angular/core");
    }

    #[test]
    fn nested_dep_test_without_organization() {
        let result = crop_dep_only(String::from("rxjs/internal"));
        assert_eq!(result, "rxjs");
    }
}

#[cfg(test)]
mod filtered_and_cropped_deps_tests {
    use crate::ast_browser::utils::filtered_and_cropped_deps;
    use crate::ast_browser::ImportStatement;
    use std::collections::HashMap;

    fn fake_dep(name: String) -> ImportStatement {
        ImportStatement {
            name: name.clone(),
            file: String::from("@angular/core"),
            line: 42,
        }
    }

    #[test]
    fn should_returns_same_test() {
        let mut base_deps: HashMap<String, ImportStatement> = HashMap::new();
        let external_dep: String = String::from("@angular/core");
        base_deps.insert(external_dep.clone(), fake_dep(external_dep));

        filtered_and_cropped_deps(&mut base_deps);

        assert_eq!(base_deps.len(), 1);
        assert!(base_deps.contains_key("@angular/core"));
    }

    #[test]
    fn should_returns_filtered_test() {
        let mut base_deps: HashMap<String, ImportStatement> = HashMap::new();
        let external_dep: String = String::from("@angular/core");
        let internal_dep: String = String::from("./aah");
        base_deps.insert(external_dep.clone(), fake_dep(external_dep));
        base_deps.insert(internal_dep.clone(), fake_dep(internal_dep));

        filtered_and_cropped_deps(&mut base_deps);

        assert_eq!(base_deps.len(), 1);
        assert!(base_deps.contains_key("@angular/core"));
    }

    #[test]
    fn should_returns_cropped_test() {
        let mut base_deps: HashMap<String, ImportStatement> = HashMap::new();
        let simple_external_dep: String = String::from("rxjs");
        let nested_external_dep: String = String::from("@angular/core/truc");
        base_deps.insert(simple_external_dep.clone(), fake_dep(simple_external_dep));
        base_deps.insert(nested_external_dep.clone(), fake_dep(nested_external_dep));
        filtered_and_cropped_deps(&mut base_deps);

        assert_eq!(base_deps.len(), 2);
        assert!(base_deps.contains_key("rxjs"));
        assert!(base_deps.contains_key("@angular/core"));
    }

    #[test]
    fn should_returns_filtered_and_cropped_test() {
        let mut base_deps: HashMap<String, ImportStatement> = HashMap::new();
        let simple_external_dep: String = String::from("node");
        let nested_external_dep: String = String::from("@angular/core/truc");
        let internal_dep: String = String::from("./aah");
        let namespace_external_dep: String = String::from("@something/utils");

        base_deps.insert(simple_external_dep.clone(), fake_dep(simple_external_dep));
        base_deps.insert(nested_external_dep.clone(), fake_dep(nested_external_dep));
        base_deps.insert(internal_dep.clone(), fake_dep(internal_dep));
        base_deps.insert(
            namespace_external_dep.clone(),
            fake_dep(namespace_external_dep),
        );

        filtered_and_cropped_deps(&mut base_deps);

        assert_eq!(base_deps.len(), 3);

        assert!(base_deps.contains_key("node"));
        assert!(base_deps.contains_key("@angular/core"));
        assert!(base_deps.contains_key("@something/utils"));
    }
}
