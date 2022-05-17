use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::Reference;
use crate::v1::endpoint::{iter::BatchEndpontIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::PaperReferencesParams;
use crate::v1::static_url::paper_references_endpoint;

type PaperReferencesEndpoint = BaseEndpoint<PaperReferencesParams>;

pub struct GetPaperReferences(PaperReferencesEndpoint);

impl GetPaperReferences {
    pub fn new(query_params: PaperReferencesParams, paper_id: String) -> GetPaperReferences {
        let endpoint = paper_references_endpoint(&paper_id);
        GetPaperReferences(BaseEndpoint { query_params, endpoint })
    }
}

type PaperReferencesError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    pub struct PaperReferencesIter<'a, T, C>(BatchEndpontIter<'a, T, PaperReferencesEndpoint, C>);

    impl<'a, T, C> PaperReferencesIter<'a, T, C> {
        fn new(
            endpoint: PaperReferencesEndpoint,
            results: Results,
            client: &'a C,
        ) -> PaperReferencesIter<'a, T, C> {
            PaperReferencesIter(BatchEndpontIter::new(endpoint, results, client))
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
}
#[cfg(feature = "blocking")]
pub use blocking::PaperReferencesIter;

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetPaperReferences {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, PaperReferencesError<C>>> + 'a
        where
            T: From<Reference> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperReferencesError<C>: From<C::Error>,
        {
            BatchEndpontIter::new(self.0, results, client).into_async_iter()
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
}
