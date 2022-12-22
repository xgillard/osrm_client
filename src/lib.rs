//! # OSRM-Client
//! 
//! The point of OSRM client is to provide a lightweight binding to the HTTP
//! services provided by the OSRM backend. The documentation of the original OSRM
//! API is available [here](https://project-osrm.org/docs/v5.24.0/api/#). 

mod errors;
mod data;
mod services;

pub use errors::*;
pub use data::*;
pub use services::*;