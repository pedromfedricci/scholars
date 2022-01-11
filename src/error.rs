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

    /// application/x-www-form-urlencoded serialization error.
    #[error("`serde_urlencoded` error: {source}")]
    UrlEncoded {
        #[from]
        source: serde_urlencoded::ser::Error,
    },

    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {typename} data from JSON: {source}")]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        /// This is meant for diagnostic only,
        /// see [`std::any::type_name`] for more information.
        typename: &'static str,
    },
}

impl<C: Error, E: Error> ApiError<E, C> {
    /// Create an [`ApiError`] from [`serde_json::Error`] when
    /// [`serde_json`] fails to deserialize JSON value.
    pub fn from_data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType { source, typename: any::type_name::<T>() }
    }

    /// Create an [`ApiError`] from a client error.
    pub fn from_client(source: C) -> Self {
        ApiError::Client { source }
    }

    /// Create an [`ApiError`] from a response error.
    pub fn from_response(source: E, status: StatusCode, url: Url) -> Self {
        ApiError::Response { source, status, url: url.to_string() }
    }

    /// Create an [`ApiError`] from a [`http::Error`].
    pub fn from_http(source: http::Error) -> Self {
        ApiError::Http { source }
    }

    /// Create an [`ApiError`] from a [`serde_json::Error`]
    pub fn from_json(source: serde_json::Error) -> Self {
        ApiError::Json { source }
    }
}
