use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::AuthorWithPapers;
use crate::v1::endpoint::BaseEndpoint;
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
    use crate::{client::Client, query::Query, v1::endpoint::EndpointIter};

    pub struct AuthorSearchIter<'a, T, C>(EndpointIter<'a, T, AuthorSearchEndpoint, C>);

    impl<'a, T, C> AuthorSearchIter<'a, T, C> {
        fn new(
            endpoint: AuthorSearchEndpoint,
            pages: Pages,
            client: &'a C,
        ) -> AuthorSearchIter<'a, T, C> {
            AuthorSearchIter(EndpointIter::new(endpoint, pages, client))
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
            <EndpointIter<'a, T, AuthorSearchEndpoint, C> as Iterator>::next(&mut self.0)
        }
    }

    impl GetAuthorSearch {
        pub fn paged<T, C>(self, pages: Pages, client: &C) -> AuthorSearchIter<'_, T, C>
        where
            T: From<AuthorWithPapers>,
        {
            AuthorSearchIter::new(self.0, pages, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, AuthorSearchError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: Client,
            AuthorSearchError<C>: From<C::Error>,
        {
            self.0.query(client)
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
            self.0.into_async_iter(pages, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, AuthorSearchError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorSearchError<C>: From<C::Error>,
        {
            self.0.query_async(client).await
        }
    }
}
