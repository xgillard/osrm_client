//! This module describes the protocol for the Table service
//! 
//! Computes the duration of the fastest route between all pairs of supplied coordinates. 
//! Returns the durations or distances or both between the coordinate pairs. 
//! Note that the distances are not the shortest distance between two coordinates,
//! but rather the distances of the fastest routes. Duration is in seconds and 
//! distances is in meters.
use displaythis::Display;
use serde::{Serialize, Deserialize};

use crate::*;

request!(TableRequest (Service::Table) -> TableResponse {
    /// Use location with given index as source. 
    /// (By default, all coordinates are used as sources).
    #[builder(default, setter(into, strip_option))]
    sources: Option<Vec<usize>>,
    /// Use location with given index as destinations. 
    /// (By default, all coordinates are used as destinations).
    #[builder(default, setter(into, strip_option))]
    destinations: Option<Vec<usize>>,
    /// Return the requested table or tables in response.
    #[builder(default, setter(into, strip_option))]
    annotations: Option<TableAnnotationRequest>,
    /// If no route found between a source/destination pair, 
    /// calculate the as-the-crow-flies distance, then use this speed to estimate duration.
    #[builder(default, setter(into, strip_option))]
    fallback_speed: Option<f32>,
    /// When using a fallback_speed, use the user-supplied coordinate ( input ), 
    /// or the snapped location ( snapped ) for calculating distances.
    #[builder(default, setter(into, strip_option))]
    fallback_coordinate: Option<FallbackCoordinateRequest>,
    #[builder(default, setter(into, strip_option))]
    /// Use in conjunction with annotations=durations . Scales the table duration values by this number.
    scale_factor: Option<f32>
});

impl WithOptions for TableRequest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];
        add_option!(opt multi opts, sources,             self.sources);
        add_option!(opt multi opts, destinations,        self.destinations);
        add_option!(opt       opts, annotations,         self.annotations);
        add_option!(opt       opts, fallback_speed,      self.fallback_speed);
        add_option!(opt       opts, fallback_coordinate, self.fallback_coordinate);
        add_option!(opt       opts, scale_factor,        self.scale_factor);
        opts
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableResponse {
    /// array of arrays that stores the matrix in row-major order. durations[i][j] gives the travel
    /// time from the i-th source to the j-th destination. Values are given in seconds. 
    /// Can be null if no route between i and j can be found.
    pub durations: Option<Vec<Vec<Option<f32>>>>,
    /// array of arrays that stores the matrix in row-major order. distances[i][j] gives the travel 
    /// distance from the i-th source to the j-th destination. Values are given in meters. 
    /// Can be null if no route between i and j can be found.
    pub distances: Option<Vec<Vec<Option<f32>>>>,
    /// array of Waypoint objects describing all sources in order
    pub sources: Vec<Waypoint>,
    /// array of Waypoint objects describing all destinations in order
    pub destinations: Vec<Waypoint>,
    /// (optional) array of arrays containing i,j pairs indicating which cells contain estimated 
    /// values based on fallback_speed. Will be absent if fallback_speed is not used.
    pub fallback_speed_cells: Option<Vec<(usize, usize)>>,
}


#[derive(Debug, Display, Clone, Copy)]
pub enum TableAnnotationRequest {
    #[display("distance")]
    Distance,
    #[display("duration")]
    Duration,
    #[display("duration,distance")]
    Both,
}
impl Default for TableAnnotationRequest {
    fn default() -> Self { TableAnnotationRequest::Duration }
}

/// When using a fallback_speed , use the user-supplied coordinate ( input ), 
/// or the snapped location ( snapped ) for calculating distances.
#[derive(Debug, Display, Clone, Copy)]
pub enum FallbackCoordinateRequest {
    /// Use the user supplied coordinate for calculating distances
    #[display("input")]
    UserSupplied,
    /// Use the snapped location for calculating distances.
    #[display("snapped")]
    Snapped,
}
impl Default for FallbackCoordinateRequest {
    fn default() -> Self { FallbackCoordinateRequest::UserSupplied }
}