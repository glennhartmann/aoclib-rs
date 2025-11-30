use std::iter;

use crate::inc_selector;

pub fn fwd_rev_incl_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    let mut fwd = start..=end;
    let mut rev = (end..=start).rev();

    iter::from_fn(move || if start > end { rev.next() } else { fwd.next() })
}

pub fn pairwise_iter<T>(v: &[T]) -> impl Iterator<Item = (&T, &T)> {
    let mut it = pairwise_iter_i(v);
    iter::from_fn(move || it.next().map(|(i, j)| (&v[i], &v[j])))
}

pub fn pairwise_iter_copy<T: Copy>(v: &[T]) -> impl Iterator<Item = (T, T)> {
    let mut it = pairwise_iter_i(v);
    iter::from_fn(move || it.next().map(|(i, j)| (v[i], v[j])))
}

pub fn pairwise_iter_i<T>(v: &[T]) -> impl Iterator<Item = (usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    iter::from_fn(move || {
        if v.len() < 2 {
            return None;
        }

        j += 1;
        if j >= v.len() {
            i += 1;
            j = i + 1;
        }

        if j >= v.len() {
            return None;
        }

        Some((i, j))
    })
}

pub fn selector_iter(len: usize) -> impl Iterator<Item = Vec<bool>> {
    let mut v: Vec<bool> = vec![false; len];
    let mut first = true;
    iter::from_fn(move || {
        if first {
            first = false;
            return Some(v.clone());
        }
        if inc_selector(&mut v) {
            None
        } else {
            Some(v.clone())
        }
    })
}

// TODO: does this make more sense as sets instead of Vecs?
// TODO: do we need a copy version of this?
pub fn selection_iter<T>(v: &[T]) -> impl Iterator<Item = Vec<&T>> {
    let mut selector: Vec<bool> = vec![false; v.len()];
    let mut first = true;
    iter::from_fn(move || {
        if first {
            first = false;
            return Some(Vec::new());
        }
        if inc_selector(&mut selector) {
            None
        } else {
            Some(
                v.iter()
                    .enumerate()
                    .filter_map(|(i, e)| if selector[i] { Some(e) } else { None })
                    .collect(),
            )
        }
    })
}

// Non-recursive variant of Heap's Algorithm https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub fn permutation_iter_copy<T: Copy>(mut a: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    let n = a.len();
    let mut c: Vec<usize> = vec![0; n];
    let mut first = true;
    let mut i = 1;
    iter::from_fn(move || {
        if first {
            first = false;
            return Some(a.clone());
        }

        while i < n {
            if c[i] < i {
                if i % 2 == 0 {
                    (a[0], a[i]) = (a[i], a[0]);
                } else {
                    (a[c[i]], a[i]) = (a[i], a[c[i]]);
                }
                c[i] += 1;
                i = 1;
                return Some(a.clone());
            } else {
                c[i] = 0;
                i += 1;
            }
        }

        None
    })
}
