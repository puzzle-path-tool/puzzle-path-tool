use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct Region {
    // TODO:  (([x, y]) [])
    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
