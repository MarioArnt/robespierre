use std::collections::HashSet;
use std::collections::HashMap;
use std::option::Option;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::result::Result::{Ok as StdOk};

#[derive(Serialize, Deserialize)]
struct Manifest {
    dependencies: Option<HashMap<String, String>>,
}

fn find_closest_parent_manifest(path: &Path) -> Option<PathBuf> {
    if Path::new(path).join("package.json").exists() {
        let manifest = Path::new(path).join("package.json");
        return Some(manifest);
    }
    return match path.parent() {
        Some(parent) => {
            find_closest_parent_manifest(&parent)
        }
        None => None
    }
}

pub fn read_manifest_dependencies() -> Result<HashSet<String>> {
    return match env::current_dir() {
        StdOk(current_dir) => {
            let manifest = find_closest_parent_manifest(&Path::new(&current_dir.into_os_string()));
            match manifest {
                Some(manifest_path) => {
                    println!("Found manifest path at {}", manifest_path.display());
                    let raw = fs::read_to_string(manifest_path)
                        .expect("Should have been able to read the manifest");
                    let manifest: Manifest = serde_json::from_str(&raw)
                        .expect("Cannot parse manifest");
                    let dependencies = manifest.dependencies;
                    let empty_hash_set: HashSet<String> = HashSet::new();
                    match dependencies {
                        Some(deps) => Ok(deps.keys().cloned().collect()),
                        None => Ok(empty_hash_set),
                    }
                }
                None => bail!("Manifest file cannot be found, make sure you are running this command in a valid NPM project")
            }
        },
        Err(_) => bail!("Fatal: unable to resolve current working directory")
    }
}
