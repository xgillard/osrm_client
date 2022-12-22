use osrm_client::{NearestBuilder, Location, Coordinates, Profile, Client};


#[tokio::main]
async fn main() -> Result<(), osrm_client::Error>{
    let client = Client::default();
    
    let req = NearestBuilder::default()
        .profile(Profile::Car)
        .coordinates(Coordinates::Single(Location::new(4.35, 50.8333)))
        .number(None)
        .build()
        .unwrap();
    
    let rsp = req.send(&client).await.unwrap();

    println!("{rsp:#?}");
    Ok(())
}