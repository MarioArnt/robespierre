use std::collections::HashSet;
use std::{path::Path, sync::Arc};
use std::{io::stderr};
use std::path::PathBuf;

use anyhow::{Result};
use glob::glob;
use swc::atoms::Atom;

use swc_ecma_ast::EsVersion;
use swc_common::{
    errors::{Handler},
    SourceMap,
};
use swc_ecma_parser::{TsConfig};
use swc_ecma_parser::Syntax;
use swc_common::{GLOBALS, Globals};
use swc_ecma_ast::Program;
use swc_ecma_ast::ModuleItem::{Stmt,ModuleDecl};
use crate::ast_browser::utils::filtered_and_cropped_deps;

#[path = "utils.rs"] mod utils;

fn process_typescript_file(path: String, actual_imports: &mut HashSet<String>) {
    let globals = Globals::new();
    let ast: Result<Program, anyhow::Error> = GLOBALS.set(&globals,|| {
        let source_map = Arc::<SourceMap>::default();
        let compiler = swc::Compiler::new(source_map.clone());
        let file_manager = source_map
            .load_file(Path::new(&path))
            .expect("Failed to load typescript source");
        let handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));
        let result = compiler.parse_js(
            file_manager,
            &handler,
            EsVersion::Es2022,
            Syntax::Typescript(TsConfig::default()),
            swc::config::IsModule::Bool(true),
            None,
        );
        result
    });
    println!("Analyzing path: {:?}", path);
    let mut file_imports: HashSet<String> = HashSet::new();
    let mut is_test_file = false;
    match ast {
        Ok(tree) => {
            match tree.module() {
                Some(module) => {
                    for item in module.body {
                        match item {
                            ModuleDecl(decl) => {
                                let import = decl.as_import();
                                match import {
                                    Some(i) => {
                                        match &i.src.raw {
                                            Some(src) => {
                                                file_imports.insert(utils::remove_first_and_last_chars(src.to_string()));
                                            }
                                            None => ()
                                        }
                                    }
                                    None => ()
                                }
                            }
                            Stmt(stmt) => {
                                match stmt.as_expr() {
                                    Some(expr) => {
                                        match expr.expr.as_call() {
                                            Some(call_expr) => {
                                                match call_expr.callee.as_expr() {
                                                    Some(callee_expr) => {
                                                        match callee_expr.as_ident() {
                                                            Some(ident) => {

                                                                if ident.sym.to_lowercase() == "describe" || ident.sym.to_lowercase() == "it" || ident.sym.to_lowercase() == "test" {
                                                                    is_test_file = true;
                                                                }
                                                            },
                                                            None => (),
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            None => (),
                                        }
                                    },
                                    None => (),
                                }
                            }
                        }
                    }
                }
                None => ()
            }
        },
        Err(e) => println!("{:?}", e),
    }
    if !is_test_file {
        actual_imports.extend(file_imports);
    }
}

pub fn resolve_actual_imports(project_root: PathBuf, pattern: String) -> HashSet<String> {
    let mut actual_imports = HashSet::new();
    let absolute_pattern = Path::new(&project_root).join(pattern);
    for entry in glob(&absolute_pattern.display().to_string()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let is_node_module = path.components().any(|component| component.as_os_str() == "node_modules");
                let file_name = path.components().last().expect("Fatal: cannot resolve file name").as_os_str().to_str().expect("Fatal: cannot resolve extension");
                let is_d_ts = &file_name[file_name.len()-5..] == ".d.ts";
                if !is_node_module && !is_d_ts {
                    process_typescript_file(path.display().to_string(), &mut actual_imports);
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
    filtered_and_cropped_deps(actual_imports)
}
