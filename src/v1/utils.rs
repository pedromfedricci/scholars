use super::parameter::{
    AuthorField, AuthorInfoField, AuthorWithPapersField, BasePaperField, FullPaperField,
    PaperField, PaperInfoField, PaperWithLinksField,
};

const PAPER_INFO_FIELDS: [PaperInfoField; 6] = [
    PaperInfoField::PaperId,
    PaperInfoField::Url,
    PaperInfoField::Title,
    PaperInfoField::Venue,
    PaperInfoField::Year,
    PaperInfoField::Authors,
];

const AUTHOR_INFO_FIELDS: [AuthorInfoField; 2] = [AuthorInfoField::AuthorId, AuthorInfoField::Name];

const BASE_PAPER_FIELDS: [BasePaperField; 7] = [
    // BasePaperField::Info(PaperInfoField)
    BasePaperField::ExternalIds,
    BasePaperField::Abstract,
    BasePaperField::ReferenceCount,
    BasePaperField::CitationCount,
    BasePaperField::InfluentialCitationCount,
    BasePaperField::IsOpenAccess,
    BasePaperField::FieldsOfStudy,
];

const AUTHOR_FIELDS: [AuthorField; 8] = [
    // AuthorField::Info(AuthorInfoField)
    AuthorField::ExternalIds,
    AuthorField::Url,
    AuthorField::Aliases,
    AuthorField::Affiliations,
    AuthorField::Homepage,
    AuthorField::PaperCount,
    AuthorField::CitationCount,
    AuthorField::HIndex,
];

const PAPER_FIELDS: [PaperField; 3] = [
    // PaperField::Base(BasePaperField)
    PaperField::Contexts,
    PaperField::Intents,
    PaperField::IsInfluential,
];

const FULL_PAPER_FIELDS: [FullPaperField; 2] = [
    // FullPaperField::Base(BasePaperField)
    // FullPaperField::Authors(Option<AuthorField>)
    // FullPaperField::Citations(Option<PaperInfoField>)
    // FullPaperField::References(Option<PaperInfoField>)
    FullPaperField::Embedding,
    FullPaperField::Tldr,
];

/// An [`Iterator`] with entries for all possible instancies of [`AuthorInfoField`] enum.
pub fn all_author_info_fields() -> impl Iterator<Item = AuthorInfoField> {
    AUTHOR_INFO_FIELDS.into_iter()
}

fn exclusive_author_fields() -> impl Iterator<Item = AuthorField> {
    AUTHOR_FIELDS.into_iter()
}

/// An [`Iterator`] with entries for all possible instancies of [`AuthorField`] enum.
pub fn all_author_fields() -> impl Iterator<Item = AuthorField> {
    all_author_info_fields().map(AuthorField::from).chain(exclusive_author_fields())
}

/// An [`Iterator`] with entries for all possible instancies of [`PaperInfoField`] enum.
pub fn all_paper_info_fields() -> impl Iterator<Item = PaperInfoField> {
    PAPER_INFO_FIELDS.into_iter()
}

fn exclusive_base_paper_fields() -> impl Iterator<Item = BasePaperField> {
    BASE_PAPER_FIELDS.into_iter()
}

/// An [`Iterator`] with entries for all possible instancies of [`BasePaperField`] enum.
pub fn all_base_paper_fields() -> impl Iterator<Item = BasePaperField> {
    exclusive_base_paper_fields().chain(all_paper_info_fields().map(BasePaperField::from))
}

fn exclusive_paper_fields() -> impl Iterator<Item = PaperField> {
    PAPER_FIELDS.into_iter()
}

/// An [`Iterator`] with entries for all possible instancies of [`PaperField`] enum.
pub fn all_paper_fields() -> impl Iterator<Item = PaperField> {
    exclusive_paper_fields().chain(all_base_paper_fields().map(PaperField::from))
}

/// An [`Iterator`] with entries for all possible instancies of [`AuthorWithPapersField`] enum.
pub fn all_author_with_papers_fields() -> impl Iterator<Item = AuthorWithPapersField> {
    all_author_fields()
        .map(AuthorWithPapersField::from)
        .chain(all_base_paper_fields().map(AuthorWithPapersField::from))
}

/// An [`Iterator`] with entries for all possible instancies of [`PaperWithLinksField`] enum.
pub fn all_paper_with_links_fields() -> impl Iterator<Item = PaperWithLinksField> {
    all_base_paper_fields()
        .map(PaperWithLinksField::from)
        .chain(all_author_info_fields().map(PaperWithLinksField::from))
        .chain(all_paper_info_fields().map(PaperWithLinksField::references_from))
        .chain(all_paper_info_fields().map(PaperWithLinksField::citations_from))
}

fn exclusive_full_paper_fields() -> impl Iterator<Item = FullPaperField> {
    FULL_PAPER_FIELDS.into_iter()
}

/// An [`Iterator`] with entries for all possible instancies of [`FullPaperField`] enum.
pub fn all_full_paper_fields() -> impl Iterator<Item = FullPaperField> {
    exclusive_full_paper_fields()
        .chain(all_author_fields().map(FullPaperField::from))
        .chain(all_base_paper_fields().map(FullPaperField::from))
        .chain(all_paper_info_fields().map(FullPaperField::citations_from))
        .chain(all_paper_info_fields().map(FullPaperField::references_from))
}

/// An [`Iterator`] with entries for all possible variants of [`PaperField`]
/// enum with the exception of the [`PaperField::Base`] variant.
/// Users can extend this set with [`PaperField::Base`] instances
/// by providing an input that can iterate over [`BasePaperField`] values.
pub fn paper_fields_with(
    fields: impl IntoIterator<Item = BasePaperField>,
) -> impl Iterator<Item = PaperField> {
    exclusive_paper_fields().chain(fields.into_iter().map(PaperField::from))
}

/// An [`Iterator`] with entries for all possible variants of [`AuthorWithPapersField`]
/// enum with the exception of the [`AuthorWithPapersField::Papers`] variant.
/// Users can extend this set with [`AuthorWithPapersField::Papers`] instances
/// by providing an input that can iterate over [`BasePaperField`] values.
pub fn author_with_papers_fields_with(
    fields: impl IntoIterator<Item = BasePaperField>,
) -> impl Iterator<Item = AuthorWithPapersField> {
    all_author_fields()
        .map(AuthorWithPapersField::from)
        .chain(fields.into_iter().map(AuthorWithPapersField::from))
}
