use std::collections::HashSet;
use std::{path::Path, sync::Arc};
use std::{io::stderr};

use anyhow::{Result};
use glob::glob;

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

#[path = "utils.rs"] mod utils;

fn process_typescript_file(path: String, actual_imports: &mut HashSet<String>) {
    let globals = Globals::new();
    let ast: Result<Program, anyhow::Error> = GLOBALS.set(&globals,|| {
        let cm = Arc::<SourceMap>::default();
        let c = swc::Compiler::new(cm.clone());
        let fm = cm
            .load_file(Path::new(&path))
            .expect("Failed to load typescript source");
        let handler = Handler::with_emitter_writer(Box::new(stderr()), Some(c.cm.clone()));
        let result = c.parse_js(
            fm,
            &handler,
            EsVersion::Es2022,
            Syntax::Typescript(TsConfig::default()),
            swc::config::IsModule::Bool(true),
            None,
        );
        result
    });
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
                                                actual_imports.insert(utils::remove_first_and_last_chars(src.to_string()));
                                            }
                                            None => todo!()
                                        }
                                    }
                                    None => todo!()
                                }
                            }
                            Stmt(_stmt) => todo!()
                        }
                    }
                }
                None => todo!()
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

pub fn resolve_actual_imports(pattern: String) -> HashSet<String> {
    let mut actual_imports = HashSet::new();
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                process_typescript_file(path.display().to_string(), &mut actual_imports);
            },
            Err(e) => println!("{:?}", e),
        }
    }
    actual_imports
}
