use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::Citation;
use crate::v1::endpoint::{iter::BatchEndpontIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Pages;
use crate::v1::query_params::PaperCitationsParams;
use crate::v1::static_url::paper_citations_endpoint;

type PaperCitationsEndpoint = BaseEndpoint<PaperCitationsParams>;

pub struct GetPaperCitations(PaperCitationsEndpoint);

impl GetPaperCitations {
    pub fn new(query_params: PaperCitationsParams, paper_id: String) -> GetPaperCitations {
        let endpoint = paper_citations_endpoint(&paper_id);
        GetPaperCitations(BaseEndpoint { query_params, endpoint })
    }
}

type PaperCitationsError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    pub struct PaperCitationsIter<'a, T, C>(BatchEndpontIter<'a, T, PaperCitationsEndpoint, C>);

    impl<'a, T, C> PaperCitationsIter<'a, T, C> {
        fn new(
            endpoint: PaperCitationsEndpoint,
            pages: Pages,
            client: &'a C,
        ) -> PaperCitationsIter<'a, T, C> {
            PaperCitationsIter(BatchEndpontIter::new(endpoint, pages, client))
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

    impl GetPaperCitations {
        pub fn paged<T, C>(self, pages: Pages, client: &C) -> PaperCitationsIter<'_, T, C> {
            PaperCitationsIter::new(self.0, pages, client)
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
}
#[cfg(feature = "blocking")]
pub use blocking::PaperCitationsIter;

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetPaperCitations {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            pages: Pages,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, PaperCitationsError<C>>> + 'a
        where
            T: From<Citation> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperCitationsError<C>: From<C::Error>,
        {
            BatchEndpontIter::new(self.0, pages, client).into_async_iter()
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
}
