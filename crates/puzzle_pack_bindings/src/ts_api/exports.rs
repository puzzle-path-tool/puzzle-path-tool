use std::collections::HashMap;

use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::variable::Identifier;
use crate::ts_api::{PuzzptApiExport, TYPES};

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "DeductionPuzzptApi")]
pub struct Deduction {
    #[serde(rename = "name")]
    name: Identifier,
    data: DeductionData,
}

#[distributed_slice(TYPES)]
static DEDUCTION_TYPE: fn() -> String = Deduction::decl;

impl PuzzptApiExport for Deduction {
    fn tag_value() -> &'static str {
        "DeductionPuzzptApi"
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "DeductionDataPuzzptApi")]
pub struct DeductionData {
    #[serde(rename = "fieldType")]
    field_type: FieldType,
}

#[distributed_slice(TYPES)]
static DEDUCTION_DATA_TYPE: fn() -> String = DeductionData::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "FieldTypePuzzptApi")]
enum FieldType {
    #[serde(rename = "Int")]
    Int,
    #[serde(rename = "Bool")]
    Bool,
    #[serde(rename = "Enum")]
    Enum(Vec<String>),
    #[serde(rename = "Set")]
    Set(Box<FieldType>),
    #[serde(rename = "Obj")]
    Obj(HashMap<String, FieldType>),
}

#[distributed_slice(TYPES)]
static FIELD_TYPE_TYPE: fn() -> String = FieldType::decl;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "RulePuzzptApi")]
pub struct Rule {
    #[serde(rename = "name")]
    name: Identifier,
    #[serde(rename = "data")]
    data: DeductionData,
}

#[distributed_slice(TYPES)]
static RULE_TYPE: fn() -> String = Rule::decl;

impl PuzzptApiExport for Rule {
    fn tag_value() -> &'static str {
        "RulePuzzptApi"
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "LogicStepPuzzptApi")]
pub struct LogicStep {
    #[serde(rename = "name")]
    name: Identifier,
}

#[distributed_slice(TYPES)]
static LOGIC_STEP_TYPE: fn() -> String = LogicStep::decl;

impl PuzzptApiExport for LogicStep {
    fn tag_value() -> &'static str {
        "LogicStepPuzzptApi"
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "PackPuzzptApi")]
pub struct Pack {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "authors")]
    authors: Vec<String>,
    #[serde(rename = "description")]
    description: String,
}

#[distributed_slice(TYPES)]
static PACK_TYPE: fn() -> String = Pack::decl;

impl PuzzptApiExport for Pack {
    fn tag_value() -> &'static str {
        "PackPuzzptApi"
    }
}
