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
// TODO: Positions are zero based (if not in R-C- format)
pub struct SudokupadSclFormat {
    #[serde(rename = "id", default)]
    id: Box<str>,

    #[serde(rename = "metadata")]
    metadata: Metadata,

    #[serde(rename = "cages", default, skip_serializing_if = "is_empty")]
    cages: Box<[Cage]>,

    #[serde(rename = "cells", default, skip_serializing_if = "is_empty")]
    cells: Box<[Box<[Cell]>]>,

    #[serde(rename = "regions", default, skip_serializing_if = "is_empty")]
    regions: Box<[Region]>,

    #[serde(rename = "lines", default, skip_serializing_if = "is_empty")]
    lines: Box<[Line]>,

    #[serde(rename = "overlays", default, skip_serializing_if = "is_empty")]
    overlays: Box<[Cosmetic]>,

    #[serde(rename = "underlays", default, skip_serializing_if = "is_empty")]
    underlays: Box<[Cosmetic]>,

    // TODO: ([x, y]) []
    #[serde(rename = "foglight", default, skip_serializing_if = "is_empty")]
    foglight: Box<[Value]>,

    #[serde(rename = "arrows", default, skip_serializing_if = "is_empty")]
    arrows: Box<[Arrow]>,

    #[serde(rename = "triggereffect", default, skip_serializing_if = "is_empty")]
    triggereffect: Box<[TriggerEffect]>,
}
