use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::{batch::Batch, paper::BasePaper};

pub type AuthorBatch = Batch<AuthorWithPapers>;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorInfo {
    // Author's ID in Semantic Scholar.
    // authorId field is always included in the response.
    // API can return null for the authorId field.
    pub author_id: Option<String>,
    // Author's name.
    // This field will be provided if no fields are specified.
    pub name: Option<String>,
}

impl From<Author> for AuthorInfo {
    fn from(author: Author) -> AuthorInfo {
        author.info
    }
}

impl From<AuthorWithPapers> for AuthorInfo {
    fn from(author: AuthorWithPapers) -> AuthorInfo {
        author.author.info
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Author {
    // Author's info.
    #[serde(flatten)]
    pub info: AuthorInfo,
    // ORCID/DBLP IDs for this author, if known.
    pub external_ids: Option<AuthorExternalId>,
    // URL on the Semantic Scholar website.
    pub url: Option<String>,
    // List of aliases.
    pub aliases: Option<HashSet<String>>,
    // List of affiliations.
    pub affiliations: Option<HashSet<String>>,
    // Author's own homepage.
    pub homepage: Option<String>,
    // Author's total publications count.
    pub paper_count: Option<u64>,
    // Author's total citations count.
    pub citation_count: Option<u64>,
    // Author's h-index.
    pub h_index: Option<u64>,
}

impl From<AuthorWithPapers> for Author {
    fn from(author: AuthorWithPapers) -> Author {
        author.author
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct AuthorExternalId {
    #[serde(rename = "DBLP")]
    dblp: Option<HashSet<String>>,
    #[serde(rename = "ORCID")]
    orcid: Option<String>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AuthorWithPapers {
    // Author's info.
    #[serde(flatten)]
    pub author: Author,
    pub papers: Option<Vec<BasePaper>>,
}
