use std::fmt::{self, Formatter};

use serde::de::{Error, Visitor};
use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct StringOrU64;

impl<'de> DeserializeAs<'de, String> for StringOrU64 {
    fn deserialize_as<D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer)
    }
}

impl SerializeAs<String> for StringOrU64 {
    fn serialize_as<S>(source: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(source)
    }
}

/// Deserialize a possible u64 value as a String.
#[inline]
fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct ToStringVisitor;

    impl<'de> Visitor<'de> for ToStringVisitor {
        type Value = String;

        fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "either a string or a u64")
        }

        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(v.to_owned())
        }

        fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v.to_string())
        }
    }

    deserializer.deserialize_any(ToStringVisitor)
}

/// Serialize an empty string as a single whitespace string.
pub(crate) fn as_non_empty_string<S: Serializer>(string: &str, ser: S) -> Result<S::Ok, S::Error> {
    // Take the opportunity to trim as well.
    let string = string.trim();

    if string.is_empty() {
        // Serialize as a single whitespace if provided string is empty.
        ser.serialize_str(" ")
    } else {
        ser.serialize_str(string)
    }
}
