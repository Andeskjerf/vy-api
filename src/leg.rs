use serde::{Deserialize, Serialize};

use crate::{destination::Destination, duration::Duration, operator::Operator};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Leg {
    #[serde(alias = "enturId")]
    entur_id: String,
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
    branding: Operator,
    operator: Operator,
    authority: Operator,
    pub from: Destination,
    pub to: Destination,
}

impl Leg {
    pub fn get_from_nsr_code(&self) -> String {
        self.from.get_nsr_code()
    }

    pub fn get_to_nsr_code(&self) -> String {
        self.to.get_nsr_code()
    }
}
