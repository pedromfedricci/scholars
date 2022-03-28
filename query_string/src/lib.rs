#![forbid(unsafe_code)]

use std::borrow::Borrow;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use form_urlencoded::Serializer as UrlEncoder;
use serde::Serialize;
use url::Url;

/// A struct that represents a URL encoded query string.
pub struct QueryString {
    urlencoder: UrlEncoder<'static, String>,
}

impl QueryString {
    /// Creates a new, empty URL encoded query string.
    pub fn new() -> QueryString {
        QueryString::default()
    }

    /// Creates a new URL encoded query string from a initial, serializable value.
    ///
    /// Fails if cannot serialize to `application/x-www-form-urlencoded` format.
    pub fn try_new(value: &impl Serialize) -> Result<QueryString, serde_urlencoded::ser::Error> {
        let mut query = Self::default();
        value.serialize(query.serializer())?;
        Ok(query)
    }

    /// Sets an [`Url`] query string with the produced URL encoded [`String`].
    pub fn set_url(mut self, url: &mut Url) {
        // Can't call any mutable function from inner `form_urlenconded:Serializer`
        // after the `finish` call, or else will panic.
        // `set_url` consumes `self` in order to prevent any further calls.
        url.set_query(Some(&self.urlencoder.finish()))
    }

    /// Serilize and append a serializable value.
    /// Fails if cannot serialize to `application/x-www-form-urlencoded` format.
    pub fn try_append(
        &mut self,
        value: &impl Serialize,
    ) -> Result<&mut QueryString, serde_urlencoded::ser::Error> {
        value.serialize(self.serializer())?;
        Ok(self)
    }

    /// Serialize and append a name/value pair.
    pub fn append_pair(&mut self, name: &str, value: &str) -> &mut QueryString {
        self.urlencoder.append_pair(name, value);
        self
    }

    /// Serialize and append the name of a parameter without any value.
    pub fn append_key_only(&mut self, name: &str) -> &mut QueryString {
        self.urlencoder.append_key_only(name);
        self
    }

    /// Serialize and append a number of serializable values.
    /// This simply calls `try_extend` repeatedly.
    ///
    /// Fails if cannot serialize a single value to `application/x-www-form-urlencoded` format.
    pub fn try_extend<I>(&mut self, iter: I) -> Result<&mut QueryString, TryExtendError<I::Item>>
    where
        I: IntoIterator,
        I::Item: Serialize,
    {
        for value in iter {
            self.try_append(&value).map_err(|source| TryExtendError { source, value })?;
        }
        Ok(self)
    }

    /// Serialize and append a number of name/value pairs.
    /// This simply calls `append_pair` repeatedly.
    pub fn extend_pairs<I, K, V>(&mut self, iter: I) -> &mut QueryString
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.urlencoder.extend_pairs(iter);
        self
    }

    /// Serialize and append a number of names without values.
    /// This simply calls `append_key_only` repeatedly.
    pub fn extend_keys_only<I, K>(&mut self, iter: I) -> &mut QueryString
    where
        I: IntoIterator,
        I::Item: Borrow<K>,
        K: AsRef<str>,
    {
        self.urlencoder.extend_keys_only(iter);
        self
    }

    /// Removes any existing name/value pair.
    pub fn clear(&mut self) -> &mut QueryString {
        self.urlencoder.clear();
        self
    }

    /// Creates a [`serde_urlencoded::Serializer`] from the inner [`UrlEncoder`].
    #[inline]
    fn serializer<'a>(&'a mut self) -> serde_urlencoded::Serializer<'static, 'a, String> {
        serde_urlencoded::Serializer::new(&mut self.urlencoder)
    }
}

impl Default for QueryString {
    /// Create a new, empty URL encoded query string.
    fn default() -> Self {
        Self { urlencoder: UrlEncoder::new(String::new()) }
    }
}

impl From<QueryString> for String {
    fn from(mut query: QueryString) -> String {
        query.urlencoder.finish()
    }
}

/// An Error type that holds the value that could not be serialized during the `try_extend` operation.
///
/// The lower-level source error is provided by the `source` method.
#[derive(Debug)]
pub struct TryExtendError<T: Serialize> {
    source: serde_urlencoded::ser::Error,
    value: T,
}

impl<T: Serialize> TryExtendError<T> {
    /// Returns a reference to the value that could not be serialized to `application/x-www-form-urlencoded` format.
    #[inline]
    pub fn unserialized(&self) -> &T {
        &self.value
    }

    /// Returns the value that could not be serialized to `application/x-www-form-urlencoded` format.
    #[inline]
    pub fn into_unserialized(self) -> T {
        self.value
    }
}

impl<T: Serialize> Display for TryExtendError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let value = &self.value;
        write!(f, "could not serialize the following value while extending: {value}")
    }
}

impl<T: Serialize> StdError for TryExtendError<T>
where
    T: Debug + Display,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

impl<T: Serialize> From<TryExtendError<T>> for serde_urlencoded::ser::Error {
    fn from(err: TryExtendError<T>) -> serde_urlencoded::ser::Error {
        err.source
    }
}
