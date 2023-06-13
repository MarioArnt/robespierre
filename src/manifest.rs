use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

use anyhow::{Result,Ok};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Manifest {
    dependencies: HashMap<String, String>,
}

pub fn read_manifest() -> Result<HashSet<String>> {
    let manifest = "package.json";
    let raw = fs::read_to_string(manifest)
        .expect("Should have been able to read the manifest");
    let manifest: Manifest = serde_json::from_str(&raw)
        .expect("Cannot parse manifest");
    return Ok(manifest.dependencies.keys().cloned().collect())
}
