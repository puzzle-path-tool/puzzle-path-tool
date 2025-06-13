#![allow(dead_code)]

use std::{error::Error, sync::Arc};

use rquickjs::{Context, Module, Object, Runtime, Value};
use swc_common::{source_map::SourceMap, FileName, Globals, Mark, GLOBALS};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{Config, Emitter, text_writer::JsWriter};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::{Fold, FoldWith};

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
        let program = parser.parse_program().unwrap().apply(pass);

        let globals2 = Globals::default();
        GLOBALS.set(&globals2, || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new(); 

            let module = module.apply()
        });
        let visitor = swc_ecma_transforms_typescript::strip_type();
        module.fol

        let mut buf = vec![];

        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);

        let mut emitter = Emitter {
            cfg: Config::default()
                .with_minify(true)
                .with_target(EsVersion::Es5),
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

        let props = namespace.as_object().unwrap().props::<String, Value>();

        for prop in props {
            let (key, value) = prop?;
            println!("export {key} = {value:?}");
        }

        Ok(())
    })?;

    Ok(())
}
