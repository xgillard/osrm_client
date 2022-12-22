//! This module defines the types required to implement the 'nearest' service

use serde::{Serialize, Deserialize};

use crate::{request, Service, WithOptions, Waypoint};

request!(Nearest (Service::Nearest) -> NearestResponse {
    number: Option<usize>
});

impl WithOptions for Nearest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];

        if let Some(number) = self.number {
            opts.push(("number", format!("{number}")));
        }

        opts
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearestResponse {
    waypoints: Vec<Waypoint>
}