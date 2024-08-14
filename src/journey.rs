use std::fmt::Display;

use json::JsonValue;

use crate::duration::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct Journey {
    departure: String,
    arrival: String,
    departure_scheduled: String,
    arrival_scheduled: String,
    total_duration: Duration,
    id: String,
}

impl Journey {
    pub fn from_json(object: JsonValue) -> Self {
        let mut id = Default::default();
        let mut departure = Default::default();
        let mut departure_scheduled = Default::default();
        let mut arrival = Default::default();
        let mut arrival_scheduled = Default::default();
        let mut total_duration = Default::default();
        object.entries().for_each(|(k, v)| match k {
            "id" => id = v.to_string(),
            "departure" => departure = v.to_string(),
            "departureScheduled" => departure_scheduled = v.to_string(),
            "arrival" => arrival = v.to_string(),
            "arrivalScheduled" => arrival_scheduled = v.to_string(),
            "totalDuration" => total_duration = Duration::from_json(v.clone()),
            _ => println!("invalid key: {k:?}"),
        });
        Self {
            id,
            departure,
            departure_scheduled,
            arrival,
            arrival_scheduled,
            total_duration,
        }
    }

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
