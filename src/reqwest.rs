use std::error::Error;

use bytes::Bytes;

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
/// a `ClientError` as the associated type for named `Error`.
/// Implemented for both `reqwest::Client` and `reqwest::blocking::Client`.
macro_rules! base_client_impl {
    ($t:ty) => {
        use $crate::client::BaseClient;

        impl BaseClient for $t {
            type Error = ClientError;

            fn endpoint(&self, endpoint: &str) -> Result<url::Url, url::ParseError> {
                url::Url::parse(endpoint)
            }
        }
    };
}

/// Converts either a `reqwest::Request` or
/// `reqwest::blocking::Request` into a `http::Request`.
/// Any error encountered is simply bubbled up.
macro_rules! convert_request_type {
    ($reqwest_req:ident, $body:ident) => {{
        let http_req = $reqwest_req.body($body)?;
        http_req.try_into()?
    }};
}

/// Converts either a `reqwest::Response` or
/// `reqwest::blocking::Response` into a `http::Response`.
macro_rules! convert_response_type {
    ($reqwest_rsp:ident) => {{
        let mut http_rsp =
            http::Response::builder().status($reqwest_rsp.status()).version($reqwest_rsp.version());
        let headers = http_rsp.headers_mut();
        if let Some(headers) = headers {
            for (key, value) in $reqwest_rsp.headers() {
                headers.insert(key, value.clone());
            }
        }
        http_rsp
    }};
}

/// Implements [`BaseClient`] and [`Client`] for [`reqwest::blocking::Client`].
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
            let reqwest_rsp = {
                let reqwest_req = convert_request_type!(builder, body);
                self.execute(reqwest_req)?
            };
            let http_rsp = convert_response_type!(reqwest_rsp);
            Ok(http_rsp.body(reqwest_rsp.bytes()?)?)
        }
    }
}

/// Implements [`BaseClient`] and [`AsyncClient`] for [`reqwest::Client`].
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
            let reqwest_rsp = {
                let reqwest_req = convert_request_type!(builder, body);
                self.execute(reqwest_req).await?
            };
            let http_rsp = convert_response_type!(reqwest_rsp);
            Ok(http_rsp.body(reqwest_rsp.bytes().await?)?)
        }
    }
}
