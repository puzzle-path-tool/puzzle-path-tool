use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::{PuzzptApiExport, TYPES};

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "export_tag")]
#[serde(rename = "ExamplePuzzptApi")]
pub struct DeductionData {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "data")]
    data: ExampleStuff,
}

#[distributed_slice(TYPES)]
static EXAMPLE_TYPE: fn() -> String = Example::decl;

impl PuzzptApiExport for Example {
    fn tag_value() -> &'static str {
        "ExamplePuzzptApi"
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "tag", content = "value")]
#[serde(rename = "ExampleStuffPuzzptApi")]
pub enum DeductionData {
    #[serde(rename = "StuffA")]
    StuffA(i32),
    #[serde(rename = "StuffB")]
    StuffB(bool),
    #[serde(rename = "StuffC")]
    StuffC,
}

#[distributed_slice(TYPES)]
static EXAMPLE_STUFF_TYPE: fn() -> String = ExampleStuff::decl;
