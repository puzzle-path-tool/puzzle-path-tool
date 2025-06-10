use csscolorparser::Color;
use serde::{Deserialize, Serialize};

use crate::serialization::is_default;

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Line {
    #[serde(rename = "wayPoints", default, skip_serializing_if = "is_default")]
    way_points: Box<[Pos<f64>]>,

    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    // Maybe can be string
    #[serde(rename = "thickness", skip_serializing_if = "Option::is_none")]
    thickness: Option<f64>,

    #[serde(rename = "target", default, skip_serializing_if = "Option::is_none")]
    target_layer: Option<Box<str>>,

    #[serde(rename = "d", default, skip_serializing_if = "Option::is_none")]
    path: Option<Box<str>>,

    #[serde(
        rename = "stroke-linecap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    stroke_linecap: Option<Box<str>>,

    #[serde(
        rename = "stroke-linejoin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    stroke_linejoin: Option<Box<str>>,
    // Maybe add field className
}
