use crate::seat::Seat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cart {
    pub id: u16,
    #[serde(rename = "carNumber")]
    car_number: u8,
    sequence: u8,
    // think this is available seats?
    #[serde(rename = "numberOfSeats")]
    number_of_seats: u8,
    // available beds?
    #[serde(rename = "numberOfBeds")]
    number_of_beds: u8,
    #[serde(rename = "railcarSetId")]
    railcar_set_id: u8,
    #[serde(rename = "litraCode")]
    litra_code: String,
    #[serde(rename = "imageContentType")]
    image_content_type: String,
    #[serde(rename = "imageHeight")]
    image_height: u32,
    #[serde(rename = "imageWidth")]
    image_width: u32,
    #[serde(rename = "baseImageUrl")]
    base_image_url: String,
    #[serde(rename = "railcarImagePath")]
    railcar_image_path: String,
    rotated: bool,
    #[serde(rename = "railcarElements")]
    pub seats: Vec<Seat>,
}
