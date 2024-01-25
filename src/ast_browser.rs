use std::collections::HashMap;
use std::io::stderr;
use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use anyhow::Result;
use glob::glob;
use log::{debug, error};

use crate::ast_browser::utils::filtered_and_cropped_deps;
use swc_common::{errors::Handler, SourceMap};
use swc_common::{Globals, GLOBALS};
use swc_ecma_ast::EsVersion;
use swc_ecma_ast::ModuleItem::{ModuleDecl, Stmt};
use swc_ecma_ast::Program;
use swc_ecma_parser::Syntax;
use swc_ecma_parser::TsConfig;

#[path = "utils.rs"]
mod utils;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ImportStatement {
    name: String,
    pub file: String,
    pub line: usize,
}

fn process_typescript_file(path: String, actual_imports: &mut HashMap<String, ImportStatement>) {
    let globals = Globals::new();
    let source_map = Arc::<SourceMap>::default();
    let ast: Result<Program, anyhow::Error> = GLOBALS.set(&globals, || {
        let compiler = swc::Compiler::new(source_map.clone());
        let file_manager = source_map
            .load_file(Path::new(&path))
            .expect("Failed to load typescript source");
        let handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));

        compiler.parse_js(
            file_manager,
            &handler,
            EsVersion::Es2022,
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..TsConfig::default()
            }),
            swc::config::IsModule::Bool(true),
            None,
        )
    });
    debug!("Analyzing path: {:?}", path);
    let mut file_imports: HashMap<String, ImportStatement> = HashMap::new();
    let mut is_test_file = false;
    match ast {
        Ok(tree) => {
            if let Some(module) = tree.module() {
                for item in module.body {
                    match item {
                        ModuleDecl(decl) => {
                            let import = decl.as_import();
                            if let Some(import_declaration) = import {
                                if !&import_declaration.type_only {
                                    match &import_declaration.src.raw {
                                        Some(src) => {
                                            let name =
                                                utils::remove_first_and_last_chars(src.to_string());
                                            let actual_import = ImportStatement {
                                                name: name.clone(),
                                                file: path.clone(),
                                                line: source_map
                                                    .lookup_char_pos(import_declaration.span.lo)
                                                    .line,
                                            };
                                            file_imports.insert(name, actual_import);
                                        }
                                        None => (),
                                    }
                                }
                            }
                        }
                        Stmt(stmt) => {
                            if let Some(expr) = stmt.as_expr() {
                                if let Some(call_expr) = expr.expr.as_call() {
                                    if let Some(callee_expr) = call_expr.callee.as_expr() {
                                        if let Some(ident) = callee_expr.as_ident() {
                                            if ident.sym.to_lowercase() == "describe"
                                                || ident.sym.to_lowercase() == "it"
                                                || ident.sym.to_lowercase() == "test"
                                            {
                                                is_test_file = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => error!("{:?}", e),
    }
    if !is_test_file {
        actual_imports.extend(file_imports);
    }
}

pub fn resolve_actual_imports(
    project_root: PathBuf,
    pattern: String,
) -> HashMap<String, ImportStatement> {
    let mut actual_imports: HashMap<String, ImportStatement> = HashMap::new();
    let absolute_pattern = Path::new(&project_root).join(pattern);
    for entry in glob(&absolute_pattern.display().to_string()).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let is_node_module = path
                    .components()
                    .any(|component| component.as_os_str() == "node_modules");
                let file_name = path
                    .components()
                    .last()
                    .expect("Fatal: cannot resolve file name")
                    .as_os_str()
                    .to_str()
                    .expect("Fatal: cannot resolve extension");
                let d_ts_string = ".d.ts";
                let is_d_ts = if file_name.len() > d_ts_string.len() {
                    &file_name[file_name.len() - d_ts_string.len()..] == d_ts_string
                } else {
                    false
                };
                if !is_node_module && !is_d_ts {
                    process_typescript_file(path.display().to_string(), &mut actual_imports);
                }
            }
            Err(e) => error!("{:?}", e),
        }
    }
    filtered_and_cropped_deps(&mut actual_imports);
    actual_imports
}
