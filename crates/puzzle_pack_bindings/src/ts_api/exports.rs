use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::exports::matching::{EmitExpression, FieldType, RegisterVariable};
use crate::ts_api::identifier::Identifier;
use crate::ts_api::{PuzzptApiExport, TYPES};

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "DeductionPuzzptApi")]
pub struct Deduction {
    #[serde(rename = "name")]
    name: Identifier,
    #[serde(rename = "data")]
    data: DeductionData,
}

impl Deduction {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name()
    }
    #[must_use]
    pub fn description(&self) -> &String {
        todo!()
    }
    #[must_use]
    pub fn data(&self) -> &FieldType {
        &self.data.field_type
    }
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
#[serde(tag = "export_tag")]
#[serde(rename = "RulePuzzptApi")]
pub struct Rule {
    #[serde(rename = "name")]
    name: Identifier,
    #[serde(rename = "data")]
    data: DeductionData,
}

impl Rule {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name()
    }
    #[must_use]
    pub fn description(&self) -> &String {
        todo!()
    }
    #[must_use]
    pub fn data(&self) -> &FieldType {
        &self.data.field_type
    }
}

#[distributed_slice(TYPES)]
static RULE_TYPE: fn() -> String = Rule::decl;

impl PuzzptApiExport for Rule {
    fn tag_value() -> &'static str {
        "RulePuzzptApi"
    }
}

//Todo: Serde
#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "LogicStepPuzzptApi")]
pub struct LogicStep {
    #[serde(rename = "name")]
    name: Identifier,
    #[serde(rename = "variables")]
    variables: Vec<RegisterVariable>,
    #[serde(rename = "requirements")]
    requirements: Vec<matching::MatchExpression>,
    #[serde(rename = "whereClauses")]
    where_clauses: Vec<matching::MatchExpression>,
    #[serde(rename = "emissions")]
    emissions: Vec<EmitExpression>,
}

#[distributed_slice(TYPES)]
static LOGIC_STEP_TYPE: fn() -> String = LogicStep::decl;

impl PuzzptApiExport for LogicStep {
    fn tag_value() -> &'static str {
        "LogicStepPuzzptApi"
    }
}

pub mod matching;

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
