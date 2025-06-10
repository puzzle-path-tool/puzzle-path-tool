use csscolorparser::Color;
use serde::{Deserialize, Serialize};

use crate::serialization::{StrOrInt, is_default};

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cosmetic {
    #[serde(rename = "center")]
    center: Pos<f64>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    height: Option<f64>,

    #[serde(rename = "thickness", skip_serializing_if = "Option::is_none")]
    thickness: Option<f64>,

    #[serde(rename = "angle", skip_serializing_if = "Option::is_none")]
    angle: Option<f64>,

    #[serde(rename = "rounded", default, skip_serializing_if = "is_default")]
    rounded: bool,

    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(rename = "borderColor", skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(rename = "values", default, skip_serializing_if = "Option::is_none")]
    text: Option<StrOrInt>,

    #[serde(rename = "fontSize", default, skip_serializing_if = "Option::is_none")]
    font_size: Option<i32>,

    #[serde(rename = "stroke", skip_serializing_if = "Option::is_none")]
    stroke: Option<Color>,
}
