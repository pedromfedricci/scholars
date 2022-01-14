#[cfg(any(feature = "blocking", feature = "async"))]
#[macro_use]
mod static_url;

pub mod definition;
pub mod error;
