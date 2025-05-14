use super::serialization::{is_default, is_empty};
use constraint::Constraint;
use cosmetic::Cosmetic;
use grid_cell::GridCell;
use negative::Negative;
use serde::{Deserialize, Serialize};

mod cell_pos;
mod constraint;
mod cosmetic;
mod direction;
mod grid_cell;
mod highlight_color;
mod negative;
mod region;

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct FPuzzlesFormat {
    #[serde(rename = "title", default)]
    title: Box<str>,

    #[serde(rename = "author", default)]
    author: Box<str>,

    #[serde(rename = "ruleset", default)]
    ruleset: Box<str>,

    #[serde(rename = "size")]
    size: i32,

    #[serde(rename = "highlightConflicts", default)]
    highlight_conflicts: bool,

    #[serde(rename = "grid")]
    grid: Box<[Box<[GridCell]>]>,

    #[serde(rename = "diagonal+", default, skip_serializing_if = "is_default")]
    diagonal_positive: bool,

    #[serde(rename = "diagonal-", default, skip_serializing_if = "is_default")]
    diagonal_negative: bool,

    #[serde(rename = "antiknight", default, skip_serializing_if = "is_default")]
    antiknight: bool,

    #[serde(rename = "antiking", default, skip_serializing_if = "is_default")]
    antiking: bool,

    #[serde(rename = "disjointgroups", default, skip_serializing_if = "is_default")]
    disjointgroups: bool,

    #[serde(rename = "nonconsecutive", default, skip_serializing_if = "is_default")]
    nonconsecutive: bool,

    #[serde(rename = "extraregion", default, skip_serializing_if = "is_empty")]
    extraregion: Box<[Constraint]>,

    #[serde(rename = "odd", default, skip_serializing_if = "is_empty")]
    odd: Box<[Constraint]>,

    #[serde(rename = "even", default, skip_serializing_if = "is_empty")]
    even: Box<[Constraint]>,

    #[serde(rename = "thermometer", default, skip_serializing_if = "is_empty")]
    thermometer: Box<[Constraint]>,

    #[serde(rename = "palindrome", default, skip_serializing_if = "is_empty")]
    palindrome: Box<[Constraint]>,

    #[serde(rename = "killercage", default, skip_serializing_if = "is_empty")]
    killercage: Box<[Constraint]>,

    #[serde(rename = "littlekillersum", default, skip_serializing_if = "is_empty")]
    littlekillersum: Box<[Constraint]>,

    #[serde(rename = "sandwichsum", default, skip_serializing_if = "is_empty")]
    sandwichsum: Box<[Constraint]>,

    #[serde(rename = "difference", default, skip_serializing_if = "is_empty")]
    difference: Box<[Constraint]>,

    #[serde(rename = "negative", default, skip_serializing_if = "is_default")]
    negative: Negative,

    #[serde(rename = "ratio", default, skip_serializing_if = "is_empty")]
    ratio: Box<[Constraint]>,

    #[serde(rename = "clone", default, skip_serializing_if = "is_empty")]
    clone: Box<[Constraint]>,

    #[serde(rename = "arrow", default, skip_serializing_if = "is_empty")]
    arrow: Box<[Constraint]>,

    #[serde(rename = "betweenline", default, skip_serializing_if = "is_empty")]
    betweenline: Box<[Constraint]>,

    #[serde(rename = "minimum", default, skip_serializing_if = "is_empty")]
    minimum: Box<[Constraint]>,

    #[serde(rename = "maximum", default, skip_serializing_if = "is_empty")]
    maximum: Box<[Constraint]>,

    #[serde(rename = "xv", default, skip_serializing_if = "is_empty")]
    xv: Box<[Constraint]>,

    #[serde(rename = "quadruple", default, skip_serializing_if = "is_empty")]
    quadruple: Box<[Constraint]>,

    #[serde(rename = "text", default, skip_serializing_if = "is_empty")]
    text: Box<[Cosmetic]>,

    #[serde(rename = "circle", default, skip_serializing_if = "is_empty")]
    circle: Box<[Cosmetic]>,

    #[serde(rename = "rectangle", default, skip_serializing_if = "is_empty")]
    rectangle: Box<[Cosmetic]>,

    #[serde(rename = "line", default, skip_serializing_if = "is_empty")]
    line: Box<[Cosmetic]>,

    #[serde(rename = "cage", default, skip_serializing_if = "is_empty")]
    cage: Box<[Cosmetic]>,
}
