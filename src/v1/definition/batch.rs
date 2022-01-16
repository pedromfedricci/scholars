use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::v1) struct Batch<T> {
    pub(in crate::v1) offset: u64,
    pub(in crate::v1) next: Option<u64>,
    pub(in crate::v1) data: Option<Vec<T>>,
}

impl<T> From<Batch<T>> for Vec<T> {
    fn from(batch: Batch<T>) -> Vec<T> {
        match batch.data {
            Some(data) => data,
            None => vec![],
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::v1) struct SearchBatch<T> {
    #[serde(flatten)]
    pub(in crate::v1) batch: Batch<T>,
    pub(in crate::v1) total: u64,
}

impl<T> From<SearchBatch<T>> for Vec<T> {
    fn from(batch: SearchBatch<T>) -> Vec<T> {
        match batch.batch.data {
            Some(data) => data,
            None => vec![],
        }
    }
}
