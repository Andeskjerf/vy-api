use std::error::Error;

use vy_api::VyAPI;

mod cart;
mod consts;
mod destination;
mod duration;
mod journey;
mod offer;
mod seat;
mod vy_api;
mod position;
mod external_reference;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let from = "Oslo S";
    let to = "Bergen stasjon";
    let date = "2024-08-19T04:00:00.000Z";

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
        offer_ids.append(&mut elem.get_offer_ids());
    });

    let offer_id = offers.first().unwrap().get_id();
    let journey_taken = search_ids.iter().find(|j| *j.id() == offer_id).unwrap();

    let id = api.make_order(offer_ids.first().unwrap()).await?;

    let carts = api
        .get_available_seats(
            id.clone(),
            journey_taken.from.get_nsr_code(),
            journey_taken.to.get_nsr_code(),
        )
        .await?;

    println!("carts: {}", carts.len());
    let mut available = 0;
    carts.iter().for_each(|c| {
        println!("seats in cart {}: {}", c.id, c.seats.len());
        available += c.seats.iter().filter(|e| e.available).count();
    });

    println!("total available seats: {}", available);

    api.delete_order(&id).await?;

    Ok(())
}
