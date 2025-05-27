use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::pos::Pos;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Line {
    // TODO: ({wayPoints: ([x, y]) [], color: "#aaaf", thickness: 9.6 })
    // Maybe flatten for Arrow, ??, Maybe they are the same type??
    #[serde(rename = "wayPoints", default)]
    way_points: Box<[Pos<f64>]>,

    // target ["arrows"], color ["none"], thickness, wayPoints, className, d
    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
