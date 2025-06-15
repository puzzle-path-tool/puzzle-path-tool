use linkme::distributed_slice;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_api::TYPES;

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "ExamplePuzzptApi")]
pub struct Example {
    #[serde(rename = "value")]
    value: String,
    #[serde(rename = "number")]
    number: i32,
    #[serde(rename = "number2")]
    number2: i32,
}

#[distributed_slice(TYPES)]
static EXAMPLE_TYPE: fn() -> String = Example::decl;
