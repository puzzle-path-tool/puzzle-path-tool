use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Cosmetic {
    // TODO: ({ center: [x,y], width: 0.8, height: 0.8, thickness: 1.28, angle: 0, rounded: true, backgroundColor: "#FFFFFF", borderColor: "#aaaf", text: StrOrInt, fontSize: 32, stroke: "#ffffff"})
    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
