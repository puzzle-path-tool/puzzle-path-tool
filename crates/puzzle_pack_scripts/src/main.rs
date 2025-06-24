#![allow(dead_code)]
#![allow(unused)]

use std::{error::Error, sync::Arc};

use puzzle_pack_bindings::ts_api::{PuzzptApiExport, examples::Example};
use puzzle_pack_scripts::files::print_file_paths;
use rquickjs::{Context, Module, Object, Runtime, Value, context};
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

fn main() -> Result<(), Box<dyn Error>> {
    print_file_paths();
    main2()
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
function _check_private_redeclaration(obj, privateCollection) {
    if (privateCollection.has(obj)) {
        throw new TypeError("Cannot initialize the same private elements twice on an object");
    }
}
function _class_apply_descriptor_get(receiver, descriptor) {
    if (descriptor.get) {
        return descriptor.get.call(receiver);
    }
    return descriptor.value;
}
function _class_apply_descriptor_set(receiver, descriptor, value) {
    if (descriptor.set) {
        descriptor.set.call(receiver, value);
    } else {
        if (!descriptor.writable) {
            throw new TypeError("attempted to set read only private field");
        }
        descriptor.value = value;
    }
}
function _class_extract_field_descriptor(receiver, privateMap, action) {
    if (!privateMap.has(receiver)) {
        throw new TypeError("attempted to " + action + " private field on non-instance");
    }
    return privateMap.get(receiver);
}
function _class_private_field_get(receiver, privateMap) {
    var descriptor = _class_extract_field_descriptor(receiver, privateMap, "get");
    return _class_apply_descriptor_get(receiver, descriptor);
}
function _class_private_field_init(obj, privateMap, value) {
    _check_private_redeclaration(obj, privateMap);
    privateMap.set(obj, value);
}
function _class_private_field_set(receiver, privateMap, value) {
    var descriptor = _class_extract_field_descriptor(receiver, privateMap, "set");
    _class_apply_descriptor_set(receiver, descriptor, value);
    return value;
}
function _define_property(obj, key, value) {
    if (key in obj) {
        Object.defineProperty(obj, key, {
            value: value,
            enumerable: true,
            configurable: true,
            writable: true
        });
    } else {
        obj[key] = value;
    }
    return obj;
}
class AWrapper {
    static unwrap(value) {
        return value.data;
    }
    static wrap(value) {
        return new AWrapper(value);
    }
    constructor(data){
        _define_property(this, "data", void 0);
        this.data = data;
    }
}
export function doStuff(value) {
    const v = AWrapper.unwrap(value);
//TODO
}
function wrapProxy(base, extra) {
    return new Proxy(base, {
        get (target, prop, receiver) {
            const extraObj = extra(target);
            if (typeof prop === "string" && prop in extraObj) {
                return extraObj[prop];
            }
            return Reflect.get(target, prop, receiver);
        },
        has (target, p) {
            if (Reflect.has(target, p)) {
                return true;
            }
            const extraObj = extra(target);
            return Reflect.has(extraObj, p);
        },
        ownKeys (target) {
            const extraObj = extra(target);
            return Array.from(new Set([
                ...Reflect.ownKeys(target),
                ...Reflect.ownKeys(extraObj)
            ]));
        },
        getOwnPropertyDescriptor (target, p) {
            const extraObj = extra(target);
            if (Reflect.has(extraObj, p)) {
                return Reflect.getOwnPropertyDescriptor(extraObj, p);
            }
            return Reflect.getOwnPropertyDescriptor(target, p);
        }
    });
}
function wrapFields(obj, id) {
    const x = Object.entries(obj).map(([key, value])=>{
        return [
            key,
            {
                id: id,
                value: value
            }
        ];
    });
    return Object.fromEntries(x);
}
var _data = /*#__PURE__*/ new WeakMap();
class BWrapper {
    static unwrap(value) {
        return _class_private_field_get(value, _data);
    }
    static wrap(value) {
        return wrapProxy(new BWrapper(value), (target)=>{
            return wrapFields(_class_private_field_get(target, _data).fields, _class_private_field_get(target, _data).name);
        });
    }
    constructor(data){
        _class_private_field_init(this, _data, {
            writable: true,
            value: void 0
        });
        _define_property(this, "ww", 1);
        _class_private_field_set(this, _data, data);
    }
}
const b = BWrapper.wrap({
    name: "SomeName",
    fields: {
        x: 1,
        y: 2,
        value: 3,
        toString: 4,
        _ee2: 5,
        [1.2e2]: 6
    }
});
/*
Run in Console:

npm run check; node out/packs/core/modules/opaque_test.js

*/ 

export const z11 = b.x.id;
export const z12 = b.y.value;
export const z13 = b;

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
                .with_target(EsVersion::Es2022),
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };

        emitter.emit_module(&module)?;
        let new_code = String::from_utf8(buf)?;
        // println!("{new_code}");
        println!("{code}");
        // let code = new_code;

        let module = Module::declare(ctx.clone(), module_name, code);
        assert!(module.is_ok(), "{:?}", ctx.catch());

        let module = module?;

        let eval_res = module.eval();
        assert!(eval_res.is_ok(), "{:?}", ctx.catch());

        let (module, _promise) = eval_res?;

        let namespace = module.namespace();
        assert!(namespace.is_ok(), "{:?}", ctx.catch());

        let namespace = namespace?;

        let props = namespace.props::<String, Value>();

        for prop in props {
            let Ok((key, value)) = prop else {
                println!("{:?}", ctx.catch());
                continue;
            };
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
