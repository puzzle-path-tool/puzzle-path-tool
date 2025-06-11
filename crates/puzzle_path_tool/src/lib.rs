use std::sync::Arc;

use mlua::AsChunk;
use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder,
    jsutils::{JsError, Script, ScriptPreProcessor},
    typescript::{TargetVersion, TypeScriptTranspiler}, values::JsValueFacade,
};

#[must_use]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn do_stuff() -> Result<JsValueFacade, JsError> {
    let transpiler = TypeScriptTranspiler::new(TargetVersion::Es2022, false, false, false);
    let rt = QuickJsRuntimeBuilder::new()
        .script_pre_processor(transpiler)
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
        assert!(do_stuff(), Some(JsValueFacade::Undefined))
    }
}
