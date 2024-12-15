use std::{
    fs::{read_to_string, File},
    io::BufWriter,
    iter,
    str::FromStr,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn rotate_left(self) -> Direction {
        self.rotate_right().opposite()
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn apply_delta_to_usizes(self, usizes: (usize, usize)) -> (usize, usize) {
        let (d_x, d_y) = self.delta();
        (usize_plus_i32(usizes.0, d_x), usize_plus_i32(usizes.1, d_y))
    }
}

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

pub fn fwd_rev_incl_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    let mut fwd = start..=end;
    let mut rev = (end..=start).rev();

    iter::from_fn(move || if start > end { rev.next() } else { fwd.next() })
}

pub fn pad<T: Clone + Copy>(contents: &Vec<&[T]>, padding: usize, default: T) -> Vec<Vec<T>> {
    let mut r = Vec::with_capacity(contents.len());
    let mut prefix = vec![vec![default; contents[0].len() + padding * 2]; padding];
    r.append(&mut prefix);

    for line in contents {
        let mut v = Vec::with_capacity(line.len() + padding * 2);
        let mut prefix = vec![default; padding];
        let mut middle = line.to_vec();
        let mut suffix = vec![default; padding];
        v.append(&mut prefix);
        v.append(&mut middle);
        v.append(&mut suffix);

        r.push(v);
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
    <T as FromStr>::Err: std::error::Error,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: 'static,
{
    let mut v = Vec::new();
    for n in s.split(delim) {
        v.push(n.parse()?);
    }

    Ok(v)
}

pub fn pairwise_iter<T: Copy>(v: &[T]) -> impl Iterator<Item = (T, T)> + use<'_, T> {
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

        Some((v[i], v[j]))
    })
}

pub fn usize_plus_i32(u: usize, i: i32) -> usize {
    (u as i32 + i) as usize
}

pub fn u8_to_string(c: u8) -> String {
    String::from_utf8(vec![c]).unwrap()
}

pub fn split_by_char(s: &str) -> Vec<&str> {
    s.split("").filter(|c| !c.is_empty()).collect()
}
