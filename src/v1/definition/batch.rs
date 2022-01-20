use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub(in crate::v1) struct Batch<T> {
    pub(in crate::v1) offset: u64,
    pub(in crate::v1) next: Option<u64>,
    pub(in crate::v1) data: Vec<T>,
}

impl<T> Default for Batch<T> {
    fn default() -> Batch<T> {
        Batch { offset: 0, next: None, data: vec![] }
    }
}

impl<T> From<Batch<T>> for Vec<T> {
    fn from(batch: Batch<T>) -> Vec<T> {
        batch.data
    }
}

impl<T> AsRef<[T]> for Batch<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub(in crate::v1) struct SearchBatch<T> {
    #[serde(flatten)]
    pub(in crate::v1) batch: Batch<T>,
    pub(in crate::v1) total: u64,
}

impl<T> Default for SearchBatch<T> {
    fn default() -> SearchBatch<T> {
        SearchBatch { batch: Batch::default(), total: 0 }
    }
}

impl<T> From<SearchBatch<T>> for Vec<T> {
    fn from(batch: SearchBatch<T>) -> Vec<T> {
        batch.batch.data
    }
}

impl<T> AsRef<[T]> for SearchBatch<T> {
    fn as_ref(&self) -> &[T] {
        &self.batch.data
    }
}
