use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::{batch::Batch, paper::BasePaper};

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Citation {
    // Details about the citing paper.
    pub citing_paper: Option<BasePaper>,
    // List of contexts.
    pub contexts: Option<HashSet<String>>,
    // List of intents.
    pub intents: Option<HashSet<String>>,
    // See: https://www.semanticscholar.org/faq#influential-citations.
    pub is_influential: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CitationBatch<T = Citation>(Batch<T>);

impl<T> CitationBatch<T> {
    pub fn get_offset(&self) -> u64 {
        self.0.offset
    }

    pub fn get_next(&self) -> Option<u64> {
        self.0.next
    }
}

impl<T> From<CitationBatch<T>> for Vec<T>
where
    T: From<Citation>,
{
    fn from(batch: CitationBatch<T>) -> Vec<T> {
        Vec::from(batch.0)
    }
}

impl<T> AsRef<[T]> for CitationBatch<T>
where
    T: From<Citation>,
{
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}
