use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none, CommaSeparator, StringWithSeparator};

use crate::serialize::as_non_empty_string;
use crate::urlencoded::UrlEncodedQuery;
use crate::v1::pagination::{Page, Paged};
use crate::v1::parameter::{
    AuthorWithPapersField, BasePaperField, FullPaperField, PaperField, PaperWithLinksField,
};

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct FieldsParam<F>
where
    F: Clone + Debug + Display + Eq + Hash + PartialEq + Serialize,
{
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, F>>")]
    fields: Option<HashSet<F>>,
}

impl<F> FieldsParam<F>
where
    F: Clone + Debug + Display + Eq + Hash + PartialEq + Serialize,
{
    fn new<T>(fields: Option<impl IntoIterator<Item = T>>) -> FieldsParam<F>
    where
        T: Into<F>,
    {
        let fields = fields.map(|fields| fields.into_iter().map(Into::into).collect());
        FieldsParam { fields }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchParams<F>
where
    F: Clone + Debug + Display + Eq + Hash + PartialEq + Serialize,
{
    #[serde(flatten)]
    fields: FieldsParam<F>,
    // A plain-text search query string. No special query syntax is supported.
    #[serde(serialize_with = "as_non_empty_string")]
    query: String,
}

impl<F> SearchParams<F>
where
    F: Clone + Debug + Display + Eq + Hash + PartialEq + Serialize,
{
    fn new<T>(query: String, fields: Option<impl IntoIterator<Item = T>>) -> SearchParams<F>
    where
        T: Into<F>,
    {
        let fields = FieldsParam::new(fields);
        SearchParams { fields, query }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct PagedParams<T> {
    #[serde(flatten)]
    page: Page,
    #[serde(flatten)]
    params: T,
}

macro_rules! define_impl_params {
    ( $($param:ident : $field:ty),* ) => {$(
        #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
        pub struct $param(FieldsParam<$field>);

        impl $param {
            pub fn new<T>(fields: Option<impl IntoIterator<Item = T>>) -> $param
            where
                T: Into<$field>,
            {
                Self(FieldsParam::new(fields))
            }
        }
    )*};
}

define_impl_params! {
    PaperParams : FullPaperField,
    AuthorParams : AuthorWithPapersField
}

macro_rules! define_impl_paged_params {
    ( $($param:ident : $field:ty),* ) => {$(
        #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
        pub struct $param(PagedParams<FieldsParam<$field>>);

        impl $param {
            pub fn new<T>(fields: Option<impl IntoIterator<Item = T>>, page: Page) -> $param
            where
                T: Into<$field>,
            {
                let params = FieldsParam::new(fields);
                Self(PagedParams { params, page })
            }
        }
    )*};
}

define_impl_paged_params! {
    AuthorPapersParams : PaperWithLinksField,
    PaperAuthorsParams : AuthorWithPapersField,
    PaperCitationsParams : PaperField,
    PaperReferencesParams : PaperField
}

macro_rules! define_impl_search_params {
    ( $($param:ident : $field:ty),* ) => {$(
        #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
        pub struct $param(PagedParams<SearchParams<$field>>);

        impl $param {
            pub fn new<T>(query: String, fields: Option<impl IntoIterator<Item = T>>, page: Page) -> $param
            where
                T: Into<$field>,
            {
                let params = SearchParams::new(query, fields);
                Self(PagedParams { params, page })
            }
        }
    )*};
}

define_impl_search_params! {
    PaperSearchParams : BasePaperField,
    AuthorSearchParams : AuthorWithPapersField
}

static EXPECT_MSG: &str = "must be serializable by `serde_urlencoded::Serialzer`";

macro_rules! impl_from_params_for_urlencoded {
    ( $($type:ty),* ) => {$(
        impl<'a> From<&'a $type> for UrlEncodedQuery<'a> {
            fn from(params: &'a $type) -> UrlEncodedQuery<'a> {
                let mut urlencoded = UrlEncodedQuery::new();
                params.serialize(urlencoded.serializer()).expect(EXPECT_MSG);
                urlencoded
            }
        }
    )*};
}

impl_from_params_for_urlencoded! {
    AuthorParams,
    AuthorSearchParams,
    AuthorPapersParams,
    PaperParams,
    PaperSearchParams,
    PaperAuthorsParams,
    PaperCitationsParams,
    PaperReferencesParams
}

macro_rules! impl_paged_for {
    ( $($type:ty),* ) => {$(
        impl Paged for $type {
            #[inline]
            fn get_page(&self) -> &Page {
                &self.0.page
            }

            #[inline]
            fn get_page_mut(&mut self) -> &mut Page {
                &mut self.0.page
            }
        }
    )*};
}

impl_paged_for! {
    AuthorSearchParams,
    AuthorPapersParams,
    PaperSearchParams,
    PaperAuthorsParams,
    PaperCitationsParams,
    PaperReferencesParams
}
