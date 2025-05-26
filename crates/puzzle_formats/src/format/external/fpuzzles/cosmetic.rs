use serde::{Deserialize, Serialize};

use crate::serialization::{StrOrInt, is_empty};

use super::{cell_pos::CellPos, direction::Direction};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cosmetic {
    #[serde(rename = "lines", default, skip_serializing_if = "is_empty")]
    lines: Box<[Box<[CellPos]>]>,

    #[serde(rename = "cell", default, skip_serializing_if = "Option::is_none")]
    cell: Option<CellPos>,

    #[serde(rename = "cells", default, skip_serializing_if = "is_empty")]
    cells: Box<[CellPos]>,

    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,

    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    value: Option<Box<str>>,

    #[serde(rename = "values", default, skip_serializing_if = "is_empty")]
    values: Box<[StrOrInt]>,

    #[serde(rename = "baseC", default, skip_serializing_if = "Option::is_none")]
    base_c: Option<Box<str>>, // Maybe Parse to Color Type

    #[serde(rename = "outlineC", default, skip_serializing_if = "Option::is_none")]
    outline_c: Option<Box<str>>, // Maybe Parse to Color Type

    #[serde(rename = "fontC", default, skip_serializing_if = "Option::is_none")]
    font_c: Option<Box<str>>, // Maybe Parse to Color Type

    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    size: Option<f64>,

    #[serde(rename = "width", default, skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(rename = "height", default, skip_serializing_if = "Option::is_none")]
    height: Option<f64>,

    #[serde(rename = "angle", default, skip_serializing_if = "Option::is_none")]
    angle: Option<f64>,
}
