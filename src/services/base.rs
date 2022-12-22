//! This module defines the base functionalities for the ORSM client.
//! These functionalities are common to all services offered through the API.

use std::fmt::Display;
use serde::Deserialize;

use crate::{OsrmStatus, Error};

/// The default URL to use as base for the interaction with OSRM
const OSRM_BASE: &str = "http://router.project-osrm.org";
/// The default version of the API which is used
const OSRM_VERSION: &str = "v1";

/// This is the client you will use to connect to the HTTP service of your 
/// ORSM backend. 
#[derive(Debug, Clone)]
pub struct Client {
    /// Under the hood, this client delegates the bulk of the work to reqwest
    /// to perform all the http interactions.
    pub(crate) reqwest: reqwest::Client,
    /// This is the base URL of the OSRM instance you will connect to. By
    /// default, this value is going to be "http://router.project-osrm.org";
    pub(crate) base_url: String,
    /// The version of the API (so far, only v1 is supported)
    pub(crate) version: String,
}

impl Default for Client {
    fn default() -> Self {
        Self { 
            reqwest:  Default::default(), 
            base_url: OSRM_BASE.to_string(),
            version:  OSRM_VERSION.to_string(),
        }
    }
}

pub trait Request : WithOptions {}
pub trait WithOptions {
    fn options(&self) -> Vec<(&'static str, String)>;
}

macro_rules! request {
    ($name:ident ($service:expr) -> $response:ty { $($fi:ident : $ft:ty),* }) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, derive_builder::Builder)]
        pub struct $name {
            // -------------------------------------------------------
            // ---  STUFFS THAT ARE COMMON TO ALL REQUESTS -----------
            // -------------------------------------------------------
            /// Mode of transportation
            #[builder(default="crate::Profile::Car")]
            profile: crate::Profile,
            /// Coordinates the request bears on
            coordinates: crate::Coordinates,
            // -------------------------------------------------------
            // ---  GENERAL OPTIONS ----------------------------------
            // -------------------------------------------------------
            /// Limits the search to segments with given bearing in degrees towards true north in clockwise direction.
            #[builder(default, setter(into, strip_option))]
            bearings: Option<Vec<crate::BearingRequest>>,
            /// Limits the search to given radius in meters.
            #[builder(default, setter(into, strip_option))]
            radiuses: Option<Vec<crate::Radius>>,
            /// Adds a Hint to the response which can be used in subsequent requests, see hints parameter.
            #[builder(default="true")]
            generate_hints: bool, 
            /// Hint from previous request to derive position in street network.
            #[builder(default, setter(into, strip_option))]
            hints: Option<Vec<crate::Hint>>,
            /// Keep waypoints on curb side.
            #[builder(default, setter(into, strip_option))]
            approaches: Option<Vec<crate::Approach>>,
            /// Additive list of classes to avoid, order does not matter
            #[builder(default, setter(into, strip_option))]
            exclude: Option<Vec<String>>,
            /// Default snapping avoids is_startpoint (see profile) edges, any will snap to any edge in the graph
            #[builder(default, setter(into, strip_option))]
            snapping: Option<crate::Snapping>,
            /// Removes waypoints from the response. Waypoints are still calculated, but not serialized. 
            /// Could be useful in case you are interested in some other part of response and do not want to transfer waste data.
            #[builder(default="false")]
            skip_waypoints: bool,
            // -------------------------------------------------------
            // ---  SERVICE SPECIFIC OPTIONS -------------------------
            // -------------------------------------------------------
            $($fi : $ft),*
        }

        impl crate::Request for $name {}

        impl $name {
            pub async fn send(&self, client: &crate::Client) -> Result<$response, crate::Error> {
                let mut options = self.options();
                self.add_general_options(&mut options);

                client.reqwest.get(self.url(client))
                    .query(&options)
                    .send()
                    .await?
                    .json::<crate::Response<$response>>()
                    .await?
                    .into()
            }

            fn url(&self, client: &crate::Client) -> String {
                let base    = &client.base_url;
                let version = &client.version;
                let service = $service;
                let profile = self.profile;
                let coord   = &self.coordinates;

                format!("{base}/{service}/{version}/{profile}/{coord}")
            }

            fn add_general_options(&self, options: &mut Vec<(&'static str, String)>) {
                if let Some(bearings) = self.bearings.as_ref() {
                    options.push(("bearings", crate::multi(bearings)));
                }
                if let Some(radiuses) = self.radiuses.as_ref() {
                    options.push(("radiuses", crate::multi(radiuses)));
                }
                
                options.push(("generate_hints", format!("{}", self.generate_hints)));
                if let Some(hints) = self.hints.as_ref() {
                    options.push(("hints", crate::multi(hints)));
                }
                if let Some(approaches) = self.approaches.as_ref() {
                    options.push(("approaches", crate::multi(approaches)));
                }
                if let Some(exclude) = self.exclude.as_ref() {
                    options.push(("exclude", crate::multi(exclude)));
                }
                if let Some(snapping) = self.snapping.as_ref() {
                    options.push(("snapping", format!("{snapping}")));
                }
                options.push(("skip_waypoints", format!("{}", self.skip_waypoints)));
            }
        }
    };
}
pub(crate) use request;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    code: OsrmStatus,
    pub message: Option<String>,
    pub data_version: Option<String>,
    #[serde(flatten)]
    data: T
}
impl <T> From<Response<T>> for Result<T, Error> {
    fn from(value: Response<T>) -> Self {
        match value.code {
            OsrmStatus::Ok => Ok(value.data),
            _ => Err(Error::ProtocolError(value.code))
        }
    }
}

pub(crate) fn multi(xs: &[impl Display]) -> String {
    let mut out = String::new();
    for (i, x) in xs.iter().enumerate() {
        if i == 0 {
            out.push_str(&format!("{x}"));
        } else {
            out.push_str(&format!(";{x}"));
        }
    }
    out
}