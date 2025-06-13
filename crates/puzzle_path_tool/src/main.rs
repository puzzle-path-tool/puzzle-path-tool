#![allow(dead_code)]

use std::error::Error;

use rquickjs::{Context, Module, Runtime, Value};

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;

    ctx.with(|ctx| -> Result<(), Box<dyn Error>> {
        let module = Module::declare(
            ctx,
            "test",
            r#"
            export const x = 3;
            export const y = "Hello";
        "#,
        )?;

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
