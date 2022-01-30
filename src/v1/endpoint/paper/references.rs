use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::Reference;
use crate::v1::endpoint::BaseEndpoint;
use crate::v1::error::ResponseError;
use crate::v1::pagination::Pages;
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
    use crate::{client::Client, query::Query, v1::endpoint::EndpointIter};

    pub struct PaperReferencesIter<'a, T, C>(EndpointIter<'a, T, PaperReferencesEndpoint, C>);

    impl<'a, T, C> PaperReferencesIter<'a, T, C> {
        fn new(
            endpoint: PaperReferencesEndpoint,
            pages: Pages,
            client: &'a C,
        ) -> PaperReferencesIter<'a, T, C> {
            PaperReferencesIter(EndpointIter::new(endpoint, pages, client))
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
            <EndpointIter<'a, T, PaperReferencesEndpoint, C> as Iterator>::next(&mut self.0)
        }
    }

    impl GetPaperReferences {
        pub fn paged<T, C>(self, pages: Pages, client: &C) -> PaperReferencesIter<'_, T, C>
        where
            T: From<Reference>,
        {
            PaperReferencesIter::new(self.0, pages, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, PaperReferencesError<C>>
        where
            T: From<Reference> + DeserializeOwned,
            C: Client,
            PaperReferencesError<C>: From<C::Error>,
        {
            self.0.query(client)
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
            pages: Pages,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, PaperReferencesError<C>>> + 'a
        where
            T: From<Reference> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperReferencesError<C>: From<C::Error>,
        {
            self.0.into_async_iter(pages, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, PaperReferencesError<C>>
        where
            T: From<Reference> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperReferencesError<C>: From<C::Error>,
        {
            self.0.query_async(client).await
        }
    }
}
