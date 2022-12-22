use osrm_client::{NearestRequestBuilder, Location, Coordinates, TransportationMode, Client};


#[tokio::main]
async fn main() -> Result<(), osrm_client::Error>{
    let client = Client::default();
    
    let req = NearestRequestBuilder::default()
        .profile(TransportationMode::Foot)
        .coordinates(Coordinates::Single(Location::new(2.290253, 48.8583701)))
        .build()
        .unwrap();
    
    let rsp = req.send(&client).await.unwrap();

    println!("{rsp:#?}");
    Ok(())
}