use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::Reference;
use crate::v1::endpoint::{iter::BatchEndpointIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::PaperReferencesParams;
use crate::v1::static_url::paper_references_endpoint;

#[cfg(feature = "blocking")]
pub use blocking::PaperReferencesIter;

#[cfg(feature = "async")]
pub use r#async::PaperReferencesAsyncIter;

type PaperReferencesEndpoint = BaseEndpoint<PaperReferencesParams>;

type PaperReferencesError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

pub struct GetPaperReferences(PaperReferencesEndpoint);

impl GetPaperReferences {
    pub fn new(query_params: PaperReferencesParams, paper_id: String) -> GetPaperReferences {
        let endpoint = paper_references_endpoint(&paper_id);
        GetPaperReferences(BaseEndpoint { query_params, endpoint })
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetPaperReferences {
        pub fn paged<T, C>(self, results: Results, client: &C) -> PaperReferencesIter<'_, T, C> {
            PaperReferencesIter::new(self.0, results, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, PaperReferencesError<C>>
        where
            T: From<Reference> + DeserializeOwned,
            C: Client,
            PaperReferencesError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }

    pub struct PaperReferencesIter<'a, T, C>(BatchEndpointIter<'a, T, PaperReferencesEndpoint, C>);

    impl<'a, T, C> PaperReferencesIter<'a, T, C> {
        fn new(
            endpoint: PaperReferencesEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperReferencesIter<'a, T, C> {
            PaperReferencesIter(BatchEndpointIter::new(endpoint, results, client))
        }
    }

    impl<'a, T, C> Iterator for PaperReferencesIter<'a, T, C>
    where
        T: From<Reference> + DeserializeOwned,
        C: Client,
        PaperReferencesError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperReferencesError<C>>;

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

    impl GetPaperReferences {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> PaperReferencesAsyncIter<'a, T, C>
        where
            T: From<Reference> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperReferencesError<C>: From<C::Error>,
        {
            PaperReferencesAsyncIter::new(self.0, results, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, PaperReferencesError<C>>
        where
            T: From<Reference> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperReferencesError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }

    pub struct PaperReferencesAsyncIter<'a, T, C: AsyncClient>(
        BatchEndpointAsyncIter<'a, T, PaperReferencesEndpoint, C>,
    );

    impl<'a, T, C: AsyncClient> PaperReferencesAsyncIter<'a, T, C> {
        fn new(
            endpoint: PaperReferencesEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperReferencesAsyncIter<'a, T, C> {
            PaperReferencesAsyncIter(BatchEndpointAsyncIter::new(endpoint, results, client))
        }
    }

    impl<'a, T: 'a, C: AsyncClient> Stream for PaperReferencesAsyncIter<'a, T, C>
    where
        T: From<Reference> + DeserializeOwned,
        C: AsyncClient + Sync,
        PaperReferencesError<C>: From<C::Error>,
    {
        type Item = Result<T, PaperReferencesError<C>>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }
}
