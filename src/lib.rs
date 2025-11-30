pub mod binary_search;
pub mod dijkstra;
pub mod dir;
pub mod iter;
pub mod option_min_max;
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
pub fn gcd<T>(a: T, b: T) -> anyhow::Result<T>
where
    T: PartialEq + PartialOrd<i64> + fmt::Display + ops::Rem<Output = T> + Copy,
{
    if a < 0 || b < 0 {
        return Err(anyhow::anyhow!(
            "should only be used on positive numbers (got {} and {})",
            a,
            b
        ));
    }

    if b == 0 {
        return Ok(a);
    }

    gcd(b, a % b)
}

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

// TODO
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
