#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub(super) struct ConstrainedU64<const MIN: u64, const MAX: u64, const DEF: u64 = MIN>(u64);

impl<const MIN: u64, const MAX: u64, const DEF: u64> ConstrainedU64<MIN, MAX, DEF> {
    /// The maximum **inclusive** value that this container can hold.
    /// Must satify MAX >= MIN.
    const MAX: u64 = MAX;

    /// The minimum **inclusive** value that this container can hold.
    /// Must satisfy MIN <= MAX.
    const MIN: u64 = MIN;

    /// The default value that a container instance will hold.
    /// Must satifu MIN <= DEF <= MAX.
    const DEF: u64 = DEF;

    #[inline]
    const fn is_valid(value: u64) -> Result<(), RangeBoundError<MIN, MAX>> {
        if value > Self::MAX {
            Err(RangeBoundError::Max(MaxError))
        } else if value < Self::MIN {
            Err(RangeBoundError::Min(MinError))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub(super) fn new(value: u64) -> Result<Self, RangeBoundError<MIN, MAX>> {
        Self::is_valid(value)?;
        Ok(Self(value))
    }

    #[inline]
    pub(super) fn set(&mut self, value: u64) -> Result<(), RangeBoundError<MIN, MAX>> {
        Self::is_valid(value)?;
        self.0 = value;
        Ok(())
    }

    #[inline]
    pub(super) fn available(value: u64) -> Option<Self> {
        match Self::is_valid(value) {
            Ok(_) if value == Self::MIN => None,
            Ok(_) => Some(Self(value)),
            Err(RangeBoundError::Min(_)) => None,
            Err(RangeBoundError::Max(_)) => Some(Self(Self::MAX)),
        }
    }

    #[inline]
    pub(super) fn get(&self) -> u64 {
        self.0
    }
}

impl<const DEF: u64> ConstrainedU64<{ u64::MIN }, { u64::MAX }, DEF> {
    #[inline]
    pub(super) fn new_infallible(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub(super) fn set_infallible(&mut self, value: u64) {
        self.0 = value;
    }
}

impl<const MIN: u64, const MAX: u64, const DEF: u64> Default for ConstrainedU64<MIN, MAX, DEF> {
    fn default() -> Self {
        Self(Self::DEF)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("must be greater or equal to {}", MIN)]
pub(super) struct MinError<const MIN: u64>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("must be lower or equal to {}", MAX)]
pub(super) struct MaxError<const MAX: u64>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub(super) enum RangeBoundError<const MIN: u64, const MAX: u64> {
    #[error(transparent)]
    Min(#[from] MinError<MIN>),
    #[error(transparent)]
    Max(#[from] MaxError<MAX>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_returns_def_value() {
        const DEF: u64 = 2;
        type DefaultMin = ConstrainedU64<1, 3, DEF>;
        assert_eq!(DefaultMin::default().get(), DEF);
        assert_eq!(DefaultMin::DEF, DEF);
    }

    #[test]
    fn is_valid_succeeds_if_greater_or_equal_to_min() {
        type GreatOrEqualMin = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [GreatOrEqualMin::MIN, GreatOrEqualMin::MIN + 1];

        for input in inputs {
            match GreatOrEqualMin::is_valid(input) {
                Ok(_) => (),
                Err(_) => {
                    panic!("must succeed if greater or equal to ConstrainedU64::MIN")
                }
            }
        }
    }

    #[test]
    fn is_valid_fails_if_lower_than_min() {
        type LowerThanMin = ConstrainedU64<1, { u64::MAX }>;

        let input = LowerThanMin::MIN - 1;

        match LowerThanMin::is_valid(input) {
            Err(err @ RangeBoundError::Min(MinError)) => {
                // assert correct Display impl.
                assert_eq!(
                    format!("must be greater or equal to {}", LowerThanMin::MIN),
                    err.to_string()
                );
            }
            _ => panic!("must fail if lower than ConstrainedU64::MIN"),
        }
    }

    #[test]
    fn is_valid_succeeds_if_lower_or_equal_to_max() {
        type LowerOrEqualMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [LowerOrEqualMax::MAX, LowerOrEqualMax::MAX - 1];

        for input in inputs {
            match LowerOrEqualMax::is_valid(input) {
                Ok(_) => (),
                Err(_) => {
                    panic!("must succeed if lower or equal to ConstrainedU64::MAX")
                }
            }
        }
    }

    #[test]
    fn is_valid_fails_if_greater_than_max() {
        type GreaterThanMax = ConstrainedU64<0, { u64::MAX - 1 }>;

        let input = GreaterThanMax::MAX + 1;

        match GreaterThanMax::is_valid(input) {
            Err(err @ RangeBoundError::Max(MaxError)) => {
                assert_eq!(
                    // assert correct Diplay impl.
                    format!("must be lower or equal to {}", GreaterThanMax::MAX),
                    err.to_string()
                );
            }
            _ => panic!("must fail if greater than ConstrainedU64::MAX"),
        }
    }

    #[test]
    fn new_succeeds_with_valid_input() {
        type GreaterEqMinLowerEqMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [GreaterEqMinLowerEqMax::MIN, GreaterEqMinLowerEqMax::MAX];

        for input in inputs {
            let constrained = GreaterEqMinLowerEqMax::new(input).expect("valid input");
            assert_eq!(input, constrained.get())
        }
    }

    #[test]
    fn new_fails_with_invalid_input() {
        type LowerMinGreaterMax = ConstrainedU64<1, { u64::MAX - 1 }>;

        let inputs = [LowerMinGreaterMax::MIN - 1, LowerMinGreaterMax::MAX + 1];

        for input in inputs {
            match LowerMinGreaterMax::new(input) {
                Err(_) => (),
                Ok(_) => panic!("new must fail with out of range input"),
            }
        }
    }

    #[test]
    fn set_succeeds_with_valid_input() {
        type GreaterEqMinLowerEqMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [GreaterEqMinLowerEqMax::MIN, GreaterEqMinLowerEqMax::MAX];
        let mut test = GreaterEqMinLowerEqMax::default();

        for input in inputs {
            match test.set(input) {
                Ok(_) => {
                    assert_eq!(input, test.get(), "must have the same value as valid input")
                }
                Err(_) => panic!("set must succeed with valid input"),
            }
        }
    }

    #[test]
    fn set_fails_with_invalid_input() {
        type LowerMinGreaterMax = ConstrainedU64<1, { u64::MAX - 1 }>;

        let inputs = [LowerMinGreaterMax::MIN - 1, LowerMinGreaterMax::MAX + 1];
        let mut test = LowerMinGreaterMax::default();

        for input in inputs {
            match test.set(input) {
                Err(_) => (),
                Ok(_) => panic!("set must fail with invalid input"),
            }
        }
    }

    #[test]
    fn get_returns_wrapped_value() {
        type GreaterEqMinLowerEqMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [GreaterEqMinLowerEqMax::MIN, GreaterEqMinLowerEqMax::MAX];

        for input in inputs {
            let mut constrained = GreaterEqMinLowerEqMax::default();
            constrained.set(input).expect("valid input");
            assert_eq!(input, constrained.get());
        }
    }

    #[test]
    fn available_succeeds_saturating_add_on_max() {
        type GreaterThanMax = ConstrainedU64<0, { u64::MAX - 1 }>;

        let input = u64::MAX;

        match GreaterThanMax::available(input) {
            Some(value) => {
                assert_eq!(
                    value.get(),
                    GreaterThanMax::MAX,
                    "must be equal to Constrained::MAX value"
                )
            }
            None => panic!("must succeed and return a Some"),
        }
    }

    #[test]
    fn available_succeeds_greater_than_min_lower_or_equal_to_max() {
        type GreaterMinLowerEqMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [
            GreaterMinLowerEqMax::MAX,
            GreaterMinLowerEqMax::MAX - 1,
            GreaterMinLowerEqMax::MIN + 1,
        ];

        for input in inputs {
            match GreaterMinLowerEqMax::available(input) {
                Some(value) => {
                    assert_eq!(value.get(), input, "must have same value as valid input")
                }
                None => panic!("must succeed and return a Some"),
            }
        }
    }

    #[test]
    fn available_fails_lower_or_equal_to_min() {
        type LowerOrEqualMin = ConstrainedU64<1, { u64::MAX }>;

        let inputs = [LowerOrEqualMin::MIN, LowerOrEqualMin::MIN - 1];

        for input in inputs {
            match LowerOrEqualMin::available(input) {
                None => (),
                Some(_) => panic!("must fail and return a None"),
            }
        }
    }

    #[test]
    fn new_infallible_succeeds_valid_input() {
        type GreaterEqMinLowerEqMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [GreaterEqMinLowerEqMax::MIN, GreaterEqMinLowerEqMax::MAX];

        for input in inputs {
            let constrained = GreaterEqMinLowerEqMax::new_infallible(input);
            assert_eq!(input, constrained.get())
        }
    }

    #[test]
    fn set_infallible_succeeds_valid_input() {
        type GreaterEqMinLowerEqMax = ConstrainedU64<0, { u64::MAX }>;

        let inputs = [GreaterEqMinLowerEqMax::MIN, GreaterEqMinLowerEqMax::MAX];
        let mut constrained = GreaterEqMinLowerEqMax::default();

        for input in inputs {
            constrained.set_infallible(input);
            assert_eq!(input, constrained.get())
        }
    }
}
