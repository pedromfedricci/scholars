use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::PaperWithLinks;
use crate::v1::endpoint::BaseEndpoint;
use crate::v1::error::ResponseError;
use crate::v1::pagination::Pages;
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
    use crate::{client::Client, query::Query, v1::endpoint::EndpointIter};

    pub struct AuthorPapersIter<'a, T, C>(EndpointIter<'a, T, AuthorPapersEndpoint, C>);

    impl<'a, T, C> AuthorPapersIter<'a, T, C> {
        fn new(
            endpoint: AuthorPapersEndpoint,
            pages: Pages,
            client: &'a C,
        ) -> AuthorPapersIter<'a, T, C> {
            AuthorPapersIter(EndpointIter::new(endpoint, pages, client))
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
            <EndpointIter<'a, T, AuthorPapersEndpoint, C> as Iterator>::next(&mut self.0)
        }
    }

    impl GetAuthorPapers {
        pub fn paged<T, C>(self, pages: Pages, client: &C) -> AuthorPapersIter<'_, T, C>
        where
            T: From<PaperWithLinks>,
        {
            AuthorPapersIter::new(self.0, pages, client)
        }

        pub fn query<T, C>(&self, client: &C) -> Result<T, AuthorPapersError<C>>
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: Client,
            AuthorPapersError<C>: From<C::Error>,
        {
            self.0.query(client)
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
            pages: Pages,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = Result<T, AuthorPapersError<C>>> + 'a
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorPapersError<C>: From<C::Error>,
        {
            self.0.into_async_iter(pages, client)
        }

        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, AuthorPapersError<C>>
        where
            T: From<PaperWithLinks> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorPapersError<C>: From<C::Error>,
        {
            self.0.query_async(client).await
        }
    }
}
