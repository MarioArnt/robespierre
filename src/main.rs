#![feature(str_split_remainder)]

mod ast_browser;
mod manifest;
mod manifest_spec;

use std::env;
use std::string::String;

fn main() {
    let project_root = manifest::find_project_root().unwrap();
    let declared_dependencies = manifest::read_manifest_dependencies(project_root.clone());
    const DEFAULT_PATTERN: &str = "**/*.ts";
    let pattern: String = env::var("ROBESPIERRE_SOURCES").unwrap_or(DEFAULT_PATTERN.to_string());
    let actual_imports = ast_browser::resolve_actual_imports(project_root, pattern);
    match declared_dependencies {
        Ok(declared) => {
            let extraneous = declared.difference(&actual_imports);
            let implicit = actual_imports.difference(&declared);
            println!("Extraneous dependencies");
            for dep in extraneous {
                println!("{:?}", dep);
            }
            println!("Implicit dependencies");
            for dep in implicit {
                println!("{:?}", dep);
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
