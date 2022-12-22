use std::{fs::File, io::{BufWriter, Write}};

use anyhow::{Result, Ok};
use osrm_client::{Client, TileRequestBuilder};


#[tokio::main]
async fn main() -> Result<()>{
    let client = Client::default();
    let (x, y) = (1310.0, 3166.0);
    let req = TileRequestBuilder::default()
        .x(x)
        .y(y)
        .zoom(13)
        .build()?;
    
    let rsp = req.send(&client).await?.to_vec();
    
    let mut out = BufWriter::new(File::create("tile.mvt")?);
    out.write_all(&rsp)?;

    Ok(())
}