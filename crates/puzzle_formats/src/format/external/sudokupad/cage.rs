use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cage {
    // TODO: ({cells: ([x, y]) [], value: "45", unique: true, hidden: true, type: "rowcol" })
    #[serde(rename = "cells", default)]
    cells: Box<[Pos<i32>]>,

    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
