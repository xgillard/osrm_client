//! This module comprises a representation of the data that is shared accross
//! all OSRM services.

use std::fmt::Display;

use serde::{Serialize, Deserialize};

/// The location of a point anywhere on earth. The order of the fields is
/// longitude, latitude
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location{
    pub longitude: f32, 
    pub latitude: f32
}
impl Location {
    pub fn new(longitude: f32, latitude: f32) -> Self {
        Self { longitude, latitude }
    }
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.longitude, self.latitude)
    }
}

/// Most services are quite flexible wrt the coordinates they accept:
/// it can either be a single coord, a sequence of coord separated by semicolon,
/// or a polyline (follows Google polyline format) or polyline with precision of 6.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Coordinates {
    /// One single coordinate
    Single(Location),
    /// A sequence of coordinates in the longitude, latitude form
    Multi(Vec<Location>),
    /// A polyline formatted according to Google polyline format (precision 5)
    Polyline(String),
    /// A polyline formatted according to Google polyline format (precision 6)
    Polyline6(String),
}
impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(c) => write!(f, "{c}"),
            Self::Multi(coord) => {
                for (i, c) in coord.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{c}")?;
                    } else {
                        write!(f, ";{c}")?;
                    }
                }
                Ok(())
            },
            Self::Polyline(s) => write!(f, "polyline({s})"),
            Self::Polyline6(s) => write!(f, "polyline6({s})"),
        }
    }
}

/// Hint from previous request to derive position in street network (base64 encoded)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint(String);
impl Display for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Object used to describe waypoint on a route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    /// Name of the street the coordinate snapped to
    pub name: String,
    /// the [longitude, latitude] pair of the snapped coordinate
    pub location: Location,
    /// The distance, in metres, from the input coordinate to the snapped coordinate
    pub distance: f32, 
    /// Unique internal identifier of the segment (ephemeral, not constant over data 
    /// updates) This can be used on subsequent request to significantly speed up the 
    /// query and to connect multiple services. E.g. you can use the hint value obtained 
    /// by the nearest query as hint values for route inputs.
    pub hint: Hint,
}