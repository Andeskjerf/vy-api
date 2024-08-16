use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Position {
    latitude: f64,
    longitude: f64,
}

impl Position {
    pub fn lat(&self) -> f64 {
        self.latitude
    }

    pub fn long(&self) -> f64 {
        self.longitude
    }
}
