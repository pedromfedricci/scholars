use std::borrow::Borrow;

use serde::Serialize;
use url::Url;

/// A struct that represents a URL encoded query string.
pub struct UrlEncodedQuery<'a> {
    urlencoded: form_urlencoded::Serializer<'a, String>,
}

impl Default for UrlEncodedQuery<'_> {
    fn default() -> Self {
        Self { urlencoded: form_urlencoded::Serializer::new(String::new()) }
    }
}

impl<'a> UrlEncodedQuery<'a> {
    /// Create a [`serde_urlencoded::Serializer`] from the inner [`form_urlencoded::Serializer`].
    pub fn serializer<'b>(&'b mut self) -> serde_urlencoded::Serializer<'a, 'b, String> {
        serde_urlencoded::Serializer::new(&mut self.urlencoded)
    }
}

impl UrlEncodedQuery<'_> {
    /// Create a new, empty URL encoded query string.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new URL encoded query from a initial, serializable value.
    pub fn with(value: &impl Serialize) -> Result<Self, serde_urlencoded::ser::Error> {
        let mut urlencoded = Self::new();
        value.serialize(urlencoded.serializer())?;
        Ok(urlencoded)
    }

    /// Set an [`Url`] by consuming [`UrlEncodedQuery`] and serilazing
    /// it into a [`String`], and setting the [`Url`] query with it.
    pub fn set_url(mut self, url: &mut Url) {
        // Can't call any mutable function from inner `urlencoded:Serializer`
        // after the `finish` call, or else will panic.
        // `set_url` consumes `self` in order to prevent any further calls.
        url.set_query(Some(&self.urlencoded.finish()))
    }

    /// Serialize and append a name/value pair.
    pub fn append_pair(&mut self, name: &str, value: &str) -> &mut Self {
        self.urlencoded.append_pair(name, value);
        self
    }

    /// Serialize and append the name of a parameter without any value.
    pub fn append_key_only(&mut self, name: &str) -> &mut Self {
        self.urlencoded.append_key_only(name);
        self
    }

    /// Remove any existing name/value pair.
    pub fn clear(&mut self) -> &mut Self {
        self.urlencoded.clear();
        self
    }

    /// Serialize and append a number of name/value pairs.
    ///
    /// This simply calls `append_pair` repeatedly.
    /// This can be more convenient, so the user doesn’t need to introduce a block
    /// to limit the scope of [`Serializer`]'s borrow of its string.
    pub fn extend_pairs<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.urlencoded.extend_pairs(iter);
        self
    }

    /// Serialize and append a number of names without values.
    ///
    /// This simply calls `append_key_only` repeatedly.
    /// This can be more convenient, so the user doesn’t need to introduce
    /// a block to limit the scope of `Serializer`'s borrow of its string.
    pub fn extend_keys_only<I, K>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Borrow<K>,
        K: AsRef<str>,
    {
        self.urlencoded.extend_keys_only(iter);
        self
    }
}
