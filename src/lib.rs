#[macro_use]
mod static_url;

#[cfg(any(feature = "blocking", feature = "async"))]
pub mod client;
#[cfg(any(feature = "blocking", feature = "async"))]
pub mod endpoint;
#[cfg(any(feature = "blocking", feature = "async"))]
pub mod query;

pub mod error;
pub mod urlencoded;
