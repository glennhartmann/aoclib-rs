use std::ops::{Add, Div};

pub trait BinarySearchable<I, V>
where
    I: Copy + PartialOrd + Add<Output = I> + Div<usize, Output = I>,
{
    fn get_val(&self, index: &I) -> V;
    fn found_result(&self, index: &I, val: &V) -> bool;
    fn set_next_bounds(&self, val: &V, lower_bound: &mut I, index: &I, upper_bound: &mut I)
    -> bool;

    fn binary_search(&self, mut lower_bound: I, mut upper_bound: I) -> Option<(I, V)> {
        while upper_bound > lower_bound {
            let mid_point = (lower_bound + upper_bound) / 2;
            let val = self.get_val(&mid_point);
            if self.found_result(&mid_point, &val) {
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
    fn get_val(&self, index: &usize) -> V {
        self.v[*index]
    }

    fn found_result(&self, _: &usize, val: &V) -> bool {
        *val == self.t
    }

    fn set_next_bounds(
        &self,
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
        let bs = BinarySearchableVec::new(v, 3);
        assert_eq!(bs.binary_search(0, l), Some((2, 3)));
    }

    #[test]
    fn test_upper() {
        let v = vec![1, 2, 3, 4, 5];
        let l = v.len();
        let bs = BinarySearchableVec::new(v, 5);
        assert_eq!(bs.binary_search(0, l), Some((4, 5)));
    }

    #[test]
    fn test_lower() {
        let v = vec![1, 2, 3, 4, 5];
        let l = v.len();
        let bs = BinarySearchableVec::new(v, 1);
        assert_eq!(bs.binary_search(0, l), Some((0, 1)));
    }
}
