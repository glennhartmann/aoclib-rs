use std::{cmp, cmp::Ord};

#[derive(Copy, Clone, Debug)]
pub struct OptionMinMax<T>(pub Option<T>);

impl<T> OptionMinMax<T>
where
    T: Ord + Copy,
{
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
}
