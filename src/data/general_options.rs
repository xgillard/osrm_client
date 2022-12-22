//! This module describes the general options that are applicable to all requests
//! of all services in OSRM.

use displaythis::Display;
use serde::{Serialize, Deserialize};

/// Which is the service being used
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum Service {
    /// Finds the fastest route between coordinates
    #[display("route")]
    Route, 
    /// Snaps coordinates to the street network and returns the nearest matches
    #[display("nearest")]
    Nearest,
    /// Computes the duration or distances of the fastest route between all pairs of supplied coordinates
    #[display("table")]
    Table,
    /// Snaps noisy GPS traces to the road network in the most plausible way
    #[display("match")]
    Match,
    /// Solves the Traveling Salesman Problem using a greedy heuristic
    #[display("trip")]
    Trip, 
    /// Generates Mapbox Vector Tiles with internal routing metadata
    #[display("tile")]
    Tile,
}

/// Limits the search to segments with given bearing in degrees towards true north in clockwise direction.
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
#[display("{value},{range}")]
pub struct BearingRequest {
    /// A value in the range 0..360
    pub value: u16,
    /// A value in the range 0..180
    pub range: u16
}

/// Limits the search to given radius in meters.
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum Radius {
    /// The default value
    #[display("unlimited")]
    Unlimited,
    /// Limits the search to a radius of ? meters >= 0
    #[display("{0}")]
    Limited(f64),
}
impl Default for Radius {
    fn default() -> Self { Self::Unlimited }
}

/// Keep waypoints on curb side
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum Approach {
    #[display(""unrestricted)]
    Unrestricted, 
    #[display("curb")]
    Curb, 
}
impl Default for Approach {
    fn default() -> Self { Self::Unrestricted }
}

/// Default snapping avoids is_startpoint (see profile) edges, any will snap to any edge in the graph
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum Snapping {
    #[display("default")]
    Default, 
    #[display("any")]
    Any, 
}
impl Default for Snapping {
    fn default() -> Self { Self::Default }
}