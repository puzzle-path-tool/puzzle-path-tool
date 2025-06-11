#![allow(dead_code)]

use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder,
    jsutils::{JsError, Script, ScriptPreProcessor},
    typescript::{TargetVersion, TypeScriptTranspiler},
    values::JsValueFacade,
};

#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct TypeScriptPreProcessor {
    transpiler: TypeScriptTranspiler,
}

impl TypeScriptPreProcessor {
    fn new() -> Self {
        Self {
            transpiler: TypeScriptTranspiler::new(TargetVersion::Es2022, false, false, false),
        }
    }
}

impl ScriptPreProcessor for TypeScriptPreProcessor {
    fn process(&self, script: &mut Script) -> Result<(), JsError> {
        self.transpiler.transpile_script(script)
    }
}

fn do_stuff() -> Result<JsValueFacade, JsError> {
    let rt = QuickJsRuntimeBuilder::new()
        .script_pre_processor(TypeScriptPreProcessor::new())
        .build();

    let filename = "script.ts";
    let code = r"
        const x: number = 1
    ";
    rt.eval_module_sync(None, Script::new(filename, code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            format!({:?}, do_stuff()),
            format!({:?}, Ok(JsValueFacade::Undefined))
        )
    }
}
