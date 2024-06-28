//! This module defines the protocol of the route service
use displaythis::Display;
use serde::{Serialize, Deserialize};

use crate::{request, Service, WithOptions, Waypoint, Route, add_option, Geometries};

request!(RouteRequest (Service::Route) -> RouteResponse {
    /// Search for alternative routes. Passing a number alternatives=n searches for up to n alternative routes.
    /// Please note that even if alternative routes are requested, a result cannot be guaranteed.
    #[builder(default, setter(into, strip_option))]
    alternatives: Option<AlternativesRequest>,
    /// Returned route steps for each route leg
    #[builder(default)]
    steps: bool,
    /// Returns additional metadata for each coordinate along the route geometry.
    #[builder(default, setter(into, strip_option))]
    annotations: Option<RouteAnnotationRequest>,
    /// Returned route geometry format (influences overview and per step)
    #[builder(default, setter(into, strip_option))]
    geometries: Option<Geometries>,
    /// Add overview geometry either full, simplified according to highest zoom level it could be display on, or not at all.
    #[builder(default, setter(into, strip_option))]
    overview: Option<OverviewRequest>,
    /// Forces the route to keep going straight at waypoints constraining uturns there even if it would be faster. 
    /// Default value depends on the profile.
    #[builder(default)]
    continue_straight: bool,
    /// Treats input coordinates indicated by given indices as waypoints in returned Match object. 
    /// Default is to treat all input coordinates as waypoints.
    #[builder(default, setter(into, strip_option))]
    waypoints: Option<Vec<usize>>
});

impl WithOptions for RouteRequest {
    fn options(&self) -> Vec<(&'static str, String)> {
        let mut opts = vec![];

        add_option!(opt       opts, alternatives,      self.alternatives);
        add_option!(          opts, steps,             self.steps);
        add_option!(opt       opts, annotations,       self.annotations);
        add_option!(opt       opts, geometries,        self.geometries);
        add_option!(opt       opts, overview,          self.overview);
        add_option!(          opts, continue_straight, self.continue_straight);
        add_option!(opt multi opts, waypoints,         self.waypoints);

        opts
    }
}
#[derive(Debug, Display, Clone, Copy)]
pub enum AlternativesRequest {
    #[display("false")]
    NoAlternative,
    #[display("true")]
    AllAlternatives,
    #[display("{0}")]
    UpTo(usize)
}

#[derive(Debug, Display, Clone, Copy)]
pub enum RouteAnnotationRequest {
    #[display("false")]
    NoAnnotation,
    #[display("true")]
    AllAnnotations,
    #[display("nodes")]
    Nodes,
    #[display("distance")]
    Distance,
    #[display("duration")]
    Duration,
    #[display("datasources")]
    Datasources,
    #[display("weight")]
    Weight,
    #[display("speed")]
    Speed,
}
impl Default for RouteAnnotationRequest {
    fn default() -> Self { RouteAnnotationRequest::NoAnnotation }
}

#[derive(Debug, Display, Clone, Copy)]
pub enum OverviewRequest {
    #[display("false")]
    NoOverview,
    #[display("simplified")]
    Simplified,
    #[display("full")]
    Full,
}
impl Default for OverviewRequest {
    fn default() -> Self { Self::NoOverview }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    /// Array of Waypoint objects representing all waypoints in order:
    pub waypoints: Vec<Waypoint>,
    /// An array of Route objects, ordered by descending recommendation rank
    pub routes: Vec<Route>,
}

#[cfg(test)]
mod test {
    use crate::RouteResponse;

    #[test]
    fn parse_response() {
        let text = r#"
        {"code":"Ok",
        "routes":[
           {"geometry":"slluHq`qZ~eChbDtcFfzCpzAulD~vBsfAbh@}j@|cAs~CxpCkoDtuA}sE|f@wcAxiAi{@nbB{n@jMd_@bk@i]xCvLyL|GjH`O",
           "legs":[
               {"steps":[], 
                "summary":"",
                "weight":1519.3,
                "duration":1498.1,
                "distance":28139.9
                }
             ],
           "weight_name":"routability",
           "weight":1519.3,
           "duration":1498.1,
           "distance":28139.9
           }
         ],
         "waypoints":[
             {"hint":"-0eQgNlS0oMEAAAAEwAAACwAAAA8AAAAGJhDQDdcQEH8uOtBodYgQgQAAAATAAAALAAAADwAAAAJ9AAA--hEAIAMCAOZ6EQAnQwIAwEAzwxXg-vq",
             "distance":7.615206,
             "name":"Jagersstraat",
             "location":[4.516091,50.859136]
             },
             {"hint":"NbnigxYXmIlLAAAAAAAAAEoAAAAAAAAATZl7QQAAAAAYEHZBAAAAACYAAAAAAAAAJQAAAAAAAAAJ9AAA74JGACkkBQNwhkYA8iIFAwEAbwVXg-vq",
             "distance":72.232413,
             "name":"Voie Minckelers",
             "location":[4.621039,50.668585]
             }
         ]
       }   
        "#;
        let response = serde_json::from_str::<RouteResponse>(text);
        assert!(response.is_ok());
    }
}