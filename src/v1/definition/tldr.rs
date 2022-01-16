use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::FullPaper;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Tldr {
    // The underlying model+version that produced the tldr.
    pub model: Option<String>,
    // paper TLDR summary.
    pub text: Option<String>,
}

impl From<FullPaper> for Tldr {
    fn from(paper: FullPaper) -> Tldr {
        paper.tldr.unwrap_or_default()
    }
}
