use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::{batch::Batch, paper::BasePaper};

pub type ReferenceBatch = Batch<Reference>;

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
