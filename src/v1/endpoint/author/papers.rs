use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::PaperWithLinks;
use crate::v1::endpoint::{iter::BatchEndpointIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::AuthorPapersParams;
use crate::v1::static_url::author_papers_endpoint;

#[cfg(feature = "blocking")]
pub use blocking::AuthorPapersIter;

#[cfg(feature = "async")]
pub use r#async::AuthorPapersAsyncIter;

type AuthorPapersEndpoint = BaseEndpoint<AuthorPapersParams>;

pub struct GetAuthorPapers(AuthorPapersEndpoint);

impl GetAuthorPapers {
    pub fn new(query_params: AuthorPapersParams, author_id: String) -> GetAuthorPapers {
        let endpoint = author_papers_endpoint(&author_id);
        GetAuthorPapers(BaseEndpoint { query_params, endpoint })
    }
}

type AuthorPapersError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetAuthorPapers {
        pub fn paged<T, C>(self, results: Results, client: &C) -> AuthorPapersIter<'_, T, C> {
            AuthorPapersIter::new(self.0, results, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, AuthorPapersError<C>>
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: Client,
            AuthorPapersError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }

    pub struct AuthorPapersIter<'a, T, C>(BatchEndpointIter<'a, T, AuthorPapersEndpoint, C>);

    impl<'a, T, C> AuthorPapersIter<'a, T, C> {
        fn new(
            endpoint: AuthorPapersEndpoint,
            results: Results,
            client: &'a C,
        ) -> AuthorPapersIter<'a, T, C> {
            AuthorPapersIter(BatchEndpointIter::new(endpoint, results, client))
        }
    }

    impl<'a, T, C> Iterator for AuthorPapersIter<'a, T, C>
    where
        T: From<PaperWithLinks> + DeserializeOwned,
        C: Client,
        AuthorPapersError<C>: From<C::Error>,
    {
        type Item = Result<T, AuthorPapersError<C>>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(From::from)
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_core::Stream;

    use super::*;
    use crate::v1::endpoint::iter::BatchEndpointAsyncIter;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetAuthorPapers {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> AuthorPapersAsyncIter<'a, T, C>
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorPapersError<C>: From<C::Error>,
        {
            AuthorPapersAsyncIter::new(self.0, results, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, AuthorPapersError<C>>
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorPapersError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }

    pub struct AuthorPapersAsyncIter<'a, T, C: AsyncClient>(
        BatchEndpointAsyncIter<'a, T, AuthorPapersEndpoint, C>,
    );

    impl<'a, T, C: AsyncClient> AuthorPapersAsyncIter<'a, T, C> {
        fn new(
            endpoint: AuthorPapersEndpoint,
            results: Results,
            client: &'a C,
        ) -> AuthorPapersAsyncIter<'a, T, C> {
            AuthorPapersAsyncIter(BatchEndpointAsyncIter::new(endpoint, results, client))
        }
    }

    impl<'a, T: 'a, C: AsyncClient> Stream for AuthorPapersAsyncIter<'a, T, C>
    where
        T: From<PaperWithLinks> + DeserializeOwned,
        C: AsyncClient + Sync,
        AuthorPapersError<C>: From<C::Error>,
    {
        type Item = Result<T, AuthorPapersError<C>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}
