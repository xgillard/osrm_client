//! This module describes the error handling related information
use displaythis::Display;
use serde::{Serialize, Deserialize};

/// This enumeration lists the problem that may arise when interacting with OSRM

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("protocol error {0}")]
    ProtocolError(OsrmStatus),
}


/// Every response object has a code property containing one of the strings 
/// below or a service dependent code:
#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum OsrmStatus {
    #[display("everything went ok")]
    Ok,
    #[display("url string is invalid")]
    InvalidUrl,
    #[display("service name is invalid")]
    InvalidService,
    #[display("version is not found")]
    InvalidVersion,
    #[display(" options are invalid")]
    InvalidOptions,
    #[display("the query string is synctactically malformed")]
    InvalidQuery,
    #[display("the successfully parsed query parameters are invalid")]
    InvalidValue,
    #[display("one of the supplied input coordinates could not snap to street segment")]
    NoSegment,
    #[display("the request size violates one of the service specific request size restrictions")]
    TooBig,
    
    // When a feature has not been implemented yet
    #[display("this request is not supported")]
    NotImplemented,

    // Emitted by the route service
    #[display("no route found")]
    NoRoute,

    // Emitted by the table service
    #[display("no route found")]
    NoTable,

    // Emitted by the match service
    #[display("No matchings found.")]
    NoMatch,

    // Emitted by the trip service
    #[display("No trips found because input coordinates are not connected.")]
    NoTrips,
}