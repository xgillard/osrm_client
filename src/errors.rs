//! This module describes the error handling related information

use std::fmt::Display;
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OsrmStatus {
    /// Everything went ok
    Ok,
    /// url string is invalid
    InvalidUrl,
    /// service name is invalid
    InvalidService,
    /// version is not found
    InvalidVersion,
    /// options are invalid
    InvalidOptions,
    /// the query string is synctactically malformed
    InvalidQuery,
    /// the successfully parsed query parameters are invalid
    InvalidValue,
    /// one of the supplied input coordinates could not snap to street segment
    NoSegment,
    /// the request size violates one of the service specific request size restrictions
    TooBig,
}
impl From<OsrmStatus> for &'static str {
    fn from(value: OsrmStatus) -> Self {
        match value {
            OsrmStatus::Ok             => "everything went ok",
            OsrmStatus::InvalidUrl     => "url string is invalid",
            OsrmStatus::InvalidService => "service name is invalid",
            OsrmStatus::InvalidVersion => "version is not found",
            OsrmStatus::InvalidOptions => "options are invalid",
            OsrmStatus::InvalidQuery   => "the query string is synctactically malformed",
            OsrmStatus::InvalidValue   => "the successfully parsed query parameters are invalid",
            OsrmStatus::NoSegment      => "one of the supplied input coordinates could not snap to street segment",
            OsrmStatus::TooBig         => "the request size violates one of the service specific request size restrictions",
        }
    }
}
impl Display for OsrmStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}
