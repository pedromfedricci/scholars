use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::{batch::Batch, paper::BasePaper};

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Reference {
    // Details about the cited paper.
    pub cited_paper: Option<BasePaper>,
    // List of contexts.
    pub contexts: Option<HashSet<String>>,
    // List of intents.
    pub intents: Option<HashSet<String>>,
    // See: https://www.semanticscholar.org/faq#influential-citations.
    pub is_influential: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReferenceBatch<T = Reference>(Batch<T>);

impl<T> ReferenceBatch<T> {
    pub fn get_offset(&self) -> u64 {
        self.0.offset
    }

    pub fn get_next(&self) -> Option<u64> {
        self.0.next
    }
}

impl<T> From<ReferenceBatch<T>> for Vec<T>
where
    T: From<Reference>,
{
    fn from(batch: ReferenceBatch<T>) -> Vec<T> {
        Vec::from(batch.0)
    }
}

impl<T> AsRef<[T]> for ReferenceBatch<T>
where
    T: From<Reference>,
{
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}
