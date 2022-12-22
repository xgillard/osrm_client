use anyhow::Result;
use futures::future::try_join_all;
use osrm_client::{Location, Coordinates, TransportationMode, Client, RouteRequestBuilder, AlternativesRequest};


#[tokio::main]
async fn main() -> Result<()>{
    let name = nominatim::Client::new(
            nominatim::IdentificationMethod::UserAgent("osrm-client-example".to_string()));

    let places = locations(&name, &[
        "Place Sainte Barbe, 2 -- 1348 Louvain-la-Neuve",
        "Clos Chapelle-aux-Champs 43, 1200 Woluwe-Saint-Lambert",
        "Rue de la Loi 16, 1000 Bruxelles",
    ]).await?;
        
    let client = Client::default();
    
    let req = RouteRequestBuilder::default()
        .profile(TransportationMode::Car)
        .coordinates(Coordinates::Multi(places))
        .alternatives(AlternativesRequest::UpTo(3))
        .build()
        .unwrap();
    
    let rsp = req.send(&client).await.unwrap();
    println!("{rsp:#?}");
    Ok(())
}

async fn locations(nomin: &nominatim::Client, addr: &[&str]) -> Result<Vec<Location>> {
    let mut requests = vec![];
    for place in addr {
        requests.push(nomin.search(place));
    }
    let places = try_join_all(requests).await?;
    let mut result = vec![];
    for place in places {
        let place = &place[0];
        let longitude = place.lon.parse::<f32>().unwrap();
        let latitude = place.lat.parse::<f32>().unwrap();
        result.push(Location::new(longitude, latitude));
    }
    Ok(result)
}