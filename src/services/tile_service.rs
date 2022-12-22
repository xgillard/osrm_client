//! This module defines the protocol of the trip service
//! 
//! This service generates Mapbox Vector Tiles that can be viewed with a vector-tile 
//! capable slippy-map viewer. The tiles contain road geometries and metadata that
//!  can be used to examine the routing graph. The tiles are generated directly from 
//! the data in-memory, so are in sync with actual routing results, and let you 
//! examine which roads are actually routable, and what weights they have applied.
//! The x, y, and zoom values are the same as described at 
//! https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames, and are supported by 
//! vector tile viewers like Mapbox GL JS.
//! 
//! The response object is either a binary encoded blob with a Content-Type of 
//! application/x-protobuf, or a 404 error. Note that OSRM is hard-coded to only 
//! return tiles from zoom level 12 and higher (to avoid accidentally returning 
//! extremely large vector tiles).
//! 
//! Vector tiles contain two layers:
//! 
//! ## `speeds` layer:
//! 
//! Property      | Type     | Description
//! --------------+----------+-----------------------------------------------
//! speed         |  integer | the speed on that road segment, in km/h
//! is_small      |  boolean | whether this segment belongs to a small (< 1000 node) strongly connected component
//! datasource    |  string  | the source for the speed value (normally lua profile unless you're using the traffic update feature , in which case it contains the stem of the filename that supplied the speed value for this segment
//! duration      |  float   | how long this segment takes to traverse, in seconds. This value is to calculate the total route ETA.
//! weight        |  integer | how long this segment takes to traverse, in units (may differ from duration when artificial biasing is applied in the Lua profiles). ACTUAL ROUTING USES THIS VALUE.
//! name          |  string  | the name of the road this segment belongs to
//! rate          |  float   | the value of length/weight - analagous to speed , but using the weight value rather than duration , rounded to the nearest integer
//! is_startpoint |  boolean | whether this segment can be used as a start/endpoint for routes
//! 
//! ## `turns` layer:
//! Property      |  Type    |  Description
//! --------------+----------+-----------------------------------------------
//! bearing_in    |  integer | the absolute bearing that approaches the intersection. -180 to +180, 0 = North, 90 = East
//! turn_angle    |  integer | the angle of the turn, relative to the bearing_in . -180 to +180, 0 = straight ahead, 90 = 90-degrees to the right
//! cost          |  float   | the time we think it takes to make that turn, in seconds. May be negative, depending on how the data model is constructed (some turns get a "bonus").
//! weight        |  float   | the weight we think it takes to make that turn. May be negative, depending on how the data model is constructed (some turns get a "bonus"). ACTUAL ROUTING USES THIS VALUE
//! type          |  string  | the type of this turn - values like turn , continue , etc. See the StepManeuver for a partial list, this field also exposes internal turn types that are never returned with an API response
//! modifier      |  string  | the direction modifier of the turn ( left , sharp left , etc)
//!
use bytes::Bytes;

use derive_builder::Builder;
use crate::*;

#[derive(Debug, Clone, Builder)]
pub struct TileRequest {
    /// Mode of transportation
    #[builder(default="crate::TransportationMode::Car")]
    profile: crate::TransportationMode,
    /// X goes from 0 (left edge is 180 °W) to 2zoom − 1 (right edge is 180 °E)
    x: f32,
    /// Y goes from 0 (top edge is 85.0511 °N) to 2zoom − 1 (bottom edge is 85.0511 °S) in a Mercator projection
    y: f32,
    /// The zoom parameter is an integer between 0 (zoomed out) and 18 (zoomed in). 
    /// 18 is normally the maximum, but some tile servers might go beyond that.
    zoom: usize
}

impl TileRequest {
    pub async fn send(&self, client: &crate::Client) -> Result<Bytes, crate::Error> {
        let response = client.reqwest.get(self.url(client))
            .send()
            .await?
            .bytes()
            .await?;
        Ok(response)
    }
    pub async fn debug(&self, client: &crate::Client) -> Result<String, crate::Error> {
        let response = client.reqwest.get(self.url(client))
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub fn url(&self, client: &crate::Client) -> String {
        let base    = &client.base_url;
        let version = &client.version;
        let service = Service::Tile;
        let profile = self.profile;
        let x = self.x;
        let y = self.y;
        let zoom = self.zoom;

        format!("{base}/{service}/{version}/{profile}/tile({x},{y},{zoom}).mvt")
    }

    pub fn show_url(&self) -> String {
        let zoom = self.zoom;
        let latitude = self.x;
        let longitude = self.y;
        format!("http://map.project-osrm.org/debug/#{zoom}/{latitude}/{longitude}")
    }
    
}