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
    data: DeductionData
}

impl Deduction {
    pub fn get_name(&self) -> &String {
        self.name.get_name()
    }
    pub fn get_description(&self) -> &String {
        todo!()
    }
    pub fn get_data(&self) -> &HashMap<String, FieldType> {
        &self.data.data
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub struct DeductionData {
    data: HashMap<String, FieldType>
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
pub enum FieldType {
    Number,
    Boolean,
    Enum(Vec<String>),
    Set(Box<FieldType>),
    Object(HashMap<String, FieldType>)
}

#[distributed_slice(TYPES)]
static DEDUCTION_TYPE: fn() -> String = Deduction::decl;

impl PuzzptApiExport for Deduction {
    fn tag_value() -> &'static str {
        "DeductionPuzzptApi"
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "RulePuzzptApi")]
pub struct Rule {
    #[serde(rename = "name")]
    name: Identifier,
    data: DeductionData
}

#[distributed_slice(TYPES)]
static RULE_TYPE: fn() -> String = Rule::decl;

impl PuzzptApiExport for Rule {
    fn tag_value() -> &'static str {
        "RulePuzzptApi"
    }
}

impl Rule {
    pub fn get_name(&self) -> &String {
        self.name.get_name()
    }
    pub fn get_description(&self) -> &String {
        todo!()
    }
    pub fn get_data(&self) -> &HashMap<String, FieldType> {
        &self.data.data
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
