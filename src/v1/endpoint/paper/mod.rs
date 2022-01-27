mod search;
pub use search::*;

use serde::de::DeserializeOwned;

use crate::client::BaseClient;
use crate::error::ApiError;
use crate::v1::definition::FullPaper;
use crate::v1::endpoint::BaseEndpoint;
use crate::v1::error::ResponseError;
use crate::v1::query_params::PaperParams;
use crate::v1::static_url::paper_endpoint;

type PaperEndpoint = BaseEndpoint<PaperParams>;

pub struct GetPaper(PaperEndpoint);

impl GetPaper {
    pub fn new(query_params: PaperParams, paper_id: String) -> GetPaper {
        let endpoint = paper_endpoint(&paper_id);
        GetPaper(BaseEndpoint { query_params, endpoint })
    }
}

type PaperError<C> = ApiError<ResponseError, <C as BaseClient>::Error>;

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl GetPaper {
        pub fn query<T, C>(&self, client: &C) -> Result<T, PaperError<C>>
        where
            T: From<FullPaper> + DeserializeOwned,
            C: Client,
            PaperError<C>: From<C::Error>,
        {
            self.0.query(client)
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl GetPaper {
        pub async fn query_async<T, C>(&self, client: &C) -> Result<T, PaperError<C>>
        where
            T: From<FullPaper> + DeserializeOwned,
            C: AsyncClient + Sync,
            PaperError<C>: From<C::Error>,
        {
            self.0.query_async(client).await
        }
    }
}
