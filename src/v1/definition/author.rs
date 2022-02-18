use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::batch::{Batch, SearchBatch};
use super::paper::BasePaper;
use super::Batched;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorBatch(Batch<AuthorWithPapers>);

impl Batched<AuthorWithPapers> for AuthorBatch {
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

impl From<AuthorBatch> for Vec<AuthorWithPapers> {
    fn from(batch: AuthorBatch) -> Vec<AuthorWithPapers> {
        Vec::from(batch.0)
    }
}

impl AsRef<Vec<AuthorWithPapers>> for AuthorBatch {
    fn as_ref(&self) -> &Vec<AuthorWithPapers> {
        self.0.as_ref()
    }
}

impl AsMut<Vec<AuthorWithPapers>> for AuthorBatch {
    fn as_mut(&mut self) -> &mut Vec<AuthorWithPapers> {
        self.0.as_mut()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorSearchBatch(SearchBatch<AuthorWithPapers>);

impl AuthorSearchBatch {
    pub fn total(&self) -> u64 {
        self.0.total
    }
}

impl Batched<AuthorWithPapers> for AuthorSearchBatch {
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

impl From<AuthorSearchBatch> for Vec<AuthorWithPapers> {
    fn from(batch: AuthorSearchBatch) -> Vec<AuthorWithPapers> {
        Vec::from(batch.0)
    }
}

impl AsRef<Vec<AuthorWithPapers>> for AuthorSearchBatch {
    fn as_ref(&self) -> &Vec<AuthorWithPapers> {
        self.0.as_ref()
    }
}

impl AsMut<Vec<AuthorWithPapers>> for AuthorSearchBatch {
    fn as_mut(&mut self) -> &mut Vec<AuthorWithPapers> {
        self.0.as_mut()
    }
}
