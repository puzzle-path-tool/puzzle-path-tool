use csscolorparser::Color;
use serde::{Deserialize, Serialize};

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Arrow {
    #[serde(rename = "wayPoints", default)]
    way_points: Box<[Pos<f64>]>,

    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(rename = "thickness", skip_serializing_if = "Option::is_none")]
    thickness: Option<f64>,

    #[serde(rename = "headLength", skip_serializing_if = "Option::is_none")]
    head_length: Option<f64>,
}
