#![deny(rust_2018_idioms)]

#[cfg(any(feature = "blocking", feature = "async"))]
#[macro_use]
mod static_url;

#[cfg(feature = "__v")]
pub(crate) mod serialize;
#[cfg(feature = "__v")]
pub(crate) mod urlencoded;

#[cfg(feature = "v1")]
pub mod v1;

#[cfg(any(feature = "blocking", feature = "async"))]
pub mod client;
#[cfg(any(feature = "blocking", feature = "async"))]
pub(crate) mod endpoint;
#[cfg(any(feature = "blocking", feature = "async"))]
pub(crate) mod query;

#[cfg(any(feature = "reqwest-async", feature = "reqwest-blocking"))]
pub mod reqwest;

pub mod error;
