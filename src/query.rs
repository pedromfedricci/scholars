use crate::{endpoint::Endpoint, error::ApiError};

/// A trait which represents an API query which can be made by a client.
#[cfg(feature = "blocking")]
pub trait Query<T, E, C>
where
    E: Endpoint,
    C: crate::client::Client,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<E::Error, C::Error>>;
}

/// A trait which represents an asynchronous API query which can be made by a client.
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncQuery<T, E, C>
where
    E: Endpoint + Sync,
    C: crate::client::AsyncClient + Sync,
{
    /// Perform the query asynchronously against the client.
    async fn query_async(&self, client: &C) -> Result<T, ApiError<E::Error, C::Error>>;
}
