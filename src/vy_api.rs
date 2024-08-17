use json::JsonValue;
use reqwest::{Client, ClientBuilder, Response, StatusCode};
use std::error::Error;

use crate::{cart::Cart, consts::VY_URL, destination::Destination, journey::Journey, offer::Offer};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0";

pub struct VyAPI {
    client: Client,
}

impl VyAPI {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            client: ClientBuilder::new().user_agent(USER_AGENT).build()?,
        })
    }

    pub async fn perform_search_and_get_ids<'a>(
        &self,
        from: &'a Destination,
        to: &'a Destination,
        date: &'a str,
    ) -> Result<Vec<Journey>, Box<dyn Error + Send + Sync>> {
        let target_url = format!("{}/services/itinerary/api/travel-planner/search", VY_URL);
        let search_body = format!(
            r#"
        {{
                "from":{{
                    "latitude":{},
                    "longitude":{},
                    "userQuery":{{"searchTerm":"{}"}},
                    "externalReferences":[
                        {{
                            "id": "{}",
                            "origin": "NSR"
                        }}
                    ]
                }},
                "to":{{
                    "latitude":{},
                    "longitude":{},
                    "userQuery":{{"searchTerm":"{}"}},
                    "externalReferences":[
                        {{
                            "id": "{}",
                            "origin": "NSR"
                        }}
                    ]
                }},
                "date":"{}",
                "filter":{{
                    "includeModes":["TRAIN","BUS","TRAM","METRO","WATER"]
                }},
                "searchContext": "FIND_JOURNEY_INITIAL"
            }}
        "#,
            from.position.lat(),
            from.position.long(),
            from.name,
            from.get_nsr_code(),
            to.position.lat(),
            to.position.long(),
            to.name,
            to.get_nsr_code(),
            date
        );

        println!("{}", search_body);

        let response = self
            .client
            .post(target_url)
            .header("Content-Type", "application/json")
            .body(search_body)
            .send()
            .await?;

        assert!(response.status() == StatusCode::OK);

        let suggestions = VyAPI::get_json_array_from_response(response, "suggestions").await?;
        let mut result: Vec<Journey> = vec![];
        suggestions.members().for_each(|member| {
            result.push(serde_json::from_str(&member.to_string()).unwrap());
        });

        assert_ne!(result.len(), 0);

        Ok(result)
    }

    pub async fn get_location_data(
        &self,
        location_name: &str,
    ) -> Result<Vec<Destination>, Box<dyn Error + Send + Sync>> {
        let location_name = location_name.replace(" ", "+");
        let response = self
            .client
            .get(format!(
                "{}/services/location/places/autosuggest?query={}&searchOrigin=default",
                VY_URL, location_name
            ))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        assert!(response.status() == StatusCode::OK);

        let suggestions = VyAPI::get_json_array_from_response(response, "suggestions").await?;
        let mut result: Vec<Destination> = vec![];
        suggestions.members().for_each(|member| {
            result.push(serde_json::from_str(&member.clone().to_string()).unwrap());
        });

        assert_ne!(result.len(), 0);

        Ok(result)
    }

    pub async fn get_offers_for_search(
        &self,
        search_results: &[Journey],
    ) -> Result<Vec<Offer>, Box<dyn Error + Send + Sync>> {
        let target_url = format!("{}/services/booking/api/offer", VY_URL);

        let ids = search_results.iter().fold(String::new(), |f, x| {
            format!(
                "{}\"{}\"{}",
                f,
                x.id(),
                if search_results.last().unwrap() != x {
                    ","
                } else {
                    ""
                }
            )
        });

        let offer_body = format!(
            r#"
            {{
                "itineraryIds":[{}],
                "addons":[],
                "isRoundTrip":false,
                "passengers":[
                    {{
                        "age":null,
                        "interrailCode":null,
                        "categories":[]
                    }}
                ],
                "promotionCode":"",
                "numberOfRetries":0,
                "orderId":null,
                "legIdsToChange":null
            }}
            "#,
            ids
        );

        println!("{}", offer_body);

        let response = self
            .client
            .post(target_url)
            .header("Content-Type", "application/json")
            .body(offer_body.clone())
            .send()
            .await?;

        assert!(response.status() == StatusCode::OK);

        let suggestions = VyAPI::get_json_array_from_response(response, "itineraryOffers").await?;
        let mut result: Vec<Offer> = vec![];
        suggestions.members().for_each(|member| {
            result.push(serde_json::from_str(&member.to_string()).unwrap());
        });

        Ok(result)
    }

    pub async fn make_order(&self, id: &String) -> Result<String, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/services/booking/api/v2/orders", VY_URL);

        let body = format!(
            r#"
            {{
                "offerIds":["{}"]
            }}
            "#,
            id
        );

        println!("{}", body);

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.clone())
            .send()
            .await?;

        assert!(response.status() == StatusCode::OK);

        let text = json::parse(response.text().await?.as_str()).unwrap();

        let mut result = String::new();
        text.entries().for_each(|(k, v)| match k {
            "id" => result = v.to_string(),
            _ => panic!("no id given"),
        });

        Ok(result)
    }

    pub async fn delete_order(&self, id: &String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!("{}/services/booking/api/v2/orders/{}", VY_URL, id);
        let response = self
            .client
            .delete(url)
            .header("Accept", "application/json")
            .header("Content-Type", "text/xml")
            .send()
            .await?;

        assert!(response.status() == StatusCode::NO_CONTENT);

        Ok(())
    }

    pub async fn get_available_seats(
        &self,
        order_guid: String,
        from_nsr: String,
        to_nsr: String,
    ) -> Result<Vec<Cart>, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "{}/services/seat/availableseating/available-railcars",
            VY_URL
        );

        let body = format!(
            r#"
                {{
                     "orderGuid": "{}",
                     "fromNsrCode": "{}",
                     "toNsrCode": "{}",
                     "specificationType": "SEAT"
                }}
            "#,
            order_guid, from_nsr, to_nsr
        );
        println!("{}", body);

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        assert!(response.status() == StatusCode::OK);

        let text = &response.text().await?;
        let parsed = json::parse(text).unwrap();

        let mut result: Vec<Cart> = vec![];

        parsed.members().for_each(|m| {
            result.push(serde_json::from_str(&m.to_string()).unwrap());
        });

        // result.0.push(serde_json::from_str(&response.text().await?).unwrap());

        Ok(result)
    }

    async fn get_json_array_from_response(
        response: Response,
        key: &str,
    ) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
        let response_json = json::parse(response.text().await?.as_str()).unwrap();
        Ok(response_json
            .entries()
            .find(|(k, _)| k == &key)
            .map(|(_, v)| v.clone())
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use reqwest::get;
    use std::time::SystemTime;

    use super::*;

    fn get_current_datetime() -> String {
        std::convert::Into::<DateTime<Utc>>::into(SystemTime::now()).to_rfc3339()
    }

    fn build_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
        Ok(ClientBuilder::new().user_agent(USER_AGENT).build()?)
    }

    // #[tokio::test]
    // async fn test_search_api() -> Result<(), Box<dyn Error + Send + Sync>> {
    //     let target_url = format!("{}/services/itinerary/api/travel-planner/search", VY_URL);
    //     let client = build_client()?;
    //     (target_url).await?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn search_has_suggestions() -> Result<(), Box<dyn Error + Send + Sync>> {
    //     let api = VyAPI::new()?;
    //     let suggestions = api
    //         .perform_search_and_get_ids("Oslo S", "Bergen stasjon", &get_current_datetime())
    //         .await?;

    //     assert_eq!(suggestions.0.len(), 7);
    //     Ok(())
    // }
}
