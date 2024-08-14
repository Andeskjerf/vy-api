use json::JsonValue;

#[derive(Debug)]
pub struct Offer {
    id: String,
    bookability: Bookability,
    segment_offers: Vec<SegmentOffer>,
}

impl Offer {
    pub fn from_json(object: JsonValue) -> Self {
        let mut id = String::new();
        let mut bookability = Bookability::default();
        let mut segment_offers: Vec<SegmentOffer> = vec![];

        object.entries().for_each(|(k, v)| match k {
            "itineraryId" => id = v.to_string(),
            "bookability" => bookability = Bookability::from_json(v.clone()),
            "segmentOffers" => v.members().for_each(|v| {
                segment_offers.push(SegmentOffer::from_json(v.clone()));
            }),
            _ => println!("invalid key: {}", k),
        });

        Self {
            id,
            bookability,
            segment_offers,
        }
    }

    pub fn get_offer_ids(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        self.segment_offers.iter().for_each(|offer| {
            // println!("bookability: {:?}, {:?}", offer.bookability.type_, offer.price_configrations);
            println!("{:?}", self.id);
            if !offer.price_configrations.is_empty() {
                result.push(offer.price_configrations.first().unwrap().id.clone());
            }
        });

        result
    }
}

#[derive(Debug)]
struct SegmentOffer {
    id: String,
    type_: String,
    pub bookability: Bookability,
    pub price_configrations: Vec<PriceConfiguration>,
}

impl SegmentOffer {
    fn from_json(object: JsonValue) -> Self {
        let mut id = String::new();
        let mut type_ = String::new();
        let mut bookability = Bookability::default();
        let mut price_configrations: Vec<PriceConfiguration> = vec![];

        object.entries().for_each(|(k, v)| match k {
            "id" => id = v.to_string(),
            "type" => type_ = v.to_string(),
            "bookability" => bookability = Bookability::from_json(v.clone()),
            "priceConfigurations" => v.members().for_each(|v| {
                price_configrations.push(PriceConfiguration::from_json(v.clone()));
            }),
            _ => (),
        });

        Self {
            id,
            type_,
            bookability,
            price_configrations,
        }
    }
}

#[derive(Debug)]
struct PriceConfiguration {
    pub id: String,
    name: String,
    type_: String,
}

impl PriceConfiguration {
    fn from_json(object: JsonValue) -> Self {
        let mut id = String::new();
        let mut type_ = String::new();
        let mut name = String::new();

        object.entries().for_each(|(k, v)| match k {
            "id" => id = v.to_string(),
            "type" => type_ = v.to_string(),
            "name" => name = v.to_string(),
            _ => (),
        });

        Self { id, type_, name }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
enum BookabilityType {
    FullyBookable,
    #[default]
    NotBookable,
    Bookable,
}

#[derive(Default, Debug)]
struct Bookability {
    type_: BookabilityType,
    summary: String,
    description: String,
    external_link: String,
}

impl Bookability {
    fn from_json(object: JsonValue) -> Self {
        let mut type_: BookabilityType = BookabilityType::NotBookable;
        let mut summary = String::new();
        let mut description = String::new();
        let mut external_link = String::new();
        object.entries().for_each(|(k, v)| match k {
            "type" => match v.as_str().unwrap() {
                "NOT_BOOKABLE" => type_ = BookabilityType::NotBookable,
                "FULLY_BOOKABLE" => type_ = BookabilityType::FullyBookable,
                "BOOKABLE" => type_ = BookabilityType::Bookable,
                _ => panic!("invalid bookability_type: {}", v),
            },
            "summary" => summary = v.to_string(),
            "description" => description = v.to_string(),
            "externalLink" => external_link = v.to_string(),
            _ => println!("invalid key: {}", k),
        });

        Self {
            type_,
            summary,
            description,
            external_link,
        }
    }
}
