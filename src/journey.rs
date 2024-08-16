use std::fmt::Display;

use json::JsonValue;

use crate::{destination::Destination, duration::Duration};

#[derive(Debug, PartialEq)]
pub struct Journey {
    departure: String,
    arrival: String,
    departure_scheduled: String,
    arrival_scheduled: String,
    total_duration: Duration,
    legs: Vec<JsonValue>,
    pub from: Destination,
    pub to: Destination,
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
        let mut legs: Vec<JsonValue> = vec![];
        object.entries().for_each(|(k, v)| match k {
            "id" => id = v.to_string(),
            "departure" => departure = v.to_string(),
            "departureScheduled" => departure_scheduled = v.to_string(),
            "arrival" => arrival = v.to_string(),
            "arrivalScheduled" => arrival_scheduled = v.to_string(),
            "totalDuration" => total_duration = Duration::from_json(v.clone()),
            "legs" => legs.push(v.clone()),
            _ => (),
        });

        // not sure if the API can return more than 1
        // so assert it and see
        assert_eq!(legs.len(), 1);

        let mut from: Destination = Default::default();
        let mut to: Destination = Default::default();

        legs.first().unwrap().members().for_each(|m| {
            m.entries().for_each(|(k, v)| {
                println!("{}", v);
                match k {
                    // "from" => from = Destination::from_json(v.clone()),
                    // "to" => to = Destination::from_json(v.clone()),
                    "from" => from = serde_json::from_str(&v.to_string()).unwrap(),
                    "to" => to = serde_json::from_str(&v.to_string()).unwrap(),
                    _ => (),
                }
            });
        });

        Self {
            id,
            departure,
            departure_scheduled,
            arrival,
            arrival_scheduled,
            total_duration,
            legs,
            from,
            to,
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
