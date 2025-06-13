#![allow(dead_code)]

use std::error::Error;

use rquickjs::{Context, Module, Object, Runtime, Value};

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    ctx.with(|ctx| -> Result<(), Box<dyn Error>> {
        let globals = ctx.globals();

        let lib = Object::new(ctx.clone())?;
        lib.set("val1", 1)?;

        globals.set("lib", lib)?;

        let code = r#"
            export const a: number = 3;

            export const b = "Hello";

            let temp1 = 5;
            temp1++;
            export const c = temp1;

            export const d = lib.val1;
        "#;

        let module = Module::declare(ctx.clone(), "test", code);

        assert!(module.is_ok(), "{:?}", ctx.catch());

        let module = module?;

        let (module, _promise) = module.eval()?;
        let namespace = module.namespace()?;

        let props = namespace.as_object().unwrap().props::<String, Value>();

        for key in props {
            let (key, value) = key?;
            // let val = namespace.get::<_, rquickjs::Value>(&key)?;
            println!("export {key} = {value:?}");
        }

        Ok(())
    })?;

    Ok(())
}
