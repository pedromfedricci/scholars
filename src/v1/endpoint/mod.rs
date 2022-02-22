mod iter;

mod author;
pub use author::*;

mod paper;
pub use paper::*;

use http::Method;
use serde::Serialize;
use serde_urlencoded::ser::Error as UrlEncodedError;

use crate::endpoint::Endpoint;
use crate::urlencoded::UrlEncodedQuery;
use crate::v1::error::ResponseError;
use crate::v1::pagination::{Page, Paged};

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

impl<P: Paged> AsRef<Page> for BaseEndpoint<P> {
    fn as_ref(&self) -> &Page {
        self.query_params.as_ref()
    }
}

impl<P: Paged> AsMut<Page> for BaseEndpoint<P> {
    fn as_mut(&mut self) -> &mut Page {
        self.query_params.as_mut()
    }
}
