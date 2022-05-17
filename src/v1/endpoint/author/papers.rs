use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::PaperWithLinks;
use crate::v1::endpoint::{iter::BatchEndpontIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Results;
use crate::v1::query_params::AuthorPapersParams;
use crate::v1::static_url::author_papers_endpoint;

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

    pub struct AuthorPapersIter<'a, T, C>(BatchEndpontIter<'a, T, AuthorPapersEndpoint, C>);

    impl<'a, T, C> AuthorPapersIter<'a, T, C> {
        fn new(
            endpoint: AuthorPapersEndpoint,
            results: Results,
            client: &'a C,
        ) -> AuthorPapersIter<'a, T, C> {
            AuthorPapersIter(BatchEndpontIter::new(endpoint, results, client))
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
}
#[cfg(feature = "blocking")]
pub use blocking::AuthorPapersIter;

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetAuthorPapers {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            results: Results,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, AuthorPapersError<C>>> + 'a
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorPapersError<C>: From<C::Error>,
        {
            BatchEndpontIter::new(self.0, results, client).into_async_iter()
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
}
