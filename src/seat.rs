use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Seat {
    id: u32,
    #[serde(rename = "seatNumber")]
    seat_number: i16,
    // #[serde(rename = "seatSpecification")]
    // seat_specification: String,
    #[serde(rename = "positionX")]
    position_x: i32,
    #[serde(rename = "positionY")]
    position_y: i32,
    direction: String,
    #[serde(rename = "seatIconId")]
    seat_icon_id: u32,
    pub available: bool,
}
