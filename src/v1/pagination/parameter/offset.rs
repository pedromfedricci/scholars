use super::constrained::ConstrainedU64;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Serialize)]
pub(in crate::v1::pagination) struct Offset {
    offset: ConstrainedU64<{ Offset::MIN }, { Offset::MAX }, { Offset::DEF }>,
}

impl Offset {
    const MAX: u64 = u64::MAX;
    const MIN: u64 = 0;
    pub(in crate::v1::pagination) const DEF: u64 = Self::MIN;

    #[inline]
    pub(in crate::v1::pagination) fn new(offset: u64) -> Offset {
        Offset { offset: ConstrainedU64::new_infallible(offset) }
    }

    #[inline]
    pub(in crate::v1::pagination) fn set(&mut self, offset: u64) {
        self.offset.set_infallible(offset)
    }

    #[inline]
    pub(in crate::v1::pagination) fn get(&self) -> u64 {
        self.offset.get()
    }
}

// use std::fmt::{Display, Formatter, Result as FmtResult};

// use super::constrained::{MaxError, MinError, RangeBoundError};

// #[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
// pub struct OffsetMinError(MinError<{ Offset::MIN }>);

// impl From<MinError<{ Offset::MIN }>> for OffsetMinError {
//     fn from(err: MinError<{ Offset::MIN }>) -> Self {
//         Self(err)
//     }
// }

// impl Display for OffsetMinError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "`offset` {}", self.0)
//     }
// }

// #[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
// pub struct OffsetMaxError(MaxError<{ Offset::MAX }>);

// impl From<MaxError<{ Offset::MAX }>> for OffsetMaxError {
//     fn from(err: MaxError<{ Offset::MAX }>) -> Self {
//         Self(err)
//     }
// }

// impl Display for OffsetMaxError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "`offset` {}", self.0)
//     }
// }

// #[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
// pub enum OffsetBoundError {
//     #[error(transparent)]
//     Min(#[from] OffsetMinError),
//     #[error(transparent)]
//     Max(#[from] OffsetMaxError),
// }

// impl From<RangeBoundError<{ Offset::MIN }, { Offset::MAX }>> for OffsetBoundError {
//     fn from(err: RangeBoundError<{ Offset::MIN }, { Offset::MAX }>) -> Self {
//         match err {
//             RangeBoundError::Min(min) => Self::Min(OffsetMinError::from(min)),
//             RangeBoundError::Max(max) => Self::Max(OffsetMaxError::from(max)),
//         }
//     }
// }

extern crate static_assertions as sa;
// Compile time assertions for `Offset` based on current web API constraints.
//
// Currently, the acceptable range is u64::MIN to u64::MAX (both inclusive),
// so `Offset` APIs are infallible and internally call ConstrainedU64 infallible APIs.
// If this is to be changed, `Offset` APIs will need to be refactored
// to be fallible just like `Limit` are.
sa::const_assert!(Offset::MIN == 0);
sa::const_assert!(Offset::MAX == u64::MAX);

// Compile time assertions for for `Offset` that must hold
// true regardless of what the current API accepts.
//
// Must not compile if Offset::MAX is lower than Offset::MIN.
#[allow(clippy::absurd_extreme_comparisons)]
const _MAX_GREATER_EQ_TO_MIN: bool = Offset::MAX >= Offset::MIN;
sa::const_assert!(_MAX_GREATER_EQ_TO_MIN);

// Must not compile if Limit::DEF is out of MIN/MAX range.
#[allow(clippy::absurd_extreme_comparisons)]
const _DEF_IN_RANGE_MIN_MAX: bool = Offset::MIN <= Offset::DEF && Offset::DEF <= Offset::MAX;
sa::const_assert!(_DEF_IN_RANGE_MIN_MAX);

#[cfg(test)]
mod tests {
    use super::*;

    fn in_range_offset() -> impl Iterator<Item = u64> {
        let values = [Offset::MAX, Offset::MIN, (Offset::MAX.saturating_add(Offset::MIN)) / 2];
        values.into_iter()
    }

    #[test]
    fn new_assigns_to_wrapped_value() {
        for value in in_range_offset() {
            let offset = Offset::new(value);
            assert_eq!(value, offset.offset.get());
        }
    }

    #[test]
    fn set_changes_wrapped_value() {
        for value in in_range_offset() {
            let mut offset = Offset::default();
            offset.set(value);
            assert_eq!(value, offset.offset.get());
        }
    }

    #[test]
    fn get_returns_wrapped_value() {
        for value in in_range_offset() {
            let offset = Offset::new(value);
            assert_eq!(value, offset.get());
        }
    }
}
