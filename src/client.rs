use std::error::Error;

use bytes::Bytes;
use http::{request::Builder, Response};
use url::{ParseError, Url};

/// A trait representing basic rest client which communicates with a Semantic Scholar API endpoint.
pub trait BaseClient {
    /// The errors which may occur for this client.
    type Error: Error;

    /// Get the URL for the endpoint for the client.
    fn endpoint(&self, endpoint: &str) -> Result<Url, ParseError>;
}

/// A trait representing a client which can communicate with a Semantic Scholar API endpoint.
#[cfg(feature = "blocking")]
pub trait Client: BaseClient {
    /// Send a http request.
    fn send(&self, request: Builder, body: Vec<u8>) -> Result<Response<Bytes>, Self::Error>;
}

/// A trait representing a async client which can communicate with a Semantic Scholar API endpoint.
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncClient: BaseClient {
    /// Send a async http request.
    async fn send(&self, request: Builder, body: Vec<u8>) -> Result<Response<Bytes>, Self::Error>;
}
