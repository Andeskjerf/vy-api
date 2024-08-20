use serde::{Deserialize, Serialize};

use crate::{destination::Destination, duration::Duration, line::Line, operator::Operator};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Leg {
    #[serde(alias = "enturId", default)]
    entur_id: Option<String>,
    #[serde(default)]
    id: String,
    #[serde(alias = "departureScheduled")]
    departure_scheduled: String,
    #[serde(alias = "arrivalScheduled")]
    arrival_scheduled: String,
    #[serde(alias = "departureRealTime")]
    departure_realtime: Option<String>,
    #[serde(alias = "arrivalRealTime")]
    arrival_realtime: Option<String>,
    duration: Duration,
    #[serde(default)]
    branding: Operator,
    #[serde(default)]
    operator: Operator,
    #[serde(default)]
    authority: Operator,
    #[serde(default)]
    pub from: Destination,
    #[serde(default)]
    pub to: Destination,
    #[serde(default)]
    line: Line,
    mode: Option<String>,
    #[serde(alias = "isNightTrain", default)]
    is_night_train: bool,
}

impl Leg {
    pub fn get_from_nsr_code(&self) -> String {
        self.from.get_nsr_code()
    }

    pub fn get_to_nsr_code(&self) -> String {
        self.to.get_nsr_code()
    }
}
