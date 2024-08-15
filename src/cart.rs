use crate::seat::Seat;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Cart {
    id: u16,
    car_number: u8,
    sequence: u8,
    // think this is available seats?
    number_of_seats: u8,
    // available beds?
    number_of_beds: u8,
    railcar_set_id: u8,
    litra_code: String,
    image_content_type: String,
    image_height: u32,
    image_width: u32,
    base_image_url: String,
    railcar_image_path: String,
    rotated: bool,
    seats: Vec<Seat>,
}

impl Cart {}
