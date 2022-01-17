/// Pagination options for
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pages {
    /// Return all results.
    All,
    /// Limit to a number of results.
    Limit(u64),
}

impl Default for Pages {
    fn default() -> Self {
        Pages::All
    }
}

/// API will return at max the first 10_000 results from any query.
/// Even if the total number of results is greater than 10_000,
/// you cannot query any result beyond that point.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    offset: u64,
    limit: u64,
}

impl Page {
    // API will return up to the first 10000 results from the search list.
    // You can't query any results beyond that point, API will return a error.
    const RANGE_LIMIT: u64 = 10_000;
    // The max number of results that a page can return is 100.
    // API will return a error if the limit param is greater than 100 or equal to 0.
    const LIMIT_MAX: u64 = 100;

    // The default limit for returned results on a single page is 10.
    const LIMIT_DEFAULT: u64 = 10;
    // Offset's default starting position is 0.
    const OFFSET_DEFAULT: u64 = 0;

    pub fn new(offset: u64, limit: u64) -> Result<Self, PaginationError> {
        Self::check_boundaries(offset, limit)?;
        Ok(Page { offset, limit })
    }

    pub fn with_offset(offset: u64) -> Result<Self, RangeBoundsError> {
        Self::is_valid_range(offset, Self::LIMIT_DEFAULT)?;
        Ok(Page { offset, limit: Self::LIMIT_DEFAULT })
    }

    pub fn with_limit(limit: u64) -> Result<Self, PaginationError> {
        Self::new(Self::OFFSET_DEFAULT, limit)
    }

    pub fn set_offset(&mut self, offset: u64) -> Result<(), RangeBoundsError> {
        Self::is_valid_range(offset, self.limit)?;
        self.offset = offset;
        Ok(())
    }

    pub fn set_limit(&mut self, limit: u64) -> Result<(), PaginationError> {
        Self::check_boundaries(self.offset, limit)?;
        self.limit = limit;
        Ok(())
    }

    pub fn next_page(&mut self, next: u64) -> Result<(), RangeBoundsError> {
        match Self::is_valid_range(next, self.limit) {
            Ok(_) => {
                self.offset = next;
                Ok(())
            }
            Err(err @ RangeBoundsError { availiable, .. }) => {
                if availiable > 0 {
                    self.offset = next;
                    self.limit = availiable;
                    Ok(())
                } else {
                    Err(err)
                }
            }
        }
    }

    // The sum of offset and limit must be < 10_000.
    // API will return a `error: "offset + limit must be < 10000"` if not so.
    #[inline]
    fn is_valid_range(offset: u64, limit: u64) -> Result<(), RangeBoundsError> {
        if offset.saturating_add(limit) < Self::RANGE_LIMIT {
            Ok(())
        } else {
            let availiable = Self::RANGE_LIMIT.saturating_sub(offset).saturating_sub(1);
            Err(RangeBoundsError { offset, limit, availiable })
        }
    }

    // API will return a `error: "'limit' must be <= 100"` if limit is over 100.
    // API will return a `error: "Unacceptable query params: [limit=0]"` if limit is equal to 0.
    #[inline]
    fn is_valid_limit(limit: u64) -> Result<(), LimitBoundsError> {
        if limit <= Self::LIMIT_MAX && limit > 0 {
            Ok(())
        } else {
            Err(LimitBoundsError { limit })
        }
    }

    // Check all page construction boundaries.
    #[inline]
    fn check_boundaries(offset: u64, limit: u64) -> Result<(), PaginationError> {
        Self::is_valid_limit(limit)?;
        Self::is_valid_range(offset, limit)?;
        Ok(())
    }

    #[inline]
    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    #[inline]
    pub fn get_limit(&self) -> u64 {
        self.limit
    }
}

impl Default for Page {
    fn default() -> Self {
        Self { offset: Self::OFFSET_DEFAULT, limit: Self::LIMIT_DEFAULT }
    }
}

pub trait Paged {
    fn get_page(&self) -> &Page;

    fn get_page_mut(&mut self) -> &mut Page;

    fn set_limit(&mut self, limit: u64) -> Result<(), PaginationError> {
        self.get_page_mut().set_limit(limit)
    }

    fn next_page(&mut self, next: u64) -> Result<(), RangeBoundsError> {
        self.get_page_mut().next_page(next)
    }

    fn get_offset(&self) -> u64 {
        self.get_page().get_offset()
    }

    fn get_limit(&self) -> u64 {
        self.get_page().get_limit()
    }
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
pub enum PaginationError {
    #[error(transparent)]
    Range(#[from] RangeBoundsError),
    #[error(transparent)]
    Limit(#[from] LimitBoundsError),
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
#[error(
    "\
offset and limit sum must be lower than 10_000, but provided: offset={offset} and limit={limit}"
)]
pub struct RangeBoundsError {
    offset: u64,
    limit: u64,
    availiable: u64,
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
#[error(
    "\
limit must be greater than 0 and lower or equal to 100, but provided: limit={limit}"
)]
pub struct LimitBoundsError {
    limit: u64,
}
