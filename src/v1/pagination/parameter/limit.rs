use std::fmt::{Display, Formatter, Result as FmtResult};

use super::constrained::{ConstrainedU64, MaxError, MinError, RangeBoundError};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Serialize)]
pub(in crate::v1::pagination) struct Limit {
    limit: ConstrainedU64<{ Limit::MIN }, { Limit::MAX }, { Limit::DEF }>,
}

impl Limit {
    pub(in crate::v1::pagination) const MAX: u64 = 100;
    const MIN: u64 = 1;
    const DEF: u64 = Self::MAX;

    #[inline]
    pub(in crate::v1::pagination) fn new(limit: u64) -> Result<Self, LimitBoundError> {
        Ok(Limit { limit: ConstrainedU64::new(limit)? })
    }

    #[inline]
    pub(in crate::v1::pagination) fn set(&mut self, limit: u64) -> Result<(), LimitBoundError> {
        Ok(self.limit.set(limit)?)
    }

    #[inline]
    pub(in crate::v1::pagination) fn available(limit: u64) -> Option<Self> {
        Some(Limit { limit: ConstrainedU64::available(limit)? })
    }

    #[inline]
    pub(in crate::v1::pagination) fn get(&self) -> u64 {
        self.limit.get()
    }
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
pub struct LimitMinError(MinError<{ Limit::MIN }>);

impl From<MinError<{ Limit::MIN }>> for LimitMinError {
    fn from(err: MinError<{ Limit::MIN }>) -> Self {
        Self(err)
    }
}

impl Display for LimitMinError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "`limit` {}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
pub struct LimitMaxError(MaxError<{ Limit::MAX }>);

impl From<MaxError<{ Limit::MAX }>> for LimitMaxError {
    fn from(err: MaxError<{ Limit::MAX }>) -> Self {
        Self(err)
    }
}

impl Display for LimitMaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "`limit` {}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
pub enum LimitBoundError {
    #[error(transparent)]
    Min(#[from] LimitMinError),
    #[error(transparent)]
    Max(#[from] LimitMaxError),
}

impl From<RangeBoundError<{ Limit::MIN }, { Limit::MAX }>> for LimitBoundError {
    fn from(err: RangeBoundError<{ Limit::MIN }, { Limit::MAX }>) -> Self {
        match err {
            RangeBoundError::Min(min) => Self::Min(LimitMinError::from(min)),
            RangeBoundError::Max(max) => Self::Max(LimitMaxError::from(max)),
        }
    }
}

extern crate static_assertions as sa;
// Compile time assertions for `Limit` based on current web API constraints.
//
// Must not compile if Limit::MIN is lower than 1.
sa::const_assert!(Limit::MIN >= 1);
// Must not compile if Limit::MAX is greater than 100.
sa::const_assert!(Limit::MAX <= 100);

// Compile time assertions for for `Limit` that must hold
// true regardless of what the current API accepts.
//
// Must not compile if Limit::MAX is lower than Limit::MIN.
sa::const_assert!(Limit::MAX >= Limit::MIN);
// Must not compile if Limit::DEF is out of MIN/MAX range.
sa::const_assert!(Limit::MIN <= Limit::DEF && Limit::DEF <= Limit::MAX);

#[cfg(test)]
mod tests {
    use super::*;

    fn in_range_values() -> impl Iterator<Item = u64> {
        let values = [Limit::MAX, Limit::MIN, (Limit::MAX.saturating_add(Limit::MIN)) / 2];
        values.into_iter()
    }

    fn out_of_range_values() -> impl Iterator<Item = u64> {
        let mut values = Vec::with_capacity(2);

        if Limit::MIN > u64::MIN {
            let lower = Limit::MIN.checked_sub(1).expect("Limit::MIN greater than 0");
            values.push(lower);
        }

        if Limit::MAX < u64::MAX {
            let greater = Limit::MAX.checked_add(1).expect("Limit::MAX lower than u64::MAX");
            values.push(greater);
        }

        values.into_iter()
    }

    fn available_in_range_values() -> impl Iterator<Item = u64> {
        let mut values = vec![];

        if Limit::MAX > Limit::MIN {
            values.push(Limit::MAX);
        }

        if Limit::MIN < u64::MAX {
            let greater_than_min =
                Limit::MIN.checked_add(1).expect("Limit::MIN lower than u64::MAX");
            values.push(greater_than_min);
        }

        values.into_iter()
    }

    fn available_lower_equal_to_min() -> impl Iterator<Item = u64> {
        let mut values = vec![Limit::MIN];

        if Limit::MIN > u64::MIN {
            let lower_than_min = Limit::MIN.checked_sub(1).expect("Limit::MIN greater than 0");
            values.push(lower_than_min);
        }

        values.into_iter()
    }

    #[test]
    fn new_assigns_to_wrapped_value() {
        for value in in_range_values() {
            let limit = Limit::new(value).expect("valid limit input");
            assert_eq!(value, limit.limit.get());
        }
    }

    #[test]
    fn new_fails_out_of_range_input() {
        for value in out_of_range_values() {
            match Limit::new(value) {
                Err(_) => (),
                Ok(_) => panic!("new must fail if limit is out of valid range"),
            }
        }
    }

    #[test]
    fn set_changes_wrapped_value() {
        for value in in_range_values() {
            let mut limit = Limit::default();
            limit.set(value).expect("valid limit input");
            assert_eq!(value, limit.limit.get());
        }
    }

    #[test]
    fn set_fails_out_of_range_input() {
        let mut limit = Limit::default();
        for value in out_of_range_values() {
            match limit.set(value) {
                Err(_) => (),
                Ok(_) => panic!("set must fail if limit is out of valid range"),
            }
        }
    }

    #[test]
    fn get_returns_wrapped_value() {
        for value in in_range_values() {
            let limit = Limit::new(value).expect("valid limit input");
            assert_eq!(value, limit.get());
        }
    }

    #[test]
    fn available_in_range_succeeds() {
        for value in available_in_range_values() {
            match Limit::available(value) {
                Some(limit) => assert_eq!(limit.get(), value),
                None => panic!("must succeed and return a Some"),
            }
        }
    }

    #[test]
    fn available_lower_or_equal_to_min_fails() {
        for value in available_lower_equal_to_min() {
            match Limit::available(value) {
                None => (),
                Some(_) => panic!("must fail and return a None"),
            }
        }
    }

    #[test]
    fn available_greater_than_max_returns_max() {
        if Limit::MAX < u64::MAX {
            let greater = Limit::MAX.checked_add(1).expect("Limit::MAX lower than u64::MAX");
            match Limit::available(greater) {
                Some(limit) => assert_eq!(limit.get(), Limit::MAX),
                None => panic!("must succeed and return a Some"),
            }
        }
    }

    #[test]
    fn check_limit_min_err_display() {
        if Limit::MIN > 0 {
            let lower = Limit::MIN.checked_sub(1).expect("Limit::MIN greater than 0");
            let limit = Limit::new(lower);

            match limit {
                Err(err @ LimitBoundError::Min(LimitMinError(MinError))) => {
                    // assert correct Display impl.
                    assert_eq!(
                        format!("`limit` must be greater or equal to {}", Limit::MIN),
                        err.to_string()
                    )
                }
                _ => panic!("must fail if lower than Limit::MIN"),
            }
        }
    }

    #[test]
    fn check_limit_max_err_display() {
        if Limit::MAX < u64::MAX {
            let greater = Limit::MAX.checked_add(1).expect("Limit::MAX lower than u64::MAX");
            let limit = Limit::new(greater);

            match limit {
                Err(err @ LimitBoundError::Max(LimitMaxError(MaxError))) => {
                    // assert correct Display impl.
                    assert_eq!(
                        format!("`limit` must be lower or equal to {}", Limit::MAX),
                        err.to_string()
                    )
                }
                _ => panic!("must fail if greater than Limit::MAX"),
            }
        }
    }
}
