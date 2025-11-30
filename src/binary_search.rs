use std::ops::{Add, Div};

/// For example usage, see <https://github.com/glennhartmann/aoc19> day_14.rs.
pub trait BinarySearchable<I, V>
where
    I: Copy + PartialOrd + Add<Output = I> + Div<usize, Output = I>,
{
    /// Gets the value at the given index. Called at with the midpoint at the beginning of each
    /// loop. Can mutate internal state.
    fn get_val(&mut self, index: &I) -> V;

    /// Called immediately after retrieving the `val` at the midpoint using `get_val()`. Returns
    /// whether the result has been found. Returning `true` will exit the loop, returning `val` and its
    /// index. Can mutate internal state.
    fn found_result(&mut self, val: &V, lower_bound: &I, mid_point: &I, upper_bound: &I) -> bool;

    /// Called immediately after `found_result()`, provided that `found_result()` returned false. The
    /// purpose is to mutate `lower_bound` and `upper_bound`. The return value indicates whether to
    /// abort the search in failure. Returning `true` will result in the loop exiting and returning
    /// `None`. Can mutate internal state.
    fn set_next_bounds(
        &mut self,
        val: &V,
        lower_bound: &mut I,
        mid_point: &I,
        upper_bound: &mut I,
    ) -> bool;

    fn binary_search(&mut self, mut lower_bound: I, mut upper_bound: I) -> Option<(I, V)> {
        while upper_bound > lower_bound {
            let mid_point = (lower_bound + upper_bound) / 2;
            let val = self.get_val(&mid_point);
            if self.found_result(&val, &lower_bound, &mid_point, &upper_bound) {
                return Some((mid_point, val));
            }
            if self.set_next_bounds(&val, &mut lower_bound, &mid_point, &mut upper_bound) {
                break;
            }
        }

        None
    }
}

pub struct BinarySearchableVec<V> {
    v: Vec<V>,
    t: V,
}

impl<V> BinarySearchableVec<V> {
    pub fn new(value: Vec<V>, target: V) -> Self {
        Self {
            v: value,
            t: target,
        }
    }
}

impl<V> BinarySearchable<usize, V> for BinarySearchableVec<V>
where
    V: Copy + PartialOrd + PartialEq,
{
    fn get_val(&mut self, index: &usize) -> V {
        self.v[*index]
    }

    fn found_result(
        &mut self,
        val: &V,
        _lower_bound: &usize,
        _index: &usize,
        _upper_bound: &usize,
    ) -> bool {
        *val == self.t
    }

    fn set_next_bounds(
        &mut self,
        val: &V,
        lower_bound: &mut usize,
        index: &usize,
        upper_bound: &mut usize,
    ) -> bool {
        if *lower_bound == *index {
            return true;
        }

        if *val > self.t {
            *upper_bound = *index;
        } else if *val < self.t {
            *lower_bound = *index;
        } else {
            panic!("should be impossible");
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let v = vec![1, 2, 3, 4, 5];
        let l = v.len();
        let mut bs = BinarySearchableVec::new(v, 3);
        assert_eq!(bs.binary_search(0, l), Some((2, 3)));
    }

    #[test]
    fn test_upper() {
        let v = vec![1, 2, 3, 4, 5];
        let l = v.len();
        let mut bs = BinarySearchableVec::new(v, 5);
        assert_eq!(bs.binary_search(0, l), Some((4, 5)));
    }

    #[test]
    fn test_lower() {
        let v = vec![1, 2, 3, 4, 5];
        let l = v.len();
        let mut bs = BinarySearchableVec::new(v, 1);
        assert_eq!(bs.binary_search(0, l), Some((0, 1)));
    }

    #[test]
    fn test_even_mid_upper() {
        let v = vec![1, 2, 3, 4];
        let l = v.len();
        let mut bs = BinarySearchableVec::new(v, 3);
        assert_eq!(bs.binary_search(0, l), Some((2, 3)));
    }

    #[test]
    fn test_even_mid_lower() {
        let v = vec![1, 2, 3, 4];
        let l = v.len();
        let mut bs = BinarySearchableVec::new(v, 2);
        assert_eq!(bs.binary_search(0, l), Some((1, 2)));
    }
}
