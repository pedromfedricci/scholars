use std::error::Error;

use bytes::Bytes;
use http::{header::HeaderName, HeaderValue};

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ClientError {
    #[error("`reqwest` error: {source}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("`http` error: {source}")]
    Http {
        #[from]
        source: http::Error,
    },
}

impl<E: Error> From<ClientError> for ApiError<E, ClientError> {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::Reqwest { .. } => Self::from_client(error),
            ClientError::Http { source } => Self::from_http(source),
        }
    }
}

/// Implements `BaseClient` for a type that will hold
/// a `ClientError` as the associated type for `BaseClient::Error`.
/// Implemented for both `reqwest::Client` and `reqwest::blocking::Client`.
macro_rules! base_client_impl {
    ($t:ty) => {
        use $crate::{client::BaseClient, reqwest::ClientError};

        impl BaseClient for $t {
            type Error = ClientError;

            fn endpoint(&self, endpoint: &str) -> Result<url::Url, url::ParseError> {
                url::Url::parse(endpoint)
            }
        }
    };
}

/// Helper function that tries to convert a [`http::Request`] into some type `T`.
/// This is used to convert [`http::Request`] into either async or blocking [`reqwest`] Request.
#[inline]
fn convert_from_http_request<T, R>(
    builder: http::request::Builder,
    body: T,
) -> Result<R, ClientError>
where
    R: TryFrom<http::Request<T>>,
    ClientError: From<<R as TryFrom<http::Request<T>>>::Error>,
{
    Ok(R::try_from(builder.body(body)?)?)
}

/// Helper function that constructs a [`http::Response`] from head parts.
/// This is used to convert either a async or blocking [`reqwest`] Response into a [`http::Response`].
#[inline]
fn convert_to_http_response<'a>(
    headers: impl IntoIterator<Item = (&'a HeaderName, &'a HeaderValue)>,
    status: http::StatusCode,
    version: http::Version,
) -> http::response::Builder {
    let mut rsp = http::Response::builder().status(status).version(version);
    if let Some(rsp_hdrs) = rsp.headers_mut() {
        for (name, value) in headers {
            rsp_hdrs.insert(name, value.clone());
        }
    }
    rsp
}

/// Implements [`crate::client::BaseClient`] and
/// [`crate::client::Client`] for [`reqwest::blocking::Client`].
#[cfg(feature = "reqwest-blocking")]
mod blocking {
    use super::*;
    use crate::client::Client;

    base_client_impl! { reqwest::blocking::Client }

    impl Client for reqwest::blocking::Client {
        fn send(
            &self,
            builder: http::request::Builder,
            body: Vec<u8>,
        ) -> Result<http::Response<Bytes>, Self::Error> {
            let rsp = self.execute(convert_from_http_request(builder, body)?)?;
            let http_rsp = convert_to_http_response(rsp.headers(), rsp.status(), rsp.version());
            Ok(http_rsp.body(rsp.bytes()?)?)
        }
    }
}

/// Implements [`crate::client::BaseClient`] and
/// [`crate::client::AsyncClient`] for [`reqwest::Client`].
#[cfg(feature = "reqwest-async")]
mod r#async {
    use super::*;
    use crate::client::AsyncClient;
    use async_trait::async_trait;

    base_client_impl! { reqwest::Client }

    #[async_trait]
    impl AsyncClient for reqwest::Client {
        async fn send(
            &self,
            builder: http::request::Builder,
            body: Vec<u8>,
        ) -> Result<http::Response<Bytes>, Self::Error> {
            let rsp = self.execute(convert_from_http_request(builder, body)?).await?;
            let http_rsp = convert_to_http_response(rsp.headers(), rsp.status(), rsp.version());
            Ok(http_rsp.body(rsp.bytes().await?)?)
        }
    }
}
