//! This module describes the general options that are applicable to all requests
//! of all services in OSRM.

use std::fmt::Display;
use serde::{Serialize, Deserialize};

/// Which is the service being used
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Service {
    /// Finds the fastest route between coordinates
    Route, 
    /// Snaps coordinates to the street network and returns the nearest matches
    Nearest,
    /// Computes the duration or distances of the fastest route between all pairs of supplied coordinates
    Table,
    /// Snaps noisy GPS traces to the road network in the most plausible way
    Match,
    /// Solves the Traveling Salesman Problem using a greedy heuristic
    Trip, 
    /// Generates Mapbox Vector Tiles with internal routing metadata
    Tile,
}
impl From<Service> for &'static str {
    fn from(value: Service) -> Self {
        match value {
            Service::Route   => "route",
            Service::Nearest => "nearest",
            Service::Table   => "table",
            Service::Match   => "match",
            Service::Trip    => "trip",
            Service::Tile    => "tile",
        }
    }
}
impl Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}


/// Mode of transportation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Profile {
    /// Traveling by driving a car
    Car, 
    /// Traveling by riding a bike
    Bike, 
    /// Traveling on bare foot
    Foot,
}
impl From<Profile> for &'static str {
    fn from(value: Profile) -> Self {
        match value {
            Profile::Car  => "car",
            Profile::Bike => "bike",
            Profile::Foot => "foot",

        }
    }
}
impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}


/// Limits the search to segments with given bearing in degrees towards true north in clockwise direction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BearingRequest {
    /// A value in the range 0..360
    pub value: u16,
    /// A value in the range 0..180
    pub range: u16
}
impl Display for BearingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.value, self.range)
    }
}

/// Limits the search to given radius in meters.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Radius {
    /// The default value
    Unlimited,
    /// Limits the search to a radius of ? meters >= 0
    Limited(f64),
}
impl Default for Radius {
    fn default() -> Self { Self::Unlimited }
}
impl Display for Radius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unlimited => f.write_str("unlimited"),
            Self::Limited(x)=> write!(f, "{x}"),
        }
    }
}

/// Keep waypoints on curb side
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Approach {
    Unrestricted, 
    Curb, 
}
impl Default for Approach {
    fn default() -> Self { Self::Unrestricted }
}
impl From<Approach> for &'static str {
    fn from(value: Approach) -> Self {
        match value {
            Approach::Unrestricted => "unrestricted",
            Approach::Curb         => "curb",
        }
    }
}
impl Display for Approach {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}

/// Default snapping avoids is_startpoint (see profile) edges, any will snap to any edge in the graph
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Snapping {
    Default, 
    Any, 
}
impl Default for Snapping {
    fn default() -> Self { Self::Default }
}
impl From<Snapping> for &'static str {
    fn from(value: Snapping) -> Self {
        match value {
            Snapping::Default => "default",
            Snapping::Any     => "any",
        }
    }
}
impl Display for Snapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}
