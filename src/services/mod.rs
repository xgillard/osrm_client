//! This module actually implements the logic to go fetch the appropriate data
//! and interpret it on the way back.

mod base;

mod nearest_service;
mod route_service;
mod table_service;
mod match_service;
mod trip_service;
mod tile_service;

pub use base::*;
pub use nearest_service::*;
pub use route_service::*;
pub use table_service::*;
pub use match_service::*;
pub use trip_service::*;
pub use tile_service::*;