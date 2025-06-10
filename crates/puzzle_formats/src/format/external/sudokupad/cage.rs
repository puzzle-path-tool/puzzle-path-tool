use serde::{Deserialize, Serialize};

use crate::serialization::is_default;

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cage {
    #[serde(rename = "cells", default)]
    cells: Box<[Pos<i32>]>,

    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    value: Option<Box<str>>,

    #[serde(rename = "unique", default, skip_serializing_if = "is_default")]
    unique: bool,

    #[serde(rename = "hidden", default, skip_serializing_if = "is_default")]
    hidden: bool,

    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    kind: Option<Box<str>>,
}
