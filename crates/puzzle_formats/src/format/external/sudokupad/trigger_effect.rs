use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct TriggerEffect {
    // TODO: ({trigger: {type: "cellvalue", cell: "r1c1"}, effect: {type: "foglight", cells: "r1c9r2c9r3c5"}})
    #[serde(rename = "restValues", default, flatten)]
    rest_values: Value,
}
