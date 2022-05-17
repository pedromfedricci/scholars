#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub(super) struct Limit {
    limit: u64,
}

impl Limit {
    // The max number of results that a single page can return is 100.
    // API will return a error if the `limit` param is greater than 100.
    const MAX: u64 = 100;

    // The minimum value that the `limit` parameter can have is 1.
    // API will return a `error: "Unacceptable query params: [limit=0]"` if limit is equal to 0.
    const MIN: u64 = 1;

    // The default `limit` for returned results on a single page is 100.
    const DEFAULT: u64 = Self::MAX;

    // API will return a `error: "'limit' must be <= 100"` if limit is over 100.
    // API will return a `error: "Unacceptable query params: [limit=0]"` if limit is equal to 0.
    #[inline]
    const fn is_valid_limit(limit: u64) -> Result<(), LimitBoundsError> {
        if limit >= Self::MIN && limit <= Self::MAX {
            Ok(())
        } else {
            Err(LimitBoundsError { limit })
        }
    }

    #[inline]
    pub(super) fn new(limit: u64) -> Result<Limit, LimitBoundsError> {
        Self::is_valid_limit(limit)?;
        Ok(Limit { limit })
    }

    #[inline]
    pub(super) fn set(&mut self, limit: u64) -> Result<(), LimitBoundsError> {
        Self::is_valid_limit(limit)?;
        Ok(())
    }

    #[inline]
    pub(super) fn get(&self) -> u64 {
        self.limit
    }
}

impl Default for Limit {
    fn default() -> Limit {
        Limit { limit: Limit::DEFAULT }
    }
}

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, PartialEq)]
#[error(
    "`limit` must be greater than {} and lower or equal to {}, but provided: {limit}",
    Limit::MIN,
    Limit::MAX
)]
pub struct LimitBoundsError {
    pub limit: u64,
}

// Compile time assertions.
extern crate static_assertions as sa;
// Must not compile if Limit::MIN is equal to 0.
sa::const_assert!(Limit::MIN > 0);
// Must not compile if Limit::MAX is greater than 100.
sa::const_assert!(Limit::MAX <= 100);
// Must not compile if Limit::MAX is lower than Limit::MIN.
sa::const_assert!(Limit::MAX >= Limit::MIN);
// Must not compile if Limit::DEFAULT is out of MIN/MAX range.
sa::const_assert!(Limit::MIN <= Limit::DEFAULT && Limit::DEFAULT <= Limit::MAX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_limit_succeeds_greater_or_equal_to_min() {
        let mut limits = vec![Limit::MIN];

        if Limit::MIN < u64::MAX {
            limits.push(Limit::MIN.checked_add(1).unwrap());
        }

        for limit in limits {
            match Limit::is_valid_limit(limit) {
                Ok(_) => (),
                Err(_) => {
                    panic!("must succeed if provided `limit` is greater or equal to Limit::MIN")
                }
            }
        }
    }

    #[test]
    fn is_valid_limit_fails_lower_than_min() {
        if Limit::MIN > 0 {
            let lower = Limit::MIN.checked_sub(1).unwrap();

            match Limit::is_valid_limit(lower) {
                Err(LimitBoundsError { limit }) => assert_eq!(limit, lower),
                Ok(_) => {
                    panic!("must fail if provided `limit` is lower than Limit::MIN")
                }
            }
        }
    }

    #[test]
    fn is_valid_limit_succeeds_lower_or_equal_to_max() {
        let mut limits = vec![Limit::MAX];

        if Limit::MAX > 0 {
            limits.push(Limit::MAX.checked_sub(1).unwrap());
        }

        for limit in limits {
            match Limit::is_valid_limit(limit) {
                Ok(_) => (),
                Err(_) => {
                    panic!("must succeed if provided `limit` is lower or equal to Limit::MAX")
                }
            }
        }
    }

    #[test]
    fn is_valid_limit_fails_greater_than_max() {
        if Limit::MAX < u64::MAX {
            let greater = Limit::MAX.checked_add(1).unwrap();

            match Limit::is_valid_limit(greater) {
                Err(LimitBoundsError { limit }) => assert_eq!(limit, greater),
                Ok(_) => {
                    panic!("must fail if provided `limit` is greater than Limit::MAX")
                }
            }
        }
    }
}
