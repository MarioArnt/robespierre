mod manifest;
mod ast_browser;

use std::env;
use std::string::String;

fn main() {
    let declared_dependencies = manifest::read_manifest();
    let default_pattern: String = "**/*.ts".to_string();
    let pattern: String = env::var("ROBESPIERRE_SOURCES").unwrap_or(default_pattern);
    let actual_imports = ast_browser::resolve_actual_imports(pattern);
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
        Err(err) => println!("{:?}", err)
    }
}
