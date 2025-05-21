use serde::{Deserialize, Serialize};

use crate::serialization::{StrOrInt, is_empty};

use super::{cell_pos::CellPos, direction::Direction};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Constraint {
    #[serde(rename = "lines", default, skip_serializing_if = "is_empty")]
    lines: Box<[Box<[CellPos]>]>,

    #[serde(rename = "cell", default, skip_serializing_if = "Option::is_none")]
    cell: Option<CellPos>,

    #[serde(rename = "cells", default, skip_serializing_if = "is_empty")]
    cells: Box<[CellPos]>,

    #[serde(rename = "cloneCells", default, skip_serializing_if = "is_empty")]
    clone_cells: Box<[CellPos]>,

    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,

    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    value: Option<Box<str>>,

    #[serde(rename = "values", default, skip_serializing_if = "is_empty")]
    values: Box<[StrOrInt]>,
}
