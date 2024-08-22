use crate::{destination::Destination, duration::Duration, leg::Leg};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Journey {
    departure: String,
    arrival: String,
    #[serde(alias = "departureScheduled")]
    departure_scheduled: String,
    #[serde(alias = "arrivalScheduled")]
    arrival_scheduled: String,
    #[serde(alias = "totalDuration")]
    total_duration: Duration,
    pub legs: Vec<Leg>,
    pub from: Destination,
    pub to: Destination,
    id: String,
}

impl Journey {
    pub fn id(&self) -> &String {
        &self.id
    }
}

impl Display for Journey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\ndeparture: {}\narrival: {}",
            self.id, self.departure, self.arrival
        )
    }
}
