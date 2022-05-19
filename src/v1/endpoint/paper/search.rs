use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::BasePaper;
use crate::v1::endpoint::{iter::SearchBatchEndpointIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::PaperSearchParams;
use crate::v1::static_url::paper_search_endpoint;

#[cfg(feature = "blocking")]
pub use blocking::PaperSearchIter;

#[cfg(feature = "async")]
pub use r#async::PaperSearchAsyncIter;

type PaperSearchEndpoint = BaseEndpoint<PaperSearchParams>;

type PaperSearchError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

pub struct GetPaperSearch(PaperSearchEndpoint);

impl GetPaperSearch {
    pub fn new(query_params: PaperSearchParams) -> GetPaperSearch {
        let endpoint = paper_search_endpoint();
        GetPaperSearch(BaseEndpoint { query_params, endpoint })
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetPaperSearch {
        pub fn paged<T, C>(self, results: Results, client: &C) -> PaperSearchIter<'_, T, C> {
            PaperSearchIter::new(self.0, results, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, PaperSearchError<C>>
        where
            T: From<BasePaper> + DeserializeOwned,
            C: Client,
            PaperSearchError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }

    pub struct PaperSearchIter<'a, T, C>(SearchBatchEndpointIter<'a, T, PaperSearchEndpoint, C>);

    impl<'a, T, C> PaperSearchIter<'a, T, C> {
        fn new(
            endpoint: PaperSearchEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperSearchIter<'a, T, C> {
            PaperSearchIter(SearchBatchEndpointIter::new(endpoint, results, client))
        }
    }

    impl<T, C> PaperSearchIter<'_, T, C> {
        pub fn total(&self) -> u64 {
            self.0.total()
        }
    }

    impl<'a, T, C> Iterator for PaperSearchIter<'a, T, C>
    where
        T: From<BasePaper> + DeserializeOwned,
        C: Client,
        PaperSearchError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperSearchError<C>>;

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

    impl GetPaperSearch {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> PaperSearchAsyncIter<'a, T, C>
        where
            T: From<BasePaper> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperSearchError<C>: From<C::Error>,
        {
            PaperSearchAsyncIter::new(self.0, results, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, PaperSearchError<C>>
        where
            T: From<BasePaper> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperSearchError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }

    pub struct PaperSearchAsyncIter<'a, T, C: AsyncClient>(
        SearchBatchEndpointAsyncIter<'a, T, PaperSearchEndpoint, C>,
    );

    impl<'a, T, C: AsyncClient> PaperSearchAsyncIter<'a, T, C> {
        fn new(
            endpoint: PaperSearchEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperSearchAsyncIter<'a, T, C> {
            PaperSearchAsyncIter(SearchBatchEndpointAsyncIter::new(endpoint, results, client))
        }
    }

    impl<T, C: AsyncClient> PaperSearchAsyncIter<'_, T, C> {
        pub fn total(&self) -> u64 {
            self.0.total()
        }
    }

    impl<'a, T: 'a, C: AsyncClient> Stream for PaperSearchAsyncIter<'a, T, C>
    where
        T: From<BasePaper> + DeserializeOwned,
        C: AsyncClient + Sync,
        PaperSearchError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperSearchError<C>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}
