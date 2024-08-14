use std::error::Error;

use consts::VY_URL;
use vy_api::VyAPI;

mod consts;
mod destination;
mod duration;
mod journey;
mod vy_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let from = "Oslo S";
    let to = "Bergen stasjon";
    let date = "2024-08-16T04:00:00.000Z";

    let api = VyAPI::new()?;

    let from_search = api.get_location_data(from).await?;
    let to_search = api.get_location_data(to).await?;

    let from = from_search
        .iter()
        .find(|x| x.name == from).cloned()
        .unwrap();

    let to = to_search
        .iter()
        .find(|x| x.name == to).cloned()
        .unwrap();

    let search_ids = api.perform_search_and_get_ids(&from, &to, date).await?;

    let target_url = format!("{}/services/booking/api/offer", VY_URL);
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
        search_ids
    );
    // let res = client
    //     .post(target_url)
    //     .json(&offer_body)
    //     .body(offer_body)
    //     .send()
    //     .await?;
    // let response_json = json::parse(res.text().await?.as_str()).unwrap();
    // println!("{:?}", response_json);

    Ok(())
}
