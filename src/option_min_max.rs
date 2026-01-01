use std::{cmp, cmp::Ord};

#[derive(Copy, Clone, Debug)]
pub struct OptionMinMax<T>(pub Option<T>);

impl<T> OptionMinMax<T>
where
    T: Ord + Copy,
{
    pub fn new(val: Option<T>) -> Self {
        Self(val)
    }

    pub fn new_concrete(val: T) -> Self {
        Self(Some(val))
    }

    pub fn min(&self, other: T) -> Self {
        Self(Some(match self.0 {
            None => other,
            Some(min) => cmp::min(other, min),
        }))
    }

    pub fn max(&self, other: T) -> Self {
        Self(Some(match self.0 {
            None => other,
            Some(max) => cmp::max(other, max),
        }))
    }

    pub fn min_self(&self, other: Self) -> Self {
        let Some(left) = self.0 else {
            return other;
        };

        let Some(right) = other.0 else {
            return *self;
        };

        Self(Some(cmp::min(left, right)))
    }

    pub fn max_self(&self, other: Self) -> Self {
        let Some(left) = self.0 else {
            return other;
        };

        let Some(right) = other.0 else {
            return *self;
        };

        Self(Some(cmp::max(left, right)))
    }

    pub fn min_option(&self, other: Option<T>) -> Self {
        let Some(left) = self.0 else {
            return Self(other);
        };

        let Some(right) = other else {
            return *self;
        };

        Self(Some(cmp::min(left, right)))
    }

    pub fn max_option(&self, other: Option<T>) -> Self {
        let Some(left) = self.0 else {
            return Self(other);
        };

        let Some(right) = other else {
            return *self;
        };

        Self(Some(cmp::max(left, right)))
    }
}
