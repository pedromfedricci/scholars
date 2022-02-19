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

impl<T> AsRef<Vec<T>> for Batch<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> AsMut<Vec<T>> for Batch<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub(in crate::v1) struct SearchBatch<T> {
    #[serde(flatten)]
    pub(in crate::v1) base: Batch<T>,
    pub(in crate::v1) total: u64,
}

#[cfg(feature = "blocking")]
impl<T> SearchBatch<T> {
    pub(in crate::v1) fn total(&self) -> u64 {
        self.total
    }
}

impl<T> Default for SearchBatch<T> {
    fn default() -> SearchBatch<T> {
        SearchBatch { base: Batch::default(), total: 0 }
    }
}

impl<T> From<SearchBatch<T>> for Vec<T> {
    fn from(batch: SearchBatch<T>) -> Vec<T> {
        batch.base.data
    }
}

impl<T> AsRef<Vec<T>> for SearchBatch<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.base.data
    }
}

impl<T> AsMut<Vec<T>> for SearchBatch<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.base.data
    }
}

pub(in crate::v1) trait Batched<T>: Default + AsRef<Vec<T>> + AsMut<Vec<T>> {
    fn offset(&self) -> u64;

    fn get_next(&self) -> Option<u64>;

    fn set_next(&mut self, next: Option<u64>);

    fn len(&self) -> usize;
}

impl<T> Batched<T> for Batch<T> {
    fn offset(&self) -> u64 {
        self.offset
    }

    fn get_next(&self) -> Option<u64> {
        self.next
    }

    fn set_next(&mut self, next: Option<u64>) {
        self.next = next;
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Batched<T> for SearchBatch<T> {
    fn offset(&self) -> u64 {
        self.base.offset()
    }

    fn get_next(&self) -> Option<u64> {
        self.base.get_next()
    }

    fn set_next(&mut self, next: Option<u64>) {
        self.base.set_next(next);
    }

    fn len(&self) -> usize {
        self.base.len()
    }
}
