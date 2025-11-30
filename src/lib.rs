pub mod binary_search;
pub mod dijkstra;
pub mod dir;
pub mod iter;
pub mod matrix;
pub mod option_min_max;
pub mod point;
pub mod trie;

use std::{
    fmt,
    fs::{File, read_to_string},
    io::BufWriter,
    ops,
    str::FromStr,
};

#[macro_export]
macro_rules! printwriteln {
    ($writer:expr, $fmt:literal) => {
        {
            println!($fmt);
            writeln!($writer, $fmt)
        }
    };
    ($writer:expr, $fmt:literal, $($args:expr),+) => {
        {
            println!($fmt, $($args),+);
            writeln!($writer, $fmt, $($args),+)
        }
    };
}

pub fn pad<T: Clone + Copy>(contents: &Vec<&[T]>, padding: usize, default: T) -> Vec<Vec<T>> {
    let mut r = Vec::with_capacity(contents.len());
    let mut prefix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut prefix);

    for line in contents {
        let prefix = vec![default; padding];
        let middle = line.to_vec();
        let suffix = vec![default; padding];

        r.push(vec![prefix, middle, suffix].into_iter().flatten().collect());
    }

    let mut suffix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut suffix);

    r
}

// TODO: having 2 exactly identical functions is stupid. Find a way to fix this.
pub fn pad_vec<T: Clone + Copy>(contents: &Vec<Vec<T>>, padding: usize, default: T) -> Vec<Vec<T>> {
    let mut r = Vec::with_capacity(contents.len());
    let mut prefix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut prefix);

    for line in contents {
        let prefix = vec![default; padding];
        let middle = line.to_vec();
        let suffix = vec![default; padding];

        r.push(vec![prefix, middle, suffix].into_iter().flatten().collect());
    }

    let mut suffix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut suffix);

    r
}

pub fn prep_io(contents: &mut String, day: u8) -> anyhow::Result<(BufWriter<File>, Vec<&str>)> {
    *contents = read_to_string(format!("inputs/{:02}.txt", day))?;
    let contents: Vec<&str> = contents.trim().lines().collect();

    let write_file = File::create(format!("outputs/{:02}.txt", day))?;
    let writer = BufWriter::new(write_file);

    Ok((writer, contents))
}

pub fn split_and_parse<T: FromStr>(s: &str, delim: &str) -> anyhow::Result<Vec<T>>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let mut v = Vec::new();
    for n in s.split(delim) {
        v.push(n.parse()?);
    }

    Ok(v)
}

/// Adds generic type to usize. Can panic if the values are outside the range of the given types.
///
/// ```
/// use aoclib_rs::usize_plus_i;
///
/// let i: i64 = 23;
/// assert_eq!(usize_plus_i(5, i), 28);
///
/// let i: i64 = -2;
/// assert_eq!(usize_plus_i(5, i), 3);
///
/// // let i: i64 = -8;
/// // usize_plus_i(5, i);
/// // panics: -3 is an invalid usize
///
/// // let i: i8 = 5;
/// // usize_plus_i(12345, i);
/// // panics: 12345 is an invalid i8
/// ```
pub fn usize_plus_i<T>(u: usize, i: T) -> usize
where
    T: TryFrom<usize, Error: fmt::Debug> + ops::Add<Output = T>,
    usize: TryFrom<T, Error: fmt::Debug>,
{
    usize::try_from(T::try_from(u).unwrap() + i).unwrap()
}

pub fn u8_to_string(c: u8) -> String {
    String::from_utf8(vec![c]).unwrap()
}

// this function is pretty silly and new code should probably just use s.chars().
pub fn split_by_char(s: &str) -> Vec<&str> {
    s.split("").filter(|c| !c.is_empty()).collect()
}

/// Given 2-dimensional data and a predicate, `position_2d()` returns the position (ie, the 2D
/// indices) of where the predicate first returned `true`, or `None` if the predicate never returns
/// `true`. The input data is interpreted as follows:
///
/// `vec![vec![1, 2, 3], vec![4, 5, 6]]` is
///
/// 1 2 3
/// 4 5 6
///
/// That is, the indices of the `3` are `(2, 0)` - in `(x, y)` form.
///
/// Iteration order is left-to-right, top-to-bottom.
///
/// ```
/// let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
/// assert_eq!(aoclib_rs::position_2d(&v, |&i| i == 3), Some((2, 0)));
/// ```
pub fn position_2d<T, P>(v: &[Vec<T>], mut predicate: P) -> Option<(usize, usize)>
where
    P: FnMut(&T) -> bool,
{
    for (y, row) in v.iter().enumerate() {
        if let Some(x) = row.iter().position(&mut predicate) {
            return Some((x, y));
        }
    }

    None
}

// Euclidean Algorithm https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
/// Greatest common divisor. Behaviour when calling with negative numbers can be unintuitive; refer
/// to examples below.
///
/// ```
/// assert_eq!(aoclib_rs::gcd(8, 12), 4);
/// assert_eq!(aoclib_rs::gcd(-8, -12), -4);
/// assert_eq!(aoclib_rs::gcd(-8, 12), 4);
/// assert_eq!(aoclib_rs::gcd(8, -12), -4);
/// ```
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + ops::Rem<Output = T> + PartialEq<i64>,
{
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

/// Increments a "selector" slice of booleans. Essentially treats the slice as a binary number and
/// increments it. Returns `true` if the input is already all true and doesn't increment. Returns
/// `false` and increments otherwise.
///
/// ```
/// use aoclib_rs::inc_selector;
///
/// let mut v = [true, false, false, true, true];
/// assert_eq!(inc_selector(&mut v), false);
/// assert_eq!(v, [true, false, true, false, false]);
///
/// let mut v = [true, true, true, true, true];
/// assert_eq!(inc_selector(&mut v), true);
/// assert_eq!(v, [true, true, true, true, true]);
/// ```
pub fn inc_selector(v: &mut [bool]) -> bool {
    if v.iter().map(|&b| if b { 1 } else { 0 }).sum::<usize>() == v.len() {
        return true;
    }

    let mut last = v.len() - 1;
    loop {
        v[last] = !v[last];

        if v[last] {
            break;
        }

        match last.checked_sub(1) {
            None => break,
            Some(l) => last = l,
        }
    }

    false
}

/// Essentially a short-circuiting version of `Iterator::fold()` (see
/// <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold>). The only difference
/// between this and `Iterator::fold()` is that the closure in this function returns two values: the
/// accumulator value and a boolean. If the boolean returned is `true`, `fold_while()` will continue
/// iterating, otherwise it will terminate early.
///
/// ```
/// assert_eq!(
///     // Sums items, but exits after item `3`.
///     aoclib_rs::fold_while([1, 2, 3, 4, 5].iter(), 0, |acc, &item| (
///         acc + item,
///         item != 3
///     )),
///     6
/// );
/// ```
pub fn fold_while<I, B, F>(it: I, init: B, mut f: F) -> B
where
    I: Iterator,
    F: FnMut(B, I::Item) -> (B, bool),
{
    let mut b = init;
    for v in it {
        let (new_b, should_continue) = f(b, v);
        b = new_b;
        if !should_continue {
            break;
        }
    }
    b
}
