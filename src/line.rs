use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Line {
    name: String,
    #[serde(alias = "longName")]
    long_name: String,
    #[serde(alias = "serviceLineId")]
    service_line_id: String,
    #[serde(alias = "colour")]
    color: String,
    #[serde(alias = "backgroundColour")]
    background_color: String,
    #[serde(alias = "borderColour")]
    border_color: String,
    #[serde(alias = "textColour")]
    text_color: String,
    #[serde(alias = "serviceDestination")]
    service_destination: String,
    #[serde(alias = "serviceDeparture")]
    service_departure: String,
    operator: String,
}
