use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::BasePaper;
use crate::v1::endpoint::{iter::SearchBatchEndpontIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::PaperSearchParams;
use crate::v1::static_url::paper_search_endpoint;

type PaperSearchEndpoint = BaseEndpoint<PaperSearchParams>;

pub struct GetPaperSearch(PaperSearchEndpoint);

impl GetPaperSearch {
    pub fn new(query_params: PaperSearchParams) -> GetPaperSearch {
        let endpoint = paper_search_endpoint();
        GetPaperSearch(BaseEndpoint { query_params, endpoint })
    }
}

type PaperSearchError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    pub struct PaperSearchIter<'a, T, C>(SearchBatchEndpontIter<'a, T, PaperSearchEndpoint, C>);

    impl<'a, T, C> PaperSearchIter<'a, T, C> {
        fn new(
            endpoint: PaperSearchEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperSearchIter<'a, T, C> {
            PaperSearchIter(SearchBatchEndpontIter::new(endpoint, results, client))
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
}
#[cfg(feature = "blocking")]
pub use blocking::PaperSearchIter;

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetPaperSearch {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, PaperSearchError<C>>> + 'a
        where
            T: From<BasePaper> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperSearchError<C>: From<C::Error>,
        {
            SearchBatchEndpontIter::new(self.0, results, client).into_async_iter()
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
}
