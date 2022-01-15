use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::Serialize;

use super::author::{AuthorField, AuthorInfoField};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaperInfoField {
    PaperId,
    Url,
    Title,
    Venue,
    Year,
    Authors,
}

impl Display for PaperInfoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::PaperId => write!(f, "paperId"),
            Self::Url => write!(f, "url"),
            Self::Title => write!(f, "title"),
            Self::Venue => write!(f, "venue"),
            Self::Year => write!(f, "year"),
            Self::Authors => write!(f, "authors"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BasePaperField {
    Info(PaperInfoField),
    Abstract,
    ExternalIds,
    ReferenceCount,
    CitationCount,
    InfluentialCitationCount,
    IsOpenAccess,
    FieldsOfStudy,
}

impl Display for BasePaperField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Info(field) => write!(f, "{}", field),
            Self::Abstract => write!(f, "abstract"),
            Self::ExternalIds => write!(f, "externalIds"),
            Self::ReferenceCount => write!(f, "referenceCount"),
            Self::CitationCount => write!(f, "citationCount"),
            Self::InfluentialCitationCount => write!(f, "influentialCitationCount"),
            Self::IsOpenAccess => write!(f, "isOpenAccess"),
            Self::FieldsOfStudy => write!(f, "fieldsOfStudy"),
        }
    }
}

impl From<PaperInfoField> for BasePaperField {
    fn from(field: PaperInfoField) -> BasePaperField {
        BasePaperField::Info(field)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaperField {
    Base(BasePaperField),
    Contexts,
    Intents,
    IsInfluential,
}

impl Display for PaperField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Base(field) => write!(f, "{}", field),
            Self::Contexts => write!(f, "contexts"),
            Self::Intents => write!(f, "intents"),
            Self::IsInfluential => write!(f, "isInfluential"),
        }
    }
}

impl From<BasePaperField> for PaperField {
    fn from(field: BasePaperField) -> PaperField {
        PaperField::Base(field)
    }
}

impl From<PaperInfoField> for PaperField {
    fn from(field: PaperInfoField) -> PaperField {
        PaperField::Base(BasePaperField::from(field))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaperWithLinksField {
    Base(BasePaperField),
    Authors(Option<AuthorInfoField>),
    Citations(Option<PaperInfoField>),
    References(Option<PaperInfoField>),
}

impl Display for PaperWithLinksField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Base(field) => write!(f, "{}", field),
            Self::Authors(None) => write!(f, "authors"),
            Self::Authors(Some(field)) => write!(f, "authors.{}", field),
            Self::Citations(None) => write!(f, "citations"),
            Self::Citations(Some(field)) => write!(f, "citations.{}", field),
            Self::References(None) => write!(f, "references"),
            Self::References(Some(field)) => write!(f, "references.{}", field),
        }
    }
}

impl PaperWithLinksField {
    #[inline]
    pub fn citations_from(field: PaperInfoField) -> PaperWithLinksField {
        PaperWithLinksField::Citations(Some(field))
    }

    #[inline]
    pub fn references_from(field: PaperInfoField) -> PaperWithLinksField {
        PaperWithLinksField::References(Some(field))
    }
}

impl From<BasePaperField> for PaperWithLinksField {
    fn from(field: BasePaperField) -> PaperWithLinksField {
        PaperWithLinksField::Base(field)
    }
}

impl From<PaperInfoField> for PaperWithLinksField {
    fn from(field: PaperInfoField) -> PaperWithLinksField {
        PaperWithLinksField::Base(BasePaperField::from(field))
    }
}

impl From<AuthorInfoField> for PaperWithLinksField {
    fn from(field: AuthorInfoField) -> PaperWithLinksField {
        PaperWithLinksField::Authors(Some(field))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FullPaperField {
    Base(BasePaperField),
    Authors(Option<AuthorField>),
    Citations(Option<PaperInfoField>),
    References(Option<PaperInfoField>),
    Embedding,
    Tldr,
}

impl Display for FullPaperField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Base(field) => write!(f, "{}", field),
            Self::Authors(None) => write!(f, "authors"),
            Self::Authors(Some(field)) => write!(f, "authors.{}", field),
            Self::Citations(None) => write!(f, "citations"),
            Self::Citations(Some(field)) => write!(f, "citations.{}", field),
            Self::References(None) => write!(f, "references"),
            Self::References(Some(field)) => write!(f, "references.{}", field),
            Self::Embedding => write!(f, "embedding"),
            Self::Tldr => write!(f, "tldr"),
        }
    }
}

impl FullPaperField {
    #[inline]
    pub fn citations_from(field: PaperInfoField) -> FullPaperField {
        FullPaperField::Citations(Some(field))
    }

    #[inline]
    pub fn references_from(field: PaperInfoField) -> FullPaperField {
        FullPaperField::References(Some(field))
    }
}

impl From<BasePaperField> for FullPaperField {
    fn from(field: BasePaperField) -> FullPaperField {
        FullPaperField::Base(field)
    }
}

impl From<PaperInfoField> for FullPaperField {
    fn from(field: PaperInfoField) -> FullPaperField {
        FullPaperField::Base(BasePaperField::from(field))
    }
}

impl From<AuthorField> for FullPaperField {
    fn from(field: AuthorField) -> FullPaperField {
        FullPaperField::Authors(Some(field))
    }
}

impl From<AuthorInfoField> for FullPaperField {
    fn from(field: AuthorInfoField) -> FullPaperField {
        FullPaperField::Authors(Some(AuthorField::from(field)))
    }
}
