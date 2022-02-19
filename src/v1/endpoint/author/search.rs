use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::AuthorWithPapers;
use crate::v1::endpoint::{iter::SearchBatchEndpontIter, BaseEndpoint};
use crate::v1::error::ResponseError;
use crate::v1::pagination::Pages;
use crate::v1::query_params::AuthorSearchParams;
use crate::v1::static_url::author_search_endpoint;

type AuthorSearchEndpoint = BaseEndpoint<AuthorSearchParams>;

pub struct GetAuthorSearch(AuthorSearchEndpoint);

impl GetAuthorSearch {
    pub fn new(query_params: AuthorSearchParams) -> GetAuthorSearch {
        let endpoint = author_search_endpoint();
        GetAuthorSearch(BaseEndpoint { query_params, endpoint })
    }
}

type AuthorSearchError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    pub struct AuthorSearchIter<'a, T, C>(SearchBatchEndpontIter<'a, T, AuthorSearchEndpoint, C>);

    impl<'a, T, C> AuthorSearchIter<'a, T, C> {
        fn new(
            endpoint: AuthorSearchEndpoint,
            pages: Pages,
            client: &'a C,
        ) -> AuthorSearchIter<'a, T, C> {
            AuthorSearchIter(SearchBatchEndpontIter::new(endpoint, pages, client))
        }
    }

    impl<T, C> AuthorSearchIter<'_, T, C> {
        pub fn total(&self) -> u64 {
            self.0.total()
        }
    }

    impl<'a, T, C> Iterator for AuthorSearchIter<'a, T, C>
    where
        T: From<AuthorWithPapers> + DeserializeOwned,
        C: Client,
        AuthorSearchError<C>: From<C::Error>,
    {
        type Item = Result<T, AuthorSearchError<C>>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(From::from)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.0.size_hint()
        }
    }

    impl GetAuthorSearch {
        pub fn paged<T, C>(self, pages: Pages, client: &C) -> AuthorSearchIter<'_, T, C> {
            AuthorSearchIter::new(self.0, pages, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, AuthorSearchError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: Client,
            AuthorSearchError<C>: From<C::Error>,
        {
            self.0.query(client).map(From::from)
        }
    }
}
#[cfg(feature = "blocking")]
pub use blocking::AuthorSearchIter;

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetAuthorSearch {
        pub fn paged_async<'a, T: 'a, C>(
            self,
            pages: Pages,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, AuthorSearchError<C>>> + 'a
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorSearchError<C>: From<C::Error>,
        {
            SearchBatchEndpontIter::new(self.0, pages, client).into_async_iter()
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, AuthorSearchError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorSearchError<C>: From<C::Error>,
        {
            self.0.query_async(client).await.map(From::from)
        }
    }
}
