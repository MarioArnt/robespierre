use serde_json::{json, to_string_pretty};
use std::fs::write;

const REPORT_FILE_NAME: &str = "robespierre_report.json";

pub fn write_json_report(extraneous: Vec<&String>, implicit: Vec<&String>) {
    let json_to_write = json!({
        "extraneous_dependencies": extraneous,
        "implicit_dependencies": implicit,
    });

    write(REPORT_FILE_NAME, to_string_pretty(&json_to_write).unwrap())
        .expect("TODO: panic message");
}
