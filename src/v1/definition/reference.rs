use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::{batch::Batch, paper::BasePaper, Batched};

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
pub struct ReferenceBatch(Batch<Reference>);

impl Batched<Reference> for ReferenceBatch {
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

impl From<ReferenceBatch> for Vec<Reference> {
    fn from(batch: ReferenceBatch) -> Vec<Reference> {
        Vec::from(batch.0)
    }
}

impl AsRef<Vec<Reference>> for ReferenceBatch {
    fn as_ref(&self) -> &Vec<Reference> {
        self.0.as_ref()
    }
}

impl AsMut<Vec<Reference>> for ReferenceBatch {
    fn as_mut(&mut self) -> &mut Vec<Reference> {
        self.0.as_mut()
    }
}
