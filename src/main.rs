#![feature(str_split_remainder)]

use ::clap::{Parser};
#[derive(Parser)]
#[command(author, version, about, long_about = "Robespierre helps you find extraneous and implicit dependencies in your NPM project")]

struct Args {
    #[arg(short, long, default_value_t = false, help = "Output a JSON report")]
    report: bool,
}

mod ast_browser;
mod manifest;
mod write_report;

use std::env;
use std::string::String;

fn main() {
    let args = Args::parse();
    let project_root = manifest::find_project_root().unwrap();
    let declared_dependencies = manifest::read_manifest_dependencies(project_root.clone());
    const DEFAULT_PATTERN: &str = "**/*.ts";
    let pattern: String = env::var("ROBESPIERRE_SOURCES").unwrap_or(DEFAULT_PATTERN.to_string());
    let actual_imports = ast_browser::resolve_actual_imports(project_root, pattern);
    match declared_dependencies {
        Ok(declared) => {
            let mut extraneous: Vec<_> = declared.difference(&actual_imports).collect();
            let mut implicit: Vec<_> = actual_imports.difference(&declared).collect();
            extraneous.sort();
            implicit.sort();

            if args.report {
                write_report::write_json_report(extraneous.clone(), implicit.clone());
            }

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
