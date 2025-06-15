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
    #[serde(rename = "stuff")]
    stuff: ExampleStuff,
}

#[derive(Serialize, Deserialize, TS, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename = "ExampleStuffPuzzptApi")]
pub enum ExampleStuff {
    StuffA { value: i32 },
    StuffB { cond: bool },
    StuffC,
}

#[distributed_slice(TYPES)]
static EXAMPLE_TYPE: fn() -> String = Example::decl;

#[distributed_slice(TYPES)]
static EXAMPLE_STUFF_TYPE: fn() -> String = ExampleStuff::decl;
