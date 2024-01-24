use log::info;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::option::Option;
use std::path::{Path, PathBuf};

use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};
use std::result::Result::Ok as StdOk;

#[derive(Serialize, Deserialize)]
struct Manifest {
    dependencies: Option<HashMap<String, String>>,
}

pub fn find_project_root() -> Result<PathBuf> {
    match env::current_dir() {
        StdOk(current_dir) => {
            let cwd = current_dir.into_os_string();
            let project_root_result = find_closest_parent_manifest(Path::new(&cwd));
            match project_root_result {
                Some(project_root) => Ok(project_root.to_path_buf()),
                None => bail!("Manifest file cannot be found, make sure you are running this command in a valid NPM project")
            }
        }
        Err(_) => bail!("Fatal: unable to resolve current working directory"),
    }
}

fn find_closest_parent_manifest(path: &Path) -> Option<&Path> {
    if Path::new(path).join("package.json").exists() {
        return Some(path);
    }
    return match path.parent() {
        Some(parent) => find_closest_parent_manifest(parent),
        None => None,
    };
}

pub fn read_manifest_dependencies(project_root: PathBuf) -> Result<HashSet<String>> {
    let manifest_path = project_root.join("package.json");
    info!("Found manifest path at {}", manifest_path.display());
    let raw =
        fs::read_to_string(manifest_path).expect("Should have been able to read the manifest");
    let manifest: Manifest = serde_json::from_str(&raw).expect("Cannot parse manifest");
    let dependencies = manifest.dependencies;
    return match dependencies {
        Some(deps) => Ok(deps.keys().cloned().collect()),
        None => Ok(HashSet::new()),
    };
}
