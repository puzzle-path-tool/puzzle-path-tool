use super::serialization::{is_default, is_empty};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "source", default)]
    source: Box<str>,

    #[serde(rename = "title", default)]
    title: Box<str>,

    #[serde(rename = "author", default)]
    author: Box<str>,

    #[serde(rename = "rules", default)]
    rules: Box<str>,

    #[serde(
        rename = "msgcorrect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    msgcorrect: Option<Box<str>>,

    #[serde(rename = "antiknight", default, skip_serializing_if = "is_default")]
    antiknight: bool,

    #[serde(rename = "antiking", default, skip_serializing_if = "is_default")]
    antiking: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cage {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cell {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Region {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Line {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cosmetic {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Arrow {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct TriggerEffect {}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
// Positions are zero based
pub struct SudokupadSclFormat {
    #[serde(rename = "id", default)]
    id: Box<str>,

    #[serde(rename = "metadata")]
    metadata: Metadata,

    // ({cells: ([x, y]) [], value: "45", unique: true, hidden: true, type: "rowcol" }) []
    #[serde(rename = "cages", default, skip_serializing_if = "is_empty")]
    cages: Box<[Value]>,

    // ({??}) [][]
    #[serde(rename = "cells", default, skip_serializing_if = "is_empty")]
    cells: Box<[Value]>,

    // (([x, y]) []) []
    #[serde(rename = "regions", default, skip_serializing_if = "is_empty")]
    regions: Box<[Value]>,

    // ({wayPoints: ([x, y]) [], color: "#aaaf", thickness: 9.6 }) []
    #[serde(rename = "lines", default, skip_serializing_if = "is_empty")]
    lines: Box<[Value]>,

    // ({ center: [x,y], width: 0.8, height: 0.8, thickness: 1.28, angle: 0, rounded: true, backgroundColor: "#FFFFFF", borderColor: "#aaaf", text: StrOrInt, fontSize: 32, stroke: "#ffffff"}) []
    #[serde(rename = "overlays", default, skip_serializing_if = "is_empty")]
    overlays: Box<[Value]>,

    // Same as overlay
    #[serde(rename = "underlays", default, skip_serializing_if = "is_empty")]
    underlays: Box<[Value]>,

    // ([x, y]) []
    #[serde(rename = "foglight", default, skip_serializing_if = "is_empty")]
    foglight: Box<[Value]>,

    #[serde(rename = "global", default, skip_serializing_if = "is_empty")]
    global: Box<[Value]>,

    // ({wayPoints: ([x, y]) [], color: "#aaaf", thickness: 9.6, headLength: 0.35 }) []
    #[serde(rename = "arrows", default, skip_serializing_if = "is_empty")]
    arrows: Box<[Value]>,

    // ({trigger: {type: "cellvalue", cell: "r1c1"}, effect: {type: "foglight", cells: "r1c9r2c9r3c5"}}) []
    #[serde(rename = "triggereffect", default, skip_serializing_if = "is_empty")]
    triggereffect: Box<[Value]>,
}
