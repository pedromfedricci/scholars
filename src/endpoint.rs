use std::error::Error;

use bytes::Bytes;
use http::{request::Builder, Method, Request, Response, Uri};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{client::BaseClient, error::ApiError, urlencoded::UrlEncodedQuery};

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// An error type that can be returned by the endpoint.
    type Error: Error;

    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;

    /// The path to the endpoint.
    fn endpoint(&self) -> &str;

    /// URL query string for the endpoint.
    fn query_params(&self) -> UrlEncodedQuery;
}

#[cfg(feature = "blocking")]
use crate::{client::Client, query::Query};

#[cfg(feature = "blocking")]
impl<T, E: Endpoint, C: Client> Query<T, E, C> for E
where
    T: DeserializeOwned,
    E::Error: DeserializeOwned,
    ApiError<E::Error, C::Error>: From<C::Error>,
{
    fn query(&self, client: &C) -> Result<T, ApiError<E::Error, C::Error>> {
        let (req, url) = build_request(self, client)?;
        let rsp = client.send(req, vec![])?;
        serialize_response::<T, E, C>(rsp, url)
    }
}

#[cfg(feature = "async")]
use crate::{client::AsyncClient, query::AsyncQuery};

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<T, E: Endpoint + Sync, C: AsyncClient + Sync> AsyncQuery<T, E, C> for E
where
    T: DeserializeOwned,
    E::Error: DeserializeOwned,
    ApiError<E::Error, C::Error>: From<C::Error>,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<E::Error, C::Error>> {
        let (req, url) = build_request(self, client)?;
        let rsp = client.send(req, vec![]).await?;
        serialize_response::<T, E, C>(rsp, url)
    }
}

/// Converts [`url::Url`] into [`http::Uri`].
#[inline]
fn url_to_http_uri(url: &Url) -> Uri {
    url.as_str().parse().expect("parse a url::Url as an http::Uri")
}

/// Gets the Endpoint's [`Url`] and creates the request [`Builder`].
#[inline]
fn build_request<E, C>(
    endpoint: &E,
    client: &C,
) -> Result<(Builder, Url), ApiError<E::Error, C::Error>>
where
    E: Endpoint,
    C: BaseClient,
{
    let mut url = client.endpoint(endpoint.endpoint())?;
    endpoint.query_params().set_url(&mut url);
    log::debug!("querying endpoint at: {}", url.as_str());
    let builder = Request::builder().method(endpoint.method()).uri(url_to_http_uri(&url));
    Ok((builder, url))
}

/// Serializes the JSON payload.
#[inline]
fn serialize_response<T, E: Endpoint, C: BaseClient>(
    rsp: Response<Bytes>,
    url: Url,
) -> Result<T, ApiError<E::Error, C::Error>>
where
    T: DeserializeOwned,
    E::Error: DeserializeOwned,
    ApiError<E::Error, C::Error>: From<C::Error>,
{
    let value = serde_json::from_slice(rsp.body())?;
    let status = rsp.status();
    if !status.is_success() {
        let err = serde_json::from_value::<E::Error>(value)?;
        return Err(ApiError::from_response(err, status, url));
    }
    serde_json::from_value::<T>(value).map_err(ApiError::data_type::<T>)
}
