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
    let offers = api.get_offers_for_search(&search_ids).await?;


    Ok(())
}
