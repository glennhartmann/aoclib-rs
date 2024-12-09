use std::iter;

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
