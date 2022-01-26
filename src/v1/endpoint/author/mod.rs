mod papers;
pub use papers::*;
mod search;
pub use search::*;

use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::AuthorWithPapers;
use crate::v1::endpoint::BaseEndpoint;
use crate::v1::error::ResponseError;
use crate::v1::query_params::AuthorParams;
use crate::v1::static_url::author_endpoint;

type AuthorEndpoint = BaseEndpoint<AuthorParams>;

pub struct GetAuthor(AuthorEndpoint);

impl GetAuthor {
    pub fn new(query_params: AuthorParams, author_id: String) -> GetAuthor {
        let endpoint = author_endpoint(&author_id);
        GetAuthor(BaseEndpoint { query_params, endpoint })
    }
}

type AuthorError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetAuthor {
        pub fn query<T, C>(&self, client: &C) -> Result<T, AuthorError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: Client,
            AuthorError<C>: From<C::Error>,
        {
            self.0.query(client)
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetAuthor {
        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, AuthorError<C>>
        where
            T: From<AuthorWithPapers> + DeserializeOwned,
            C: AsyncClient + Sync,
            AuthorError<C>: From<C::Error>,
        {
            self.0.query_async(client).await
        }
    }
}
