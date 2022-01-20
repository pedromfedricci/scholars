use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use super::author::{Author, AuthorInfo};
use super::batch::{Batch, SearchBatch};
use super::embedding::Embedding;
use super::tldr::Tldr;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PaperInfo {
    // Papers's ID in Semantic Scholar.
    // paperId field is always included in the response.
    // API can return null for the paperId field.
    pub paper_id: Option<String>,
    // Paper's URL on the Semantic Scholar website.
    pub url: Option<String>,
    // Paper's title.
    // This field will be provided if no fields are specified.
    pub title: Option<String>,
    // Paper's publicaton venue.
    pub venue: Option<String>,
    // Paper's publication year.
    pub year: Option<u64>,
    // Paper's authors.
    // Up to 500 authors will be returned in the response.
    pub authors: Option<HashSet<AuthorInfo>>,
}

impl From<BasePaper> for PaperInfo {
    fn from(paper: BasePaper) -> PaperInfo {
        paper.info
    }
}

impl From<PaperWithLinks> for PaperInfo {
    fn from(paper: PaperWithLinks) -> PaperInfo {
        paper.base.info
    }
}

impl From<FullPaper> for PaperInfo {
    fn from(paper: FullPaper) -> PaperInfo {
        paper.base.info
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BasePaper {
    // Paper's info.
    #[serde(flatten)]
    pub info: PaperInfo,
    // Other catalog IDs for this paper, if known.
    // Supports ArXiv, MAG, ACL, PubMed, Medline, PubMedCentral, DBLP, DOI.
    pub external_ids: Option<PaperExternalId>,
    // Paper's abstract.
    #[serde(rename = "abstract")]
    pub r#abstract: Option<String>,
    // Paper's total reference count.
    pub reference_count: Option<u64>,
    // Paper's total citation count.
    pub citation_count: Option<u64>,
    // See: https://www.semanticscholar.org/faq#influential-citations.
    pub influential_citation_count: Option<u64>,
    // See: https://www.openaccess.nl/en/what-is-open-access.
    pub is_open_access: Option<bool>,
    // A list of high-level academic categories.
    pub fields_of_study: Option<HashSet<String>>,
}

impl From<PaperWithLinks> for BasePaper {
    fn from(paper: PaperWithLinks) -> BasePaper {
        paper.base
    }
}

impl From<FullPaper> for BasePaper {
    fn from(paper: FullPaper) -> BasePaper {
        paper.base
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct PaperExternalId {
    #[serde(rename = "ArXiv")]
    pub ar_xiv: Option<String>,
    #[serde(rename = "MAG")]
    pub mag: Option<String>,
    #[serde(rename = "ACL")]
    pub acl: Option<String>,
    #[serde(rename = "PubMed")]
    pub pub_med: Option<String>,
    #[serde(rename = "Medline")]
    pub medline: Option<String>,
    #[serde(rename = "PubMedCentral")]
    pub pub_med_central: Option<String>,
    #[serde(rename = "DBLP")]
    pub dblp: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    #[serde(rename = "CorpusId")]
    pub corpus_id: Option<u64>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PaperWithLinks {
    #[serde(flatten)]
    pub base: BasePaper,
    pub authors: HashSet<AuthorInfo>,
    pub citations: Vec<PaperInfo>,
    pub references: Vec<PaperInfo>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FullPaper {
    #[serde(flatten)]
    pub base: BasePaper,
    pub authors: Vec<Author>,
    pub citations: Vec<PaperInfo>,
    pub references: Vec<PaperInfo>,
    pub embedding: Option<Embedding>,
    pub tldr: Option<Tldr>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaperBatch<T = PaperWithLinks>(Batch<T>);

impl<T> PaperBatch<T> {
    pub fn get_offset(&self) -> u64 {
        self.0.offset
    }

    pub fn get_next(&self) -> Option<u64> {
        self.0.next
    }
}

impl<T> From<PaperBatch<T>> for Vec<T>
where
    T: From<PaperWithLinks>,
{
    fn from(batch: PaperBatch<T>) -> Vec<T> {
        Vec::from(batch.0)
    }
}

impl<T> AsRef<[T]> for PaperBatch<T>
where
    T: From<PaperWithLinks>,
{
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaperSearchBatch<T = BasePaper>(SearchBatch<T>);

impl<T> PaperSearchBatch<T> {
    pub fn get_offset(&self) -> u64 {
        self.0.batch.offset
    }

    pub fn get_next(&self) -> Option<u64> {
        self.0.batch.next
    }

    pub fn get_total(&self) -> u64 {
        self.0.total
    }
}

impl<T> From<PaperSearchBatch<T>> for Vec<T>
where
    T: From<BasePaper>,
{
    fn from(batch: PaperSearchBatch<T>) -> Vec<T> {
        Vec::from(batch.0)
    }
}

impl<T> AsRef<[T]> for PaperSearchBatch<T>
where
    T: From<BasePaper>,
{
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}
