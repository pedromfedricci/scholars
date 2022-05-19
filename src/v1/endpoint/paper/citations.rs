use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::Citation;
use crate::v1::endpoint::{iter::BatchEndpointIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::PaperCitationsParams;
use crate::v1::static_url::paper_citations_endpoint;

#[cfg(feature = "blocking")]
pub use blocking::PaperCitationsIter;

#[cfg(feature = "async")]
pub use r#async::PaperCitationsAsyncIter;

type PaperCitationsEndpoint = BaseEndpoint<PaperCitationsParams>;

type PaperCitationsError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

pub struct GetPaperCitations(PaperCitationsEndpoint);

impl GetPaperCitations {
    pub fn new(query_params: PaperCitationsParams, paper_id: String) -> GetPaperCitations {
        let endpoint = paper_citations_endpoint(&paper_id);
        GetPaperCitations(BaseEndpoint { query_params, endpoint })
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetPaperCitations {
        pub fn paged<T, C>(self, results: Results, client: &C) -> PaperCitationsIter<'_, T, C> {
            PaperCitationsIter::new(self.0, results, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, PaperCitationsError<C>>
        where
            T: From<Citation> + DeserializeOwned,
            C: Client,
            PaperCitationsError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }

    pub struct PaperCitationsIter<'a, T, C>(BatchEndpointIter<'a, T, PaperCitationsEndpoint, C>);

    impl<'a, T, C> PaperCitationsIter<'a, T, C> {
        fn new(
            endpoint: PaperCitationsEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperCitationsIter<'a, T, C> {
            PaperCitationsIter(BatchEndpointIter::new(endpoint, results, client))
        }
    }

    impl<'a, T, C> Iterator for PaperCitationsIter<'a, T, C>
    where
        T: From<Citation> + DeserializeOwned,
        C: Client,
        PaperCitationsError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperCitationsError<C>>;

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

    impl GetPaperCitations {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> PaperCitationsAsyncIter<'a, T, C>
        where
            T: From<Citation> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperCitationsError<C>: From<C::Error>,
        {
            PaperCitationsAsyncIter::new(self.0, results, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, PaperCitationsError<C>>
        where
            T: From<Citation> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperCitationsError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }

    pub struct PaperCitationsAsyncIter<'a, T, C: AsyncClient>(
        BatchEndpointAsyncIter<'a, T, PaperCitationsEndpoint, C>,
    );

    impl<'a, T, C: AsyncClient> PaperCitationsAsyncIter<'a, T, C> {
        fn new(
            endpoint: PaperCitationsEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperCitationsAsyncIter<'a, T, C> {
            PaperCitationsAsyncIter(BatchEndpointAsyncIter::new(endpoint, results, client))
        }
    }

    impl<'a, T: 'a, C: AsyncClient> Stream for PaperCitationsAsyncIter<'a, T, C>
    where
        T: From<Citation> + DeserializeOwned,
        C: AsyncClient + Sync,
        PaperCitationsError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperCitationsError<C>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}
