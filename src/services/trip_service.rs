//! This module defines the protocol of the trip service
//! 
//! The trip plugin solves the Traveling Salesman Problem using a greedy heuristic 
//! (farthest-insertion algorithm) for 10 or more waypoints and uses brute force for 
//! less than 10 waypoints. The returned path does not have to be the fastest path. 
//! As TSP is NP-hard it only returns an approximation. Note that all input coordinates 
//! have to be connected for the trip service to work.
//! 
//! It is possible to explicitely set the start or end coordinate of the trip. 
//! When source is set to first, the first coordinate is used as start coordinate of the 
//! trip in the output. When destination is set to last, the last coordinate will be used 
//! as destination of the trip in the returned output. If you specify any, any of the 
//! coordinates can be used as the first or last coordinate in the output.
//! 
//! However, if source=any&destination=any the returned round-trip will still start at 
//! the first input coordinate by default.
//! 
//! Currently, not all combinations of roundtrip, source and destination are supported. 
//! Right now, the following combinations are possible:
//! roundtrip | source 	| destination | supported
//! ----------+---------+-------------+-----------
//! true 	  | first 	| last 	      |    yes
//! true 	  | first 	| any 	      |    yes
//! true 	  | any 	| last 	      |    yes
//! true 	  | any 	| any 	      |    yes
//! false     |	first 	| last 	      |    yes
//! false     |	first 	| any 	      |    no
//! false     |	any 	| last 	      |    no
//! false     |	any 	| any 	      |    no

use displaythis::Display;
use serde::{Serialize, Deserialize};

use crate::*;

request!(TripRequest (Service::Trip) -> TripResponse {
    /// Returned route is a roundtrip (route returns to first location)
    roundtrip: bool,
    /// Returned route starts at any or first coordinate (by default, any)
    source: Option<Source>,
    /// Returned route ends at any or last coordinate (by default, any)
    destination: Option<Destination>,
    /// Returned route steps for each route
    #[builder(default)]
    steps: bool,
    /// Returned route geometry format (influences overview and per step)
    #[builder(default, setter(into, strip_option))]
    geometries: Option<Geometries>,
    /// Returns additional metadata for each coordinate along the route geometry.
    #[builder(default, setter(into, strip_option))]
    annotations: Option<RouteAnnotationRequest>,
    /// Add overview geometry either full, simplified according to highest zoom level it could be display on, or not at all.
    #[builder(default, setter(into, strip_option))]
    overview: Option<OverviewRequest>
});

impl WithOptions for TripRequest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];
        add_option!(          opts, roundtrip,         self.roundtrip);
        add_option!(opt       opts, source,            self.source);
        add_option!(opt       opts, destination,       self.destination);
        add_option!(          opts, steps,             self.steps);
        add_option!(opt       opts, geometries,        self.geometries);
        add_option!(opt       opts, annotations,       self.annotations);
        add_option!(opt       opts, overview,          self.overview);
        opts
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripResponse {
    /// Array of Waypoint objects representing all waypoints in input order.
    waypoints: Vec<TripWaypoint>,
    /// An array of Route objects that assemble the trace
    trips: Vec<Route>,
}

/// Returned route starts at any or first coordinate (by default, any)
#[derive(Debug, Clone, Copy, Display)]
pub enum Source {
    #[display("first")]
    First,
    #[display("any")]
    Any
}
impl Default for Source {
    fn default() -> Self { Self::Any }
}

/// Returned route ends at any or first coordinate (by default, any)
#[derive(Debug, Clone, Copy, Display)]
pub enum Destination {
    #[display("last")]
    Last,
    #[display("any")]
    Any
}
impl Default for Destination {
    fn default() -> Self { Self::Any }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripWaypoint {
    #[serde(flatten)]
    /// The actual waypoint data
    pub waypoint: Waypoint,
    /// Index to trips of the sub-trip the point was matched to.
    pub trips_index: usize,
    /// Index of the point in the trip.
    pub waypoint_index: usize,
}