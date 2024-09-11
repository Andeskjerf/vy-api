use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug, Clone)]
pub struct Line {
    pub name: String,
    #[serde(alias = "longName")]
    long_name: Option<String>,
    #[serde(alias = "serviceLineId")]
    pub service_line_id: String,
    #[serde(alias = "colour")]
    color: Option<String>,
    #[serde(alias = "backgroundColour")]
    background_color: Option<String>,
    #[serde(alias = "borderColour")]
    border_color: Option<String>,
    #[serde(alias = "textColour")]
    text_color: Option<String>,
    #[serde(alias = "serviceDestination")]
    pub service_destination: Option<String>,
    #[serde(alias = "serviceDeparture")]
    service_departure: Option<String>,
    operator: Option<String>,
}
