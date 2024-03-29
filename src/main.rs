use ::clap::Parser;
use std::collections::HashMap;
#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Robespierre helps you find extraneous and implicit dependencies in your NPM project"
)]

struct Args {
    #[arg(short, long, default_value_t = false, help = "Output a JSON report")]
    report: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Display output in JSON format"
    )]
    json: bool,
    #[arg(short, long, default_value_t = false, help = "Verbose mode")]
    verbose: bool,
}

mod ast_browser;
mod manifest;
mod write_report;

use crate::ast_browser::ImportStatement;
use ansi_term::Style;
use env_logger::{Builder, Target};
use log::{error, info};
use serde_json::{json, to_string_pretty};
use std::env;
use std::io::Write;
use std::string::String;

fn main() {
    let args = Args::parse();

    configure_logging(&args);

    let project_root = manifest::find_project_root().unwrap();
    let declared_dependencies = manifest::read_manifest_dependencies(project_root.clone());

    const DEFAULT_PATTERN: &str = "**/*.ts";
    let pattern: String = env::var("ROBESPIERRE_SOURCES").unwrap_or(DEFAULT_PATTERN.to_string());

    let actual_imports_map = ast_browser::resolve_actual_imports(project_root, pattern);
    let actual_imports = actual_imports_map.keys().cloned().collect();

    match declared_dependencies {
        Ok(declared) => {
            let mut extraneous: Vec<_> = declared.difference(&actual_imports).collect();
            let mut implicit: Vec<_> = actual_imports.difference(&declared).collect();
            extraneous.sort();
            implicit.sort();

            if args.report {
                write_report::write_json_report(extraneous.clone(), implicit.clone());
            }

            if args.json {
                print_result_in_json_format(extraneous, implicit)
            } else {
                print_result(actual_imports_map, extraneous, implicit);
            }
        }
        Err(err) => error!("{:?}", err),
    }
}

fn print_result_in_json_format(extraneous: Vec<&String>, implicit: Vec<&String>) {
    let json_output = json!({
        "extraneous_dependencies": extraneous,
        "implicit_dependencies": implicit,
    });
    let json_to_print = to_string_pretty(&json_output).unwrap();
    info!("{}", json_to_print);
}

fn print_result(
    actual_imports_map: HashMap<String, ImportStatement>,
    extraneous: Vec<&String>,
    implicit: Vec<&String>,
) {
    info!("{}", Style::new().bold().paint("Extraneous dependencies:"));
    for dep in extraneous {
        info!("├── {}", Style::new().underline().paint(dep));
    }
    info!("{}", Style::new().bold().paint("Implicit dependencies:"));
    for dep in implicit {
        let details = actual_imports_map.get(dep).unwrap();
        info!("├── {}", Style::new().underline().paint(dep));
        info!("│   └── file://{}:{}", details.file, details.line);
    }
}

fn configure_logging(args: &Args) {
    let mut logging_builder = Builder::from_default_env();

    logging_builder.format(|buf, record| writeln!(buf, "{}", record.args()));

    if args.verbose {
        logging_builder.filter_level(log::LevelFilter::Debug);
    } else {
        logging_builder.filter_level(log::LevelFilter::Info);
    }
    logging_builder.target(Target::Stdout);
    logging_builder.init();
}
