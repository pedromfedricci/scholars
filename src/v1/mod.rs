#[cfg(any(feature = "blocking", feature = "async"))]
mod static_url;

#[cfg(any(feature = "blocking", feature = "async"))]
pub mod endpoint;

pub mod definition;
pub mod error;
pub mod pagination;
pub mod parameter;
pub mod query_params;
pub mod utils;
