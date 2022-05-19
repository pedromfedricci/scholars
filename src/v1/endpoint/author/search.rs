use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::AuthorWithPapers;
use crate::v1::endpoint::{iter::SearchBatchEndpointIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::AuthorSearchParams;
use crate::v1::static_url::author_search_endpoint;

#[cfg(feature = "blocking")]
pub use blocking::AuthorSearchIter;

#[cfg(feature = "async")]
pub use r#async::AuthorSearchAsyncIter;

type AuthorSearchEndpoint = BaseEndpoint<AuthorSearchParams>;

type AuthorSearchError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

pub struct GetAuthorSearch(AuthorSearchEndpoint);

impl GetAuthorSearch {
    pub fn new(query_params: AuthorSearchParams) -> GetAuthorSearch {
        let endpoint = author_search_endpoint();
        GetAuthorSearch(BaseEndpoint { query_params, endpoint })
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetAuthorSearch {
        pub fn paged<T, C>(self, results: Results, client: &C) -> AuthorSearchIter<'_, T, C> {
            AuthorSearchIter::new(self.0, results, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, AuthorSearchError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: Client,
            AuthorSearchError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }

    pub struct AuthorSearchIter<'a, T, C>(SearchBatchEndpointIter<'a, T, AuthorSearchEndpoint, C>);

    impl<'a, T, C> AuthorSearchIter<'a, T, C> {
        fn new(
            endpoint: AuthorSearchEndpoint,
            results: Results,
            client: &'a C,
        ) -> AuthorSearchIter<'a, T, C> {
            AuthorSearchIter(SearchBatchEndpointIter::new(endpoint, results, client))
        }
    }

    impl<T, C> AuthorSearchIter<'_, T, C> {
        pub fn total(&self) -> u64 {
            self.0.total()
        }
    }

    impl<'a, T, C> Iterator for AuthorSearchIter<'a, T, C>
    where
        T: From<AuthorWithPapers> + DeserializeOwned,
        C: Client,
        AuthorSearchError<C>: From<C::Error>,
    {
        type Item = Result<T, AuthorSearchError<C>>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(From::from)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_core::Stream;

    use super::*;
    use crate::v1::endpoint::iter::SearchBatchEndpointAsyncIter;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetAuthorSearch {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> AuthorSearchAsyncIter<'a, T, C>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorSearchError<C>: From<C::Error>,
        {
            AuthorSearchAsyncIter::new(self.0, results, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, AuthorSearchError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorSearchError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }

    pub struct AuthorSearchAsyncIter<'a, T, C: AsyncClient>(
        SearchBatchEndpointAsyncIter<'a, T, AuthorSearchEndpoint, C>,
    );

    impl<'a, T, C: AsyncClient> AuthorSearchAsyncIter<'a, T, C> {
        fn new(
            endpoint: AuthorSearchEndpoint,
            results: Results,
            client: &'a C,
        ) -> AuthorSearchAsyncIter<'a, T, C> {
            AuthorSearchAsyncIter(SearchBatchEndpointAsyncIter::new(endpoint, results, client))
        }
    }

    impl<T, C: AsyncClient> AuthorSearchAsyncIter<'_, T, C> {
        pub fn total(&self) -> u64 {
            self.0.total()
        }
    }

    impl<'a, T: 'a, C: AsyncClient> Stream for AuthorSearchAsyncIter<'a, T, C>
    where
        T: From<AuthorWithPapers> + DeserializeOwned,
        C: AsyncClient + Sync,
        AuthorSearchError<C>: From<C::Error>,
    {
        type Item = Result<T, AuthorSearchError<C>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}
