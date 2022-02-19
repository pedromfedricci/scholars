use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::AuthorWithPapers;
use crate::v1::endpoint::{iter::BatchEndpontIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Pages;
use crate::v1::query_params::PaperAuthorsParams;
use crate::v1::static_url::paper_authors_endpoint;

type PaperAuthorsEndpoint = BaseEndpoint<PaperAuthorsParams>;

pub struct GetPaperAuthors(PaperAuthorsEndpoint);

impl GetPaperAuthors {
    pub fn new(query_params: PaperAuthorsParams, paper_id: String) -> GetPaperAuthors {
        let endpoint = paper_authors_endpoint(&paper_id);
        Self(BaseEndpoint { query_params, endpoint })
    }
}

type PaperAuthorsError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    pub struct PaperAuthorsIter<'a, T, C>(BatchEndpontIter<'a, T, PaperAuthorsEndpoint, C>);

    impl<'a, T, C> PaperAuthorsIter<'a, T, C> {
        fn new(
            endpoint: PaperAuthorsEndpoint,
            pages: Pages,
            client: &'a C,
        ) -> PaperAuthorsIter<'a, T, C> {
            PaperAuthorsIter(BatchEndpontIter::new(endpoint, pages, client))
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

    impl GetPaperAuthors {
        pub fn paged<T, C>(self, pages: Pages, client: &C) -> PaperAuthorsIter<'_, T, C> {
            PaperAuthorsIter::new(self.0, pages, client)
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
}
#[cfg(feature = "blocking")]
pub use blocking::PaperAuthorsIter;

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetPaperAuthors {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            pages: Pages,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, PaperAuthorsError<C>>> + 'a
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperAuthorsError<C>: From<C::Error>,
        {
            BatchEndpontIter::new(self.0, pages, client).into_async_iter()
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
}
