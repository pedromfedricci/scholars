mod limit;

use limit::{Limit, LimitBoundsError};

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

trait CheckedNonZero<Rhs = Self> {
    type Output;

    fn is_not_zero(&self) -> bool;

    fn checked_nonzero_sub(&self, rhs: Rhs) -> Self::Output;
}

impl CheckedNonZero for u64 {
    type Output = Option<u64>;

    fn is_not_zero(&self) -> bool {
        *self != 0
    }

    fn checked_nonzero_sub(&self, rhs: u64) -> Self::Output {
        self.checked_sub(rhs).filter(Self::is_not_zero)
    }
}

/// API will return at max the first 10_000 results from any query.
/// Even if the total number of results is greater than 10_000,
/// you cannot query any result beyond that point.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    offset: u64,
    #[serde(flatten)]
    limit: Limit,
}

impl Page {
    // API will return up to the first 10000 results from the search list.
    // You can't query any results beyond that point, API will return a error.
    pub(in crate::v1) const RANGE_LIMIT: u64 = Self::SUM_MAX + 1;

    // Offset's default starting position is 0.
    const OFFSET_DEFAULT: u64 = 0;

    // The sum of offset and limit must be < 10_000.
    const SUM_MAX: u64 = 9_999;

    pub fn new(offset: u64, limit: u64) -> Result<Self, PaginationError> {
        let limit = Limit::new(limit)?;
        Self::is_valid_range(offset, limit.get())?;
        Ok(Page { offset, limit })
    }

    pub fn with_offset(offset: u64) -> Result<Self, RangeBoundsError> {
        let limit = Limit::default();
        Self::is_valid_range(offset, limit.get())?;
        Ok(Page { offset, limit })
    }

    pub fn with_limit(limit: u64) -> Result<Self, PaginationError> {
        Self::new(Self::OFFSET_DEFAULT, limit)
    }

    pub fn set_offset(&mut self, offset: u64) -> Result<(), RangeBoundsError> {
        Self::is_valid_range(offset, self.limit.get())?;
        self.offset = offset;
        Ok(())
    }

    pub fn set_limit(&mut self, limit: u64) -> Result<(), PaginationError> {
        Self::is_valid_range(self.offset, limit)?;
        self.limit.set(limit)?;
        Ok(())
    }

    pub fn next_page(&mut self, next: u64) -> Result<(), PaginationError> {
        match self.set_offset(next) {
            Err(RangeBoundsError { available: Some(limit), .. }) => {
                Self::is_valid_range(next, limit)?;
                self.limit.set(limit)?;
                self.offset = next;
                Ok(())
            }
            result => result.map_err(PaginationError::Range),
        }
    }

    // The sum of offset and limit must be < 10_000.
    // API will return a `error: "offset + limit must be < 10000"` if not so.
    #[inline]
    fn is_valid_range(offset: u64, limit: u64) -> Result<(), RangeBoundsError> {
        match offset.checked_add(limit) {
            Some(sum) if sum <= Self::SUM_MAX => Ok(()),
            _ => {
                let available = Self::SUM_MAX.checked_nonzero_sub(offset);
                Err(RangeBoundsError { offset, limit, available })
            }
        }
    }

    #[inline]
    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    #[inline]
    pub fn get_limit(&self) -> u64 {
        self.limit.get()
    }
}

impl Default for Page {
    fn default() -> Self {
        Self { offset: Self::OFFSET_DEFAULT, limit: Limit::default() }
    }
}

pub(in crate::v1) trait Paged: AsRef<Page> + AsMut<Page> {
    fn set_limit(&mut self, limit: u64) -> Result<(), PaginationError> {
        self.as_mut().set_limit(limit)
    }

    fn next_page(&mut self, next: u64) -> Result<(), PaginationError> {
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
    Limit(#[from] LimitBoundsError),
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
#[error("`offset` and `limit` sum must be lower or equal to {}, but provided: offset={offset} and limit={limit}", Page::SUM_MAX)]
pub struct RangeBoundsError {
    pub offset: u64,
    pub limit: u64,
    pub available: Option<u64>,
}

// Compile time assertions.
extern crate static_assertions as sa;
// Must not compile if Page::SUM_MAX is 0.
sa::const_assert!(Page::SUM_MAX > 0);
// Must not compile if Page::SUM_MAX is greater or equal to 10_000;
sa::const_assert!(Page::SUM_MAX < 10_000);

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_test::{assert_ser_tokens, Token};

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
                Err(RangeBoundsError { offset, limit, available }) => {
                    assert_eq!(offset, sum.0);
                    assert_eq!(limit, sum.1);
                    assert_eq!(available, Page::SUM_MAX.checked_nonzero_sub(sum.0));
                }
                Ok(_) => panic!(
                    "must fail when the sum of `offset` and `limit` is greater than Page::SUM_MAX"
                ),
            }
        }
    }

    #[test]
    fn is_valid_range_does_not_overflow() {
        let sum = (u64::MAX, 1);

        match Page::is_valid_range(sum.0, sum.1) {
            Err(RangeBoundsError { offset, limit, available }) => {
                assert_eq!(offset, sum.0);
                assert_eq!(limit, sum.1);
                assert_eq!(available, None, "`available` must be None, must not overflow");
            }
            // If sum of u64::MAX and any other u64 value
            // is lower or equal to Page::SUM_MAX, than
            // Page::SUM_MAX itself is equal to u64::MAX.
            Ok(_) => assert_eq!(Page::SUM_MAX, u64::MAX),
        }
    }

    #[test]
    fn next_page_fails_if_no_availabe_new_limit() {
        let mut nexts = vec![u64::MAX];

        if Page::SUM_MAX < u64::MAX {
            let greater = Page::SUM_MAX.checked_add(1).unwrap();
            nexts.push(greater);
        }

        for next in nexts {
            let mut page = Page::default();
            match page.next_page(next) {
                Err(PaginationError::Range(RangeBoundsError { offset, limit, available })) => {
                    assert_eq!(offset, next);
                    assert_eq!(limit, page.get_limit());
                    assert_eq!(
                        available, None,
                        "`available` must be None, reached API upper results limit"
                    );
                }
                Ok(_) => {
                    panic!("must fail when API upper results limit is reached")
                }
                Err(_) => unreachable!(),
            }
        }
    }

    #[test]
    fn page_serialization() {
        let page = Page::default();

        // Expecting a flat layout on serialization.
        assert_ser_tokens(
            &page,
            &[
                Token::Map { len: None },
                Token::Str("offset"),
                Token::U64(page.get_offset()),
                Token::Str("limit"),
                Token::U64(page.get_limit()),
                Token::MapEnd,
            ],
        );
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
