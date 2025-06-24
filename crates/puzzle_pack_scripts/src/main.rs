#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, sync::Arc};

use puzzle_pack_bindings::ts_api::{PuzzptApiExport, examples::Example};
use puzzle_pack_scripts::files::print_file_paths;
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

fn main() {
    print_file_paths();
    main2();
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::too_many_lines)]
fn main2() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    ctx.with(|ctx| -> Result<(), Box<dyn Error>> {
        let globals = ctx.globals();

        let lib = Object::new(ctx.clone())?;
        lib.set("val1", 1)?;

        globals.set("lib", lib)?;

        let module_name = "test.ts";
        let code22: &'static str = r#"
            export const a = 3;

            export const b = "Hello";

            let temp1 = 5;
            temp1++;
            export const c = temp1;

            export const d = lib.val1;

            export const e = 44;
            export default e; 
            export const r1 = {
                puzzpt_export: {
                    export_tag: "ExamplePuzzptApi",
                    value: "HEY",
                    number: 1,
                    stuff: { tag: "StuffA", value: 1 },
                }
            };

            class Example {
                constructor() {
                    this.puzzpt_export = {
                        export_tag: "ExamplePuzzptApi",
                        value: "Another",
                        number: 33,
                        stuff: { tag: "StuffC" },
                    };
                }
            }

            export const r2 = new Example();
        "#;
        let code = r#"// Opaque Data test

                                             
class AWrapper {
                     data       ;
    constructor(data       ) {
        this.data = data;
    }
    static unwrap(value          )        {
        return value.data;
    }
    static wrap(value       )           {
        return new AWrapper(value);
    }
}
                         

export function doStuff(value   ) {
    const v = AWrapper.unwrap(value);

    //TODO
}

// Opaque Data test 2

                                          

                                                               
                                      
                                                
  

function wrapProxy                                                 (
    base       ,
    extra                           ,
)                 {
    return new Proxy(base, {
        get(target, prop, receiver) {
            const extraObj = extra(target);

            if (typeof prop === "string" && prop in extraObj) {
                return extraObj[prop];
            }
            return Reflect.get(target, prop, receiver);
        },
        has(target, p) {
            if (Reflect.has(target, p)) {
                return true;
            }

            const extraObj = extra(target);

            return Reflect.has(extraObj, p);
        },
        ownKeys(target) {
            const extraObj = extra(target);

            return Array.from(
                new Set([
                    ...Reflect.ownKeys(target),
                    ...Reflect.ownKeys(extraObj),
                ]),
            );
        },
        getOwnPropertyDescriptor(target, p) {
            const extraObj = extra(target);

            if (Reflect.has(extraObj, p)) {
                return Reflect.getOwnPropertyDescriptor(extraObj, p);
            }
            return Reflect.getOwnPropertyDescriptor(target, p);
        },
    })                  ;
}

function wrapFields                            (
    obj   ,
    id        ,
)             {
    const x = Object.entries(obj).map(([key, value]) => {
        return [key, { id: id, value: value }];
    });
    return Object.fromEntries(x);
}

class BWrapper                       {
             #data          ;
             ww         = 1;
            constructor(data          ) {
        this.#data = data;
    }
    static unwrap                      (value      )           {
        return value.#data;
    }
    static wrap                            (value          )       {
        return wrapProxy(new BWrapper(value), (target) => {
            return wrapFields(target.#data.fields, target.#data.name);
        });
    }
}
                                                               

export const b = BWrapper.wrap({
    name: "SomeName",
    fields: {
        x: 1,
        y: 2,
        value: 3,
        toString: 4,
        _ee2: 5,
        [1.2e2]: 6,
    },
});

// console.log({ ...b });
// console.log(b.x.id);
// console.log(b.x.value);

export const a11 = b.x.id
export const a12 = b.x.value
export const a13 = b.x

/*
Run in Console:

npm run check; node out/packs/core/modules/opaque_test.js

*/
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

        let eval_res = module.eval();
        assert!(eval_res.is_ok(), "{:?}", ctx.catch());

        let (module, _promise) = eval_res?;

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
            let Ok(value) = rquickjs_serde::from_value::<serde_json::Value>(value) else {
                println!("Err: failed to deserialize export as json");
                //TODO: Err: failed to deserialize export as json
                continue;
            };

            if Example::has_tag(&value) {
                let Ok(item) = Example::deserialize(value) else {
                    //TODO: Err: failed to deserialize export as Example
                    println!("Err: failed to deserialize export as Example");
                    continue;
                };
                println!("load {item:?}");
            }
        }

        Ok(())
    })?;

    Ok(())
}
