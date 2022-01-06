use std::{any, error::Error};

use http::StatusCode;
use url::Url;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError<E: Error, C: Error> {
    /// The Client returned an error.
    #[error("client error: {source}")]
    Client { source: C },

    /// The API returned an error object.
    #[error("response returned a error: {source}")]
    Response { source: E, status: StatusCode, url: String },

    /// The URL failed to parse.
    #[error("failed to parse url: {source}")]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    /// JSON deserialization from api failed.
    #[error("could not parse JSON response: {source}")]
    Json {
        #[from]
        source: serde_json::Error,
    },

    /// HTTP generic error.
    #[error("`http` error: {source}")]
    Http {
        #[from]
        source: http::Error,
    },

    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {typename} data from JSON: {source}")]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
}

impl<C: Error, E: Error> ApiError<E, C> {
    pub fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType { source, typename: any::type_name::<T>() }
    }

    /// Create an API error in a client error.
    pub fn from_client(source: C) -> Self {
        ApiError::Client { source }
    }

    /// Create an API error for a respnse error.
    pub fn from_response(source: E, status: StatusCode, url: Url) -> Self {
        ApiError::Response { source, status, url: url.to_string() }
    }
}
