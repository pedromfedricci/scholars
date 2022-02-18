use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::{batch::Batch, paper::BasePaper, Batched};

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
pub struct CitationBatch(Batch<Citation>);

impl Batched<Citation> for CitationBatch {
    fn offset(&self) -> u64 {
        self.0.offset()
    }

    fn get_next(&self) -> Option<u64> {
        self.0.get_next()
    }

    fn set_next(&mut self, next: Option<u64>) {
        self.0.set_next(next)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<CitationBatch> for Vec<Citation> {
    fn from(batch: CitationBatch) -> Vec<Citation> {
        Vec::from(batch.0)
    }
}

impl AsRef<Vec<Citation>> for CitationBatch {
    fn as_ref(&self) -> &Vec<Citation> {
        self.0.as_ref()
    }
}

impl AsMut<Vec<Citation>> for CitationBatch {
    fn as_mut(&mut self) -> &mut Vec<Citation> {
        self.0.as_mut()
    }
}
