mod parameter;

use parameter::{Limit, LimitBoundError, Offset};

/// Pagination options for querying a number of endpoint's results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Results {
    /// Return all results.
    All,
    /// Limit to a number of results.
    Limit(u64),
}

impl Default for Results {
    fn default() -> Self {
        Results::All
    }
}

/// API will return at max the first 10_000 results from any query.
/// Even if the total number of results is greater than 10_000,
/// you cannot query any result beyond that point.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(flatten)]
    offset: Offset,
    #[serde(flatten)]
    limit: Limit,
}

impl Page {
    // Max number of results that any paged endpoint will return.
    pub(in crate::v1) const RANGE_LIMIT: u64 = Self::SUM_MAX + 1;

    // The sum of `offset` and `limit` must be <= SUM_MAX.
    // We can only query the first SUM_MAX results for any paged endpoint.
    // Currently, that range is at: 0 - 9_999, that means, the first 10_000.
    // Any result that is out of this range can't be returned.
    const SUM_MAX: u64 = 9_999;

    pub fn new(offset: u64, limit: u64) -> Result<Self, PaginationError> {
        let limit = Limit::new(limit)?;
        let offset = Offset::new(offset);
        Self::is_valid_range(offset.get(), limit.get())?;
        Ok(Page { offset, limit })
    }

    pub fn with_offset(offset: u64) -> Result<Self, RangeBoundsError> {
        let limit = Limit::default();
        let offset = Offset::new(offset);
        Self::is_valid_range(offset.get(), limit.get())?;
        Ok(Page { offset, limit })
    }

    pub fn with_limit(limit: u64) -> Result<Self, PaginationError> {
        Self::new(Offset::DEF, limit)
    }

    pub fn set_offset(&mut self, offset: u64) -> Result<(), RangeBoundsError> {
        Self::is_valid_range(offset, self.limit.get())?;
        self.offset.set(offset);
        Ok(())
    }

    pub fn set_limit(&mut self, limit: u64) -> Result<(), PaginationError> {
        Self::is_valid_range(self.offset.get(), limit)?;
        self.limit.set(limit)?;
        Ok(())
    }

    pub fn next_page(&mut self, next: u64) -> Result<(), RangeBoundsError> {
        match self.set_offset(next) {
            Err(RangeBoundsError { available: Some(limit), .. }) => {
                Self::is_valid_range(next, limit.get())?;
                self.limit = limit;
                self.offset.set(next);
                Ok(())
            }
            result => result,
        }
    }

    // The sum of `offset` and `limit` must be <= Page::SUM_MAX.
    // API will return a error if not so.
    #[inline]
    fn is_valid_range(offset: u64, limit: u64) -> Result<(), RangeBoundsError> {
        match offset.checked_add(limit) {
            Some(sum) if sum <= Self::SUM_MAX => Ok(()),
            _ => {
                let candidate = Self::SUM_MAX.saturating_sub(offset);
                let available = Limit::available(candidate);
                Err(RangeBoundsError { offset, limit, available })
            }
        }
    }

    #[inline]
    pub fn get_offset(&self) -> u64 {
        self.offset.get()
    }

    #[inline]
    pub fn get_limit(&self) -> u64 {
        self.limit.get()
    }
}

pub(in crate::v1) trait Paged: AsRef<Page> + AsMut<Page> {
    fn set_limit(&mut self, limit: u64) -> Result<(), PaginationError> {
        self.as_mut().set_limit(limit)
    }

    fn next_page(&mut self, next: u64) -> Result<(), RangeBoundsError> {
        self.as_mut().next_page(next)
    }

    fn get_offset(&self) -> u64 {
        self.as_ref().get_offset()
    }

    fn get_limit(&self) -> u64 {
        self.as_ref().get_limit()
    }
}

impl<T: AsRef<Page> + AsMut<Page>> Paged for T {}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
pub enum PaginationError {
    #[error(transparent)]
    Range(#[from] RangeBoundsError),
    #[error(transparent)]
    Limit(#[from] LimitBoundError),
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
#[error("`offset` and `limit` sum must be lower or equal to {}, but provided: offset={offset} and limit={limit}", Page::SUM_MAX)]
pub struct RangeBoundsError {
    pub offset: u64,
    pub limit: u64,
    available: Option<Limit>,
}

extern crate static_assertions as sa;
// Compile time assertions for `Page` based on current web API constraints.
//
// Must not compile if Page::SUM_MAX is 0.
sa::const_assert!(Page::SUM_MAX > 0);
// Must not compile if Page::SUM_MAX is greater or equal to 10_000;
sa::const_assert!(Page::SUM_MAX < 10_000);

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn is_valid_range_succeeds_lower_or_equal_to_max() {
        let mut sums = vec![(Page::SUM_MAX, 0), (0, Page::SUM_MAX)];

        if Page::SUM_MAX > 0 {
            let lower = Page::SUM_MAX.checked_sub(1).unwrap();
            sums.extend_from_slice(&[(lower, 0), (0, lower), (lower, 1), (1, lower)]);
        }

        for sum in sums {
            match Page::is_valid_range(sum.0, sum.1) {
                Ok(_) => (),
                Err(_) => panic!(
                    "must succeed when sum of `offset` and `limit` is lower or equal to Page::SUM_MAX"
                ),
            }
        }
    }

    #[test]
    fn is_valid_range_fails_greater_than_max() {
        let mut sums = vec![(Page::SUM_MAX, 1), (1, Page::SUM_MAX)];

        if Page::SUM_MAX < u64::MAX {
            let greater = Page::SUM_MAX.checked_add(1).unwrap();
            sums.extend_from_slice(&[(greater, 0), (0, greater), (greater, 1), (1, greater)]);
        }

        for sum in sums {
            match Page::is_valid_range(sum.0, sum.1) {
                Err(_) => (),
                Ok(_) => panic!(
                    "must fail when the sum of `offset` and `limit` is greater than Page::SUM_MAX"
                ),
            }
        }
    }

    #[test]
    fn is_valid_range_fails_no_available_next_limit() {
        let mut sums = vec![(Page::SUM_MAX, 1)];

        if Page::SUM_MAX < u64::MAX {
            let greater = Page::SUM_MAX.checked_add(1).unwrap();
            sums.extend_from_slice(&[(greater, 0), (greater, 1)]);
        }

        for sum in sums {
            match Page::is_valid_range(sum.0, sum.1) {
                Err(RangeBoundsError { available, .. }) => {
                    assert_eq!(available, None);
                }
                Ok(_) => panic!(
                    "must fail when the sum of `offset` and `limit` is greater than Page::SUM_MAX"
                ),
            }
        }
    }

    #[test]
    fn is_valid_range_fails_available_returns_max_limit() {
        let mut sums = vec![(1, Page::SUM_MAX)];

        if Page::SUM_MAX < u64::MAX {
            let greater = Page::SUM_MAX.checked_add(1).unwrap();
            sums.extend_from_slice(&[(0, greater), (1, greater)]);
        }

        for sum in sums {
            match Page::is_valid_range(sum.0, sum.1) {
                Err(RangeBoundsError { available, .. }) => {
                    let limit = available.expect("`available can't be greater than Limit::MAX`");
                    assert_eq!(limit.get(), Limit::MAX);
                }
                Ok(_) => panic!(
                    "must fail when the sum of `offset` and `limit` is greater than Page::SUM_MAX"
                ),
            }
        }
    }

    #[test]
    fn is_valid_range_does_not_overflow_u64_max_offset() {
        match Page::is_valid_range(u64::MAX, 1) {
            Err(RangeBoundsError { available, .. }) => {
                assert_eq!(available, None, "`available` must be None, must not overflow");
            }
            // If sum of u64::MAX and any other u64 value
            // is lower or equal to Page::SUM_MAX, than
            // Page::SUM_MAX itself is equal to u64::MAX.
            Ok(_) => assert_eq!(Page::SUM_MAX, u64::MAX),
        }
    }

    #[test]
    fn is_valid_range_does_not_overflow_u64_max_limit() {
        match Page::is_valid_range(1, u64::MAX) {
            Err(RangeBoundsError { available, .. }) => {
                let limit = available.expect("`available can't be greater than Limit::MAX`");
                assert_eq!(limit.get(), Limit::MAX);
            }
            // If sum of u64::MAX and any other u64 value
            // is lower or equal to Page::SUM_MAX, than
            // Page::SUM_MAX itself is equal to u64::MAX.
            Ok(_) => assert_eq!(Page::SUM_MAX, u64::MAX),
        }
    }

    #[test]
    fn page_urlencoded_serialization() {
        let page = Page::default();
        let offset = page.get_offset();
        let limit = page.get_limit();

        let mut urlencoder = form_urlencoded::Serializer::new(String::new());
        let serializer = serde_urlencoded::Serializer::new(&mut urlencoder);
        page.serialize(serializer).unwrap();
        let encoded = urlencoder.finish();

        assert_eq!(format!("offset={offset}&limit={limit}"), encoded);
    }
}
