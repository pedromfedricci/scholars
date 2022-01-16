use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::paper::FullPaper;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Embedding {
    // The underlying model+version that produced the embedding.
    pub model: Option<String>,
    // Numerical embedding vector.
    pub vector: Option<Vec<f64>>,
}

impl From<FullPaper> for Embedding {
    fn from(paper: FullPaper) -> Embedding {
        paper.embedding.unwrap_or_default()
    }
}
