use std::collections::HashMap;

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::TYPES;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "IdentifierPuzzptApi")]
pub struct Identifier {
    #[serde(rename = "pack")]
    pack: String,
    #[serde(rename = "module")]
    module: Vec<String>,
    #[serde(rename = "name")]
    name: String,
}

#[distributed_slice(TYPES)]
static INDENTIFIER_TYPE: fn() -> String = Identifier::decl;

impl Identifier {
    pub fn get_name(&self) -> String {
        format!("{}_{}_{}", self.pack, self.module.join("-"), self.name)
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "VariableIdentifierPuzzptApi")]
pub struct VariableIdentifier {
    id: i32,
}

#[distributed_slice(TYPES)]
static VARIABLE_INDENTIFIER_TYPE: fn() -> String = VariableIdentifier::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "VariablePuzzptApi")]
pub enum Variable {
    #[serde(rename = "Int")]
    Int(VarValue<i32>),
    #[serde(rename = "Bool")]
    Bool(VarValue<bool>),
    #[serde(rename = "Enum")]
    Enum(VarValue<EnumType>),
    #[serde(rename = "Set")]
    Set(Box<Variable>),
    #[serde(rename = "Obj")]
    Obj(Box<HashMap<String, Variable>>),
}

// #[distributed_slice(TYPES)]
// static VARIABLE_TYPE: fn() -> String = Variable::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "VarValuePuzzptApi")]
pub enum VarValue<T>
where
    T: 'static,
{
    Const(T),
    Ref(VariableIdentifier),
}

#[distributed_slice(TYPES)]
static VAR_VALUE_TYPE: fn() -> String = VarValue::<()>::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub struct EnumType {
    values: Vec<String>,
    value: String,
}
