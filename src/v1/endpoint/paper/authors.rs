use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::AuthorWithPapers;
use crate::v1::endpoint::{iter::BatchEndpointIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::PaperAuthorsParams;
use crate::v1::static_url::paper_authors_endpoint;

#[cfg(feature = "blocking")]
pub use blocking::PaperAuthorsIter;

#[cfg(feature = "async")]
pub use r#async::PaperAuthorsAsyncIter;

type PaperAuthorsEndpoint = BaseEndpoint<PaperAuthorsParams>;

type PaperAuthorsError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

pub struct GetPaperAuthors(PaperAuthorsEndpoint);

impl GetPaperAuthors {
    pub fn new(query_params: PaperAuthorsParams, paper_id: String) -> GetPaperAuthors {
        let endpoint = paper_authors_endpoint(&paper_id);
        Self(BaseEndpoint { query_params, endpoint })
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetPaperAuthors {
        pub fn paged<T, C>(self, results: Results, client: &C) -> PaperAuthorsIter<'_, T, C> {
            PaperAuthorsIter::new(self.0, results, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, PaperAuthorsError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: Client,
            PaperAuthorsError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }

    pub struct PaperAuthorsIter<'a, T, C>(BatchEndpointIter<'a, T, PaperAuthorsEndpoint, C>);

    impl<'a, T, C> PaperAuthorsIter<'a, T, C> {
        fn new(
            endpoint: PaperAuthorsEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperAuthorsIter<'a, T, C> {
            PaperAuthorsIter(BatchEndpointIter::new(endpoint, results, client))
        }
    }

    impl<'a, T, C> Iterator for PaperAuthorsIter<'a, T, C>
    where
        T: From<AuthorWithPapers> + DeserializeOwned,
        C: Client,
        PaperAuthorsError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperAuthorsError<C>>;

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

    impl GetPaperAuthors {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> PaperAuthorsAsyncIter<'a, T, C>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperAuthorsError<C>: From<C::Error>,
        {
            PaperAuthorsAsyncIter::new(self.0, results, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, PaperAuthorsError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperAuthorsError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }

    pub struct PaperAuthorsAsyncIter<'a, T, C: AsyncClient>(
        BatchEndpointAsyncIter<'a, T, PaperAuthorsEndpoint, C>,
    );

    impl<'a, T, C: AsyncClient> PaperAuthorsAsyncIter<'a, T, C> {
        fn new(
            endpoint: PaperAuthorsEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperAuthorsAsyncIter<'a, T, C> {
            PaperAuthorsAsyncIter(BatchEndpointAsyncIter::new(endpoint, results, client))
        }
    }

    impl<'a, T: 'a, C: AsyncClient> Stream for PaperAuthorsAsyncIter<'a, T, C>
    where
        T: From<AuthorWithPapers> + DeserializeOwned,
        C: AsyncClient + Sync,
        PaperAuthorsError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperAuthorsError<C>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}
