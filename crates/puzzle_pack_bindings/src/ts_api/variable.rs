use std::collections::HashMap;

use linkme::distributed_slice;
use serde::{Deserialize, Serialize, de::value};
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

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "VariableIdentifierPuzzptApi")]
#[serde(transparent)]
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
    Int(i32),
    #[serde(rename = "Bool")]
    Bool(bool),
    #[serde(rename = "Enum")]
    Enum { values: Vec<String>, value: String },
    #[serde(rename = "Set")]
    Set(Box<Variable>),
    #[serde(rename = "Obj")]
    Obj(Box<HashMap<String, Variable>>),
    #[serde(rename = "Ref")]
    Ref(VariableIdentifier),
}

#[distributed_slice(TYPES)]
static VARIABLE_TYPE: fn() -> String = Variable::decl;
