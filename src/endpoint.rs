use std::error::Error;

use bytes::Bytes;
use http::{request::Builder, Method, Request, Response, Uri};
use serde::de::DeserializeOwned;
use serde_urlencoded::ser::Error as UrlEncodedError;
use url::Url;

use crate::{client::BaseClient, error::ApiError, urlencoded::UrlEncodedQuery};

/// A trait for providing the necessary information for a single REST API endpoint.
pub(crate) trait Endpoint {
    /// An error type that can be returned by the endpoint.
    type Error: Error;

    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;

    /// The path to the endpoint.
    fn endpoint(&self) -> &str;

    /// URL query string for the endpoint.
    fn query_params(&self) -> Result<UrlEncodedQuery<'_>, UrlEncodedError>;
}

pub(in crate) type EndpointError<E, C> = ApiError<<E as Endpoint>::Error, <C as BaseClient>::Error>;

pub(in crate) type EndpointResult<T, E, C> = Result<T, EndpointError<E, C>>;

/// Converts [`url::Url`] into [`http::Uri`].
#[inline]
fn url_to_http_uri(url: &Url) -> Uri {
    url.as_str().parse().expect("a parsed `Url` must be a valid `Uri`")
}

/// Gets the Endpoint's [`Url`] and creates the request [`Builder`].
#[inline]
fn build_request<E: Endpoint, C: BaseClient>(
    endpoint: &E,
    client: &C,
) -> Result<(Builder, Url), EndpointError<E, C>> {
    let mut url = client.endpoint(endpoint.endpoint())?;
    endpoint.query_params()?.set_url(&mut url);
    log::debug!("querying Semantic Scholar API at {}", url.as_str());
    let builder = Request::builder().method(endpoint.method()).uri(url_to_http_uri(&url));
    Ok((builder, url))
}

/// Serializes the JSON payload.
#[inline]
fn serialize_response<T, E: Endpoint, C: BaseClient>(
    rsp: Response<Bytes>,
    url: Url,
) -> EndpointResult<T, E, C>
where
    T: DeserializeOwned,
    E::Error: DeserializeOwned,
    EndpointError<E, C>: From<C::Error>,
{
    let value = serde_json::from_slice(rsp.body())?;
    let status = rsp.status();
    if !status.is_success() {
        let err = serde_json::from_value::<E::Error>(value)?;
        return Err(ApiError::from_response(err, status, url));
    }
    serde_json::from_value::<T>(value).map_err(ApiError::from_data_type::<T>)
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl<T, E: Endpoint, C: Client> Query<T, E, C> for E
    where
        T: DeserializeOwned,
        E::Error: DeserializeOwned,
        EndpointError<E, C>: From<C::Error>,
    {
        fn query(&self, client: &C) -> EndpointResult<T, E, C> {
            let (req, url) = build_request(self, client)?;
            let rsp = client.send(req, vec![])?;
            serialize_response::<T, E, C>(rsp, url)
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    #[async_trait::async_trait]
    impl<T, E: Endpoint + Sync, C: AsyncClient + Sync> AsyncQuery<T, E, C> for E
    where
        T: DeserializeOwned,
        E::Error: DeserializeOwned,
        EndpointError<E, C>: From<C::Error>,
    {
        async fn query_async(&self, client: &C) -> EndpointResult<T, E, C> {
            let (req, url) = build_request(self, client)?;
            let rsp = client.send(req, vec![]).await?;
            serialize_response::<T, E, C>(rsp, url)
        }
    }
}
