mod iter;
use iter::*;

mod author;
pub use author::*;

use http::Method;
use serde::Serialize;
use serde_urlencoded::ser::Error as UrlEncodedError;

use crate::client::BaseClient;
use crate::endpoint::Endpoint;
use crate::error::ApiError;
use crate::urlencoded::UrlEncodedQuery;
use crate::v1::error::ResponseError;
use crate::v1::pagination::{Page, Paged};

type EndpointError<E, C> = ApiError<<E as Endpoint>::Error, <C as BaseClient>::Error>;

type EndpointResult<T, E, C> = Result<T, EndpointError<E, C>>;

#[derive(Debug)]
pub(in crate::v1) struct BaseEndpoint<P> {
    query_params: P,
    endpoint: String,
}

impl<P: Serialize> Endpoint for BaseEndpoint<P> {
    type Error = ResponseError;

    #[inline]
    fn method(&self) -> Method {
        Method::GET
    }

    #[inline]
    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn query_params(&self) -> Result<UrlEncodedQuery<'_>, UrlEncodedError> {
        UrlEncodedQuery::with(&self.query_params)
    }
}

impl<P: Paged> Paged for BaseEndpoint<P> {
    #[inline]
    fn get_page(&self) -> &Page {
        self.query_params.get_page()
    }

    #[inline]
    fn get_page_mut(&mut self) -> &mut Page {
        self.query_params.get_page_mut()
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::client::AsyncClient;
    use crate::query::AsyncQuery;
    use crate::v1::definition::Batch;
    use crate::v1::pagination::Pages;

    impl<'a, P: Paged + Serialize + 'a> BaseEndpoint<P> {
        pub(in crate::v1) fn into_async_iter<T: 'a, C>(
            self,
            pages: Pages,
            client: &'a C,
        ) -> impl futures_util::Stream<Item = EndpointResult<T, Self, C>> + 'a
        where
            Self: AsyncQuery<Batch<T>, Self, C> + Sync,
            C: AsyncClient + Sync,
            EndpointError<Self, C>: From<C::Error>,
        {
            let iter = EndpointIter::new(self, pages, client);
            futures_util::stream::unfold(iter, |mut iter| async move {
                iter.next_async().await.map(|item| (item, iter))
            })
        }
    }
}
