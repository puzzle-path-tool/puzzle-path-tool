use crate::serialization::is_empty;
use arrow::Arrow;
use cage::Cage;
use cell::Cell;
use cosmetic::Cosmetic;
use line::Line;
use metadata::Metadata;
use region::Region;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use trigger_effect::TriggerEffect;

mod arrow;
mod cage;
mod cell;
mod cosmetic;
mod line;
mod metadata;
mod region;
mod trigger_effect;

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
    cages: Box<[Cage]>,

    // ({??}) [][]
    #[serde(rename = "cells", default, skip_serializing_if = "is_empty")]
    cells: Box<[Cell]>,

    // (([x, y]) []) []
    #[serde(rename = "regions", default, skip_serializing_if = "is_empty")]
    regions: Box<[Region]>,

    // ({wayPoints: ([x, y]) [], color: "#aaaf", thickness: 9.6 }) []
    #[serde(rename = "lines", default, skip_serializing_if = "is_empty")]
    lines: Box<[Line]>,

    // ({ center: [x,y], width: 0.8, height: 0.8, thickness: 1.28, angle: 0, rounded: true, backgroundColor: "#FFFFFF", borderColor: "#aaaf", text: StrOrInt, fontSize: 32, stroke: "#ffffff"}) []
    #[serde(rename = "overlays", default, skip_serializing_if = "is_empty")]
    overlays: Box<[Cosmetic]>,

    // Same as overlay
    #[serde(rename = "underlays", default, skip_serializing_if = "is_empty")]
    underlays: Box<[Cosmetic]>,

    // ([x, y]) []
    #[serde(rename = "foglight", default, skip_serializing_if = "is_empty")]
    foglight: Box<[Value]>,

    // ({wayPoints: ([x, y]) [], color: "#aaaf", thickness: 9.6, headLength: 0.35 }) []
    #[serde(rename = "arrows", default, skip_serializing_if = "is_empty")]
    arrows: Box<[Arrow]>,

    // ({trigger: {type: "cellvalue", cell: "r1c1"}, effect: {type: "foglight", cells: "r1c9r2c9r3c5"}}) []
    #[serde(rename = "triggereffect", default, skip_serializing_if = "is_empty")]
    triggereffect: Box<[TriggerEffect]>,
}
