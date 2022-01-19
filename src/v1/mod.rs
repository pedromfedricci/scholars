#[cfg(any(feature = "blocking", feature = "async"))]
mod static_url;

pub mod definition;
pub mod error;
pub mod pagination;
pub mod parameter;
pub mod utils;
