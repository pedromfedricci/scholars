#[macro_use]
mod macros;

#[cfg(any(feature = "blocking", feature = "async"))]
pub mod client;
#[cfg(any(feature = "blocking", feature = "async"))]
pub mod endpoint;
#[cfg(any(feature = "blocking", feature = "async"))]
pub mod query;

pub mod error;
pub mod urlencoded;
