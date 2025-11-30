use std::iter;

use crate::inc_selector;

/// Forward/reverse inclusive range. If `start <= end`, returns a forward iterator from `start`
/// to `end`, inclusive. Else, returns a reverse iterator from `end` to `start`, inclusive.
///
/// ```
/// use aoclib_rs::iter::fwd_rev_incl_range;
/// assert_eq!(fwd_rev_incl_range(1, 3).collect::<Vec<_>>(), [1, 2, 3]);
/// assert_eq!(fwd_rev_incl_range(3, 1).collect::<Vec<_>>(), [3, 2, 1]);
/// ```
pub fn fwd_rev_incl_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    let mut fwd = start..=end;
    let mut rev = (end..=start).rev();

    iter::from_fn(move || if start > end { rev.next() } else { fwd.next() })
}

/// Iterates through every pair in the input slice. Returns references to the input.
///
/// ```
/// let input = [1, 2, 3];
/// let want = [(&input[0], &input[1]), (&input[0], &input[2]), (&input[1], &input[2])];
/// assert_eq!(aoclib_rs::iter::pairwise_iter(&input).collect::<Vec<_>>(), want);
/// ```
pub fn pairwise_iter<T>(v: &[T]) -> impl Iterator<Item = (&T, &T)> {
    let mut it = pairwise_iter_i(v);
    iter::from_fn(move || it.next().map(|(i, j)| (&v[i], &v[j])))
}

/// Iterates through every pair in the input slice. Returns copies of the input.
///
/// ```
/// let input = [1, 2, 3];
/// let want = [(1, 2), (1, 3), (2, 3)];
/// assert_eq!(aoclib_rs::iter::pairwise_iter_copy(&input).collect::<Vec<_>>(), want);
/// ```
pub fn pairwise_iter_copy<T: Copy>(v: &[T]) -> impl Iterator<Item = (T, T)> {
    let mut it = pairwise_iter_i(v);
    iter::from_fn(move || it.next().map(|(i, j)| (v[i], v[j])))
}

/// Iterates through every pair in the input slice. Returns indices into the input.
///
/// ```
/// let input = [1, 2, 3];
/// let want = [(0, 1), (0, 2), (1, 2)];
/// assert_eq!(aoclib_rs::iter::pairwise_iter_i(&input).collect::<Vec<_>>(), want);
/// ```
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

/// Iterates through a "selector" of `len` elements, generating every possible selection. The idea
/// is that you "select" the elements in some other structure at the indices where the returned `Vec`
/// is true. Put another way, the returned `Vec<bool>`s can be looked at as counting up in binary.
///
/// ```
/// assert_eq!(
///     aoclib_rs::iter::selector_iter(2).collect::<Vec<_>>(),
///     [
///         vec![false, false],
///         vec![false, true],
///         vec![true, false],
///         vec![true, true]
///     ]
/// );
/// ```
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
/// Iterates through every possible combination of selecting elements from the input. Returns
/// references to the input.
///
/// ```
/// let input = [1, 2];
/// let want = [vec![], vec![&input[1]], vec![&input[0]], vec![&input[0], &input[1]]];
/// assert_eq!(aoclib_rs::iter::selection_iter(&input).collect::<Vec<_>>(), want);
/// ```
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
/// Iterates through all permutations of the input. Returns copies of the input.
///
/// ```
/// assert_eq!(
///     aoclib_rs::iter::permutation_iter_copy(vec![1, 2, 3]).collect::<Vec<_>>(),
///     [
///         vec![1, 2, 3],
///         vec![2, 1, 3],
///         vec![3, 1, 2],
///         vec![1, 3, 2],
///         vec![2, 3, 1],
///         vec![3, 2, 1]
///     ]
/// );
/// ```
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
