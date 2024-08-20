use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    #[serde(alias = "itineraryId")]
    id: String,
    bookability: Bookability,
    #[serde(alias = "segmentOffers")]
    segment_offers: Vec<SegmentOffer>,
}

impl Offer {
    pub fn get_offer_ids(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        self.segment_offers.iter().for_each(|offer| {
            if !offer.price_configrations.is_empty() {
                result.push(offer.price_configrations.first().unwrap().id.clone());
            }
        });

        result
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SegmentOffer {
    id: String,
    #[serde(alias = "type")]
    type_: String,
    #[serde(alias = "legIds")]
    leg_ids: Vec<String>,
    pub bookability: Bookability,
    #[serde(alias = "priceConfigurations")]
    pub price_configrations: Vec<PriceConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceConfiguration {
    pub id: String,
    name: String,
    #[serde(alias = "type")]
    type_: String,
    authorities: Vec<String>,
}

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum BookabilityType {
    #[serde(rename = "FULLY_BOOKABLE")]
    FullyBookable,
    #[serde(rename = "NOT_BOOKABLE")]
    #[default]
    NotBookable,
    #[serde(rename = "PARTIALLY_BOOKABLE")]
    PartiallyBookable,
    #[serde(rename = "BOOKABLE")]
    Bookable,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Bookability {
    #[serde(alias = "type")]
    type_: BookabilityType,
    summary: Option<String>,
    description: Option<String>,
    #[serde(alias = "externalLink")]
    external_link: Option<String>,
}
