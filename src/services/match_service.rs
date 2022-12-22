//! This module defines the protocol of the match service
//! 
//! Map matching matches/snaps given GPS points to the road network in the most plausible way. 
//! Please note the request might result multiple sub-traces. Large jumps in the timestamps (> 60s) 
//! or improbable transitions lead to trace splits if a complete matching could not be found. 
//! The algorithm might not be able to match all points. Outliers are removed if they can not be 
//! matched successfully.

use displaythis::Display;
use serde::{Serialize, Deserialize};

use crate::*;

request!(MatchRequest (Service::Match) -> MatchResponse {
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
    overview: Option<OverviewRequest>,
    /// Timestamps for the input locations in seconds since UNIX epoch. Timestamps need to be monotonically increasing.
    #[builder(default, setter(into, strip_option))]
    timestamps: Option<Vec<u64>>,
    /// Allows the input track splitting based on huge timestamp gaps between points.
    #[builder(default, setter(into, strip_option))]
    gaps: Option<GapHandling>,
    /// Allows the input track modification to obtain better matching quality for noisy tracks.
    #[builder(default)]
    tidy: bool,
    /// Treats input coordinates indicated by given indices as waypoints in returned Match object. 
    /// Default is to treat all input coordinates as waypoints.
    #[builder(default, setter(into, strip_option))]
    waypoints: Option<Vec<usize>>
});

impl WithOptions for MatchRequest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];
        add_option!(          opts, steps,             self.steps);
        add_option!(opt       opts, annotations,       self.annotations);
        add_option!(opt       opts, geometries,        self.geometries);
        add_option!(opt       opts, overview,          self.overview);
        add_option!(opt multi opts, timestamps,        self.timestamps);
        add_option!(opt multi opts, radiuses,          self.radiuses);
        add_option!(opt       opts, gaps,              self.gaps);
        add_option!(          opts, tidy,              self.tidy);
        add_option!(opt multi opts, waypoints,         self.waypoints);
        opts
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResponse {
    /// Array of Waypoint objects representing all points of the trace in order. 
    /// If the trace point was ommited by map matching because it is an outlier, 
    /// the entry will be null.
    pub tracepoints: Vec<MatchingWaypoint>,
    /// An array of Route objects that assemble the trace. 
    pub matchings: Vec<MatchingRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingWaypoint {
    #[serde(flatten)]
    /// The actual waypoint data
    pub waypoint: Waypoint,
    /// Index to the Route object in matchings the sub-trace was matched to.
    pub matchings_index: usize,
    /// Index of the waypoint inside the matched route.
    pub waypoint_index: usize,
    /// Number of probable alternative matchings for this trace point. 
    /// A value of zero indicate that this point was matched unambiguously. 
    /// Split the trace at these points for incremental map matching.
    pub alternatives_count: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingRoute {
    #[serde(flatten)]
    /// The actual route data
    route: Route,
    /// Confidence of the matching. float value between 0 and 1. 1 is very confident that the matching is correct.
    confidence: f32,
}

/// Allows the input track splitting based on huge timestamp gaps between points.
#[derive(Debug, Clone, Copy, Display)]
pub enum GapHandling {
    #[display("split")]
    Split,
    #[display("ignore")]
    Ignore,
}
impl Default for GapHandling {
    fn default() -> Self { Self::Split }
}