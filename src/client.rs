#[cfg(feature = "blocking")]
pub use blocking::Client;

#[cfg(feature = "async")]
pub use r#async::AsyncClient;

use std::error::Error;
use url::{ParseError, Url};

/// A trait representing basic rest client which communicates with a Semantic Scholar API endpoint.
pub trait BaseClient {
    /// The errors which may occur for this client.
    type Error: Error;

    /// Get the URL for the endpoint for the client.
    fn endpoint(&self, endpoint: &str) -> Result<Url, ParseError>;
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::BaseClient;
    use bytes::Bytes;
    use http::{request::Builder, Response};

    /// A trait representing a client which can communicate with a Semantic Scholar API endpoint.
    pub trait Client: BaseClient {
        /// Send a http request.
        fn send(&self, request: Builder, body: Vec<u8>) -> Result<Response<Bytes>, Self::Error>;
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::BaseClient;
    use async_trait::async_trait;
    use bytes::Bytes;
    use http::{request::Builder, Response};

    /// A trait representing a async client which can communicate with a Semantic Scholar API endpoint.
    #[async_trait]
    pub trait AsyncClient: BaseClient {
        /// Send an async http request.
        async fn send(
            &self,
            request: Builder,
            body: Vec<u8>,
        ) -> Result<Response<Bytes>, Self::Error>;
    }
}
