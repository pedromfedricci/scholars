#[cfg(any(feature = "blocking", feature = "async"))]
pub mod client;

pub mod error;
pub mod urlencoded;
