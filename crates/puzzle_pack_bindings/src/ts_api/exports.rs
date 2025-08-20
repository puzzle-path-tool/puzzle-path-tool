use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::variable::Identifier;
use crate::ts_api::{PuzzptApiExport, TYPES};

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "DeductionPuzzptApi")]
pub struct Deduction {
    name: Identifier,
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
    name: Identifier,
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
    name: String,
}

#[distributed_slice(TYPES)]
static PACK_TYPE: fn() -> String = Pack::decl;

impl PuzzptApiExport for Pack {
    fn tag_value() -> &'static str {
        "PackPuzzptApi"
    }
}
