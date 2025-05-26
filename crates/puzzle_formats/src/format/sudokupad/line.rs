use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Line {
    // TODO: ({wayPoints: ([x, y]) [], color: "#aaaf", thickness: 9.6 })
    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
