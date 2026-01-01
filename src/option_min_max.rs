use std::{cmp, cmp::Ord};

/// Simple helper type that makes it easier to get mins and maxes when some values may be None.
/// Typical usage looks something like:
///
/// ```
/// let mut m = aoclib_rs::option_min_max::OptionMinMax::NONE;
/// for i in [1, 5, 8, 3, 2, 12, 7] {
///     m = m.max(i);
/// }
/// assert_eq!(m.unwrap(), 12);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct OptionMinMax<T: Ord + Copy>(Option<T>);

impl<T: Ord + Copy> OptionMinMax<T> {
    pub const NONE: Self = Self::new(None);

    pub const fn new(val: Option<T>) -> Self {
        Self(val)
    }

    pub fn new_concrete(val: T) -> Self {
        Self(Some(val))
    }

    /// Returns a new `OptionMinMax` containing the minimum value, or `other` if `self` contains
    /// `None`.
    ///
    /// ```
    /// use aoclib_rs::option_min_max::OptionMinMax;
    /// assert_eq!(OptionMinMax::NONE.min(3).unwrap(), 3);
    /// assert_eq!(OptionMinMax::new_concrete(2).min(3).unwrap(), 2);
    /// ```
    pub fn min(&self, other: T) -> Self {
        Self(Some(match self.0 {
            None => other,
            Some(min) => cmp::min(other, min),
        }))
    }

    /// Returns a new `OptionMinMax` containing the maximum value, or `other` if `self` contains
    /// `None`.
    ///
    /// ```
    /// use aoclib_rs::option_min_max::OptionMinMax;
    /// assert_eq!(OptionMinMax::NONE.max(3).unwrap(), 3);
    /// assert_eq!(OptionMinMax::new_concrete(2).max(3).unwrap(), 3);
    /// ```
    pub fn max(&self, other: T) -> Self {
        Self(Some(match self.0 {
            None => other,
            Some(max) => cmp::max(other, max),
        }))
    }

    /// Returns a new `OptionMinMax` containing the non-`None` value, if one exists. If both values
    /// are non-`None`, returns the minimum.
    ///
    /// ```
    /// use aoclib_rs::option_min_max::OptionMinMax;
    /// assert_eq!(OptionMinMax::<i32>::NONE.min_self(OptionMinMax::NONE).get(), None);
    /// assert_eq!(OptionMinMax::NONE.min_self(OptionMinMax::new_concrete(3)).unwrap(), 3);
    /// assert_eq!(OptionMinMax::new_concrete(4).min_self(OptionMinMax::NONE).unwrap(), 4);
    /// assert_eq!(OptionMinMax::new_concrete(4).min_self(OptionMinMax::new_concrete(2)).unwrap(), 2);
    /// ```
    pub fn min_self(&self, other: Self) -> Self {
        let Some(left) = self.0 else {
            return other;
        };

        let Some(right) = other.0 else {
            return *self;
        };

        Self(Some(cmp::min(left, right)))
    }

    /// Returns a new `OptionMinMax` containing the non-`None` value, if one exists. If both values
    /// are non-`None`, returns the maximum.
    ///
    /// ```
    /// use aoclib_rs::option_min_max::OptionMinMax;
    /// assert_eq!(OptionMinMax::<i32>::NONE.max_self(OptionMinMax::NONE).get(), None);
    /// assert_eq!(OptionMinMax::NONE.max_self(OptionMinMax::new_concrete(3)).unwrap(), 3);
    /// assert_eq!(OptionMinMax::new_concrete(4).max_self(OptionMinMax::NONE).unwrap(), 4);
    /// assert_eq!(OptionMinMax::new_concrete(4).max_self(OptionMinMax::new_concrete(2)).unwrap(), 4);
    /// ```
    pub fn max_self(&self, other: Self) -> Self {
        let Some(left) = self.0 else {
            return other;
        };

        let Some(right) = other.0 else {
            return *self;
        };

        Self(Some(cmp::max(left, right)))
    }

    /// Returns a new `OptionMinMax` containing the non-`None` value, if one exists. If both values
    /// are non-`None`, returns the minimum.
    ///
    /// ```
    /// use aoclib_rs::option_min_max::OptionMinMax;
    /// assert_eq!(OptionMinMax::<i32>::NONE.min_option(None).get(), None);
    /// assert_eq!(OptionMinMax::NONE.min_option(Some(3)).unwrap(), 3);
    /// assert_eq!(OptionMinMax::new_concrete(4).min_option(None).unwrap(), 4);
    /// assert_eq!(OptionMinMax::new_concrete(4).min_option(Some(2)).unwrap(), 2);
    /// ```
    pub fn min_option(&self, other: Option<T>) -> Self {
        let Some(left) = self.0 else {
            return Self(other);
        };

        let Some(right) = other else {
            return *self;
        };

        Self(Some(cmp::min(left, right)))
    }

    /// Returns a new `OptionMinMax` containing the non-`None` value, if one exists. If both values
    /// are non-`None`, returns the minimum.
    ///
    /// ```
    /// use aoclib_rs::option_min_max::OptionMinMax;
    /// assert_eq!(OptionMinMax::<i32>::NONE.max_option(None).get(), None);
    /// assert_eq!(OptionMinMax::NONE.max_option(Some(3)).unwrap(), 3);
    /// assert_eq!(OptionMinMax::new_concrete(4).max_option(None).unwrap(), 4);
    /// assert_eq!(OptionMinMax::new_concrete(4).max_option(Some(2)).unwrap(), 4);
    /// ```
    pub fn max_option(&self, other: Option<T>) -> Self {
        let Some(left) = self.0 else {
            return Self(other);
        };

        let Some(right) = other else {
            return *self;
        };

        Self(Some(cmp::max(left, right)))
    }

    pub fn get(&self) -> Option<T> {
        self.0
    }

    /// can panic
    pub fn unwrap(&self) -> T {
        self.0.unwrap()
    }
}

impl<T: Ord + Copy> Default for OptionMinMax<T> {
    fn default() -> Self {
        Self::NONE
    }
}
