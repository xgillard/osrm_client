//! This module defines the types required to implement the 'nearest' service
//!
//! Snaps a coordinate to the street network and returns the nearest n matches.
//! Where coordinates only supports a single {longitude},{latitude} entry

use serde::{Serialize, Deserialize};

use crate::{request, Service, WithOptions, Waypoint, add_option};

request!(NearestRequest (Service::Nearest) -> NearestResponse {
    /// Number of nearest segments that should be returned
    #[builder(default, setter(into, strip_option))]
    number: Option<usize>
});

impl WithOptions for NearestRequest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];

        add_option!(opt opts, number, self.number);

        opts
    }
}

/// As waypoints is a single thing, returned by that service, using it with option 
/// skip_waypoints set to true is quite useless, but still possible. 
/// In that case only code field will be returned (which would mean an empty response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearestResponse {
    /// array of Waypoint objects sorted by distance to the input coordinate. 
    /// Each object has at least the following additional properties
    pub waypoints: Option<Vec<Waypoint>>
}