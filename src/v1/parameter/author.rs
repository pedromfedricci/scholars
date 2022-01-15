use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::Serialize;

use super::paper::{BasePaperField, PaperInfoField};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthorInfoField {
    AuthorId,
    Name,
}

impl Display for AuthorInfoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::AuthorId => write!(f, "authorId"),
            Self::Name => write!(f, "name"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthorField {
    Info(AuthorInfoField),
    ExternalIds,
    Url,
    Aliases,
    Affiliations,
    Homepage,
    PaperCount,
    CitationCount,
    HIndex,
}

impl Display for AuthorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Info(field) => write!(f, "{}", field),
            Self::ExternalIds => write!(f, "externalIds"),
            Self::Url => write!(f, "url"),
            Self::Aliases => write!(f, "aliases"),
            Self::Affiliations => write!(f, "affiliations"),
            Self::Homepage => write!(f, "homepage"),
            Self::PaperCount => write!(f, "paperCount"),
            Self::CitationCount => write!(f, "citationCount"),
            Self::HIndex => write!(f, "hIndex"),
        }
    }
}

impl From<AuthorInfoField> for AuthorField {
    fn from(field: AuthorInfoField) -> AuthorField {
        AuthorField::Info(field)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthorWithPapersField {
    Author(AuthorField),
    Papers(Option<BasePaperField>),
}

impl Display for AuthorWithPapersField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Author(field) => write!(f, "{}", field),
            Self::Papers(Some(field)) => write!(f, "papers.{}", field),
            // FIXME
            // Write `papers.title` to get the same (expected)
            // result as writing a single (dotless) `papers` field.
            // Using the `papers` field should work, as
            // indicated by the API doc, but doing so
            // will return a 500 InternalServerError response =/.
            //
            // Self::Papers(None) => write!(f, "papers"),
            //
            Self::Papers(None) => {
                write!(f, "papers.{}", BasePaperField::Info(PaperInfoField::Title))
            }
        }
    }
}

impl From<AuthorField> for AuthorWithPapersField {
    fn from(field: AuthorField) -> AuthorWithPapersField {
        AuthorWithPapersField::Author(field)
    }
}

impl From<AuthorInfoField> for AuthorWithPapersField {
    fn from(field: AuthorInfoField) -> AuthorWithPapersField {
        AuthorWithPapersField::Author(AuthorField::from(field))
    }
}

impl From<BasePaperField> for AuthorWithPapersField {
    fn from(field: BasePaperField) -> AuthorWithPapersField {
        AuthorWithPapersField::Papers(Some(field))
    }
}

impl From<PaperInfoField> for AuthorWithPapersField {
    fn from(field: PaperInfoField) -> AuthorWithPapersField {
        AuthorWithPapersField::Papers(Some(BasePaperField::from(field)))
    }
}
