use serde::Deserialize;

use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Operator {
    #[serde(default)]
    name: String,
    #[serde(default)]
    id: String,
}
