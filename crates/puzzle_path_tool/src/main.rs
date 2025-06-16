#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, sync::Arc};

use puzzle_core::ts_api::{PuzzptApiExport, examples::Example};
use rquickjs::{Context, Module, Object, Runtime, Value};
use serde::Deserialize;
use swc_common::{FileName, GLOBALS, Globals, Mark, source_map::SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{Config, Emitter, text_writer::JsWriter};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::{Fold, FoldWith};

struct File {
    name: Box<str>,
    content: Box<str>,
}

fn transpile(_files: impl IntoIterator<Item = File>) -> Vec<File> {
    todo!()
}

fn run_module(_file: File) {
    todo!()
}

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    ctx.with(|ctx| -> Result<(), Box<dyn Error>> {
        let globals = ctx.globals();

        let lib = Object::new(ctx.clone())?;
        lib.set("val1", 1)?;

        globals.set("lib", lib)?;

        let module_name = "test.ts";
        let code: &'static str = r#"
            export const a: number = 3;

            export const b = "Hello";

            let temp1 = 5;
            temp1++;
            export const c = temp1;

            export const d = lib.val1;

            export const e = 44;
            export default e; 
        "#;

        let filename = Arc::new(FileName::Custom(module_name.into()));
        let cm: Arc<SourceMap> = Arc::default();
        let fm = cm.new_source_file(filename, code);

        let mut parser = Parser::new(
            Syntax::Typescript(TsSyntax {
                ..Default::default()
            }),
            StringInput::from(&*fm),
            None,
        );
        let module = parser.parse_module().unwrap();
        // let program = parser.parse_program().unwrap().apply(pass);

        let globals2 = Globals::default();
        GLOBALS.set(&globals2, || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            // let module = module.apply()
        });
        let visitor = swc_ecma_transforms_typescript::strip_type();
        // module.fol

        let mut buf = vec![];

        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);

        let mut emitter = Emitter {
            cfg: Config::default()
                .with_minify(true)
                .with_target(EsVersion::Es2020),
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };

        emitter.emit_module(&module)?;
        let new_code = String::from_utf8(buf)?;
        println!("{new_code}");
        let code = new_code;

        let module = Module::declare(ctx.clone(), module_name, code);

        assert!(module.is_ok(), "{:?}", ctx.catch());

        let module = module?;

        let (module, _promise) = module.eval()?;
        let namespace = module.namespace()?;

        let props = namespace.props::<String, Value>();

        for prop in props {
            let (key, value) = prop?;
            println!("export {key} = {value:?}");

            let Some(value) = value.as_object() else {
                continue;
            };
            let Ok(value) = value.get::<_, Value>("puzzpt_export") else {
                continue;
            };
            let Some(value) = value.as_object() else {
                continue;
            };
            let Ok(Some(value)) = ctx.json_stringify(value.clone()) else {
                continue;
            };
            let Ok(value) = value.get::<String>() else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(value.as_str()) else {
                continue;
            };
            // Replace with faster native version: https://github.com/DelSkayn/rquickjs/issues/47
            // TODO: error handling

            if Example::has_tag(&value) {
                let Ok(item) = Example::deserialize(value) else {
                    continue;
                };
                println!("load {item:?}");
            }
        }

        Ok(())
    })?;

    Ok(())
}
