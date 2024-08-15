use std::error::Error;

use vy_api::VyAPI;

mod consts;
mod destination;
mod duration;
mod journey;
mod offer;
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
        .find(|x| x.name == from)
        .cloned()
        .unwrap();

    let to = to_search.iter().find(|x| x.name == to).cloned().unwrap();

    let search_ids = api.perform_search_and_get_ids(&from, &to, date).await?;
    let offers = api.get_offers_for_search(&search_ids).await?;

    let mut offer_ids: Vec<String> = vec![];

    offers.iter().for_each(|elem| {
        // println!("{:?}", elem.get_offer_ids());
        offer_ids.append(&mut elem.get_offer_ids());
    });

    // println!("{:?}", offer_ids);
    let offer_id = offers.first().unwrap().get_id();
    let journey_taken = search_ids.iter().find(|j| *j.id() == offer_id).unwrap();
    println!("{:?}", journey_taken.from);

    let id = api.make_order(offer_ids.first().unwrap()).await?;
    println!("{}", id);

    api.get_available_seats(
        id,
        journey_taken.from.get_nsr_code(),
        journey_taken.to.get_nsr_code(),
    )
    .await?;

    Ok(())
}
