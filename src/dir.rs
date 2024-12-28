use std::iter;

use crate::usize_plus_i32;

pub trait Direction: Sized {
    fn delta(self) -> (i32, i32);
    fn rotate_right(self) -> Self;
    fn rotate_left(self) -> Self;
    fn rotate_right_90(self) -> Self;
    fn rotate_left_90(self) -> Self;
    fn opposite(self) -> Self;
    fn iter() -> impl Iterator<Item = Self>;

    fn apply_delta_to_usizes(self, usizes: (usize, usize)) -> (usize, usize) {
        let (d_x, d_y) = self.delta();
        (usize_plus_i32(usizes.0, d_x), usize_plus_i32(usizes.1, d_y))
    }

    fn iter_valid_usizes_deltas(
        curr: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut dir_iter = Self::iter();
        iter::from_fn(move || loop {
            match dir_iter.next() {
                None => return None,
                Some(d) => {
                    let (dx, dy) = d.delta();
                    let next = (curr.0 as i32 + dx, curr.1 as i32 + dy);
                    if next.0 >= 0
                        && next.0 < size.0 as i32
                        && next.1 >= 0
                        && next.1 < size.1 as i32
                    {
                        return Some((next.0 as usize, next.1 as usize));
                    }
                }
            }
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Direction for Dir4 {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir4::Up => (0, -1),
            Dir4::Down => (0, 1),
            Dir4::Left => (-1, 0),
            Dir4::Right => (1, 0),
        }
    }

    fn rotate_right(self) -> Dir4 {
        match self {
            Dir4::Up => Dir4::Right,
            Dir4::Down => Dir4::Left,
            Dir4::Left => Dir4::Up,
            Dir4::Right => Dir4::Down,
        }
    }

    fn rotate_left(self) -> Dir4 {
        self.rotate_right().opposite()
    }

    fn rotate_right_90(self) -> Dir4 {
        self.rotate_right()
    }

    fn rotate_left_90(self) -> Dir4 {
        self.rotate_left()
    }

    fn opposite(self) -> Dir4 {
        match self {
            Dir4::Up => Dir4::Down,
            Dir4::Down => Dir4::Up,
            Dir4::Left => Dir4::Right,
            Dir4::Right => Dir4::Left,
        }
    }

    fn iter() -> impl Iterator<Item = Dir4> {
        let mut d = Dir4::Up;
        let mut first = true;
        iter::from_fn(move || {
            if d == Dir4::Up && !first {
                return None;
            }
            first = false;

            let r = d;
            d = d.rotate_right();

            Some(r)
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir8 {
    Dir4(Dir4),
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction for Dir8 {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir8::Dir4(d4) => d4.delta(),
            Dir8::UpRight => (1, -1),
            Dir8::UpLeft => (-1, -1),
            Dir8::DownRight => (1, 1),
            Dir8::DownLeft => (-1, 1),
        }
    }

    fn rotate_right(self) -> Dir8 {
        match self {
            Dir8::Dir4(Dir4::Up) => Dir8::UpRight,
            Dir8::Dir4(Dir4::Down) => Dir8::DownLeft,
            Dir8::Dir4(Dir4::Left) => Dir8::UpLeft,
            Dir8::Dir4(Dir4::Right) => Dir8::DownRight,
            Dir8::UpRight => Dir8::Dir4(Dir4::Right),
            Dir8::UpLeft => Dir8::Dir4(Dir4::Up),
            Dir8::DownRight => Dir8::Dir4(Dir4::Down),
            Dir8::DownLeft => Dir8::Dir4(Dir4::Left),
        }
    }

    fn rotate_left(self) -> Dir8 {
        match self {
            Dir8::Dir4(Dir4::Up) => Dir8::UpLeft,
            Dir8::Dir4(Dir4::Down) => Dir8::DownRight,
            Dir8::Dir4(Dir4::Left) => Dir8::DownLeft,
            Dir8::Dir4(Dir4::Right) => Dir8::UpRight,
            Dir8::UpRight => Dir8::Dir4(Dir4::Up),
            Dir8::UpLeft => Dir8::Dir4(Dir4::Left),
            Dir8::DownRight => Dir8::Dir4(Dir4::Right),
            Dir8::DownLeft => Dir8::Dir4(Dir4::Down),
        }
    }

    fn rotate_right_90(self) -> Dir8 {
        match self {
            Dir8::Dir4(d4) => Dir8::Dir4(d4.rotate_right()),
            Dir8::UpRight => Dir8::DownRight,
            Dir8::UpLeft => Dir8::UpRight,
            Dir8::DownRight => Dir8::DownLeft,
            Dir8::DownLeft => Dir8::UpLeft,
        }
    }

    fn rotate_left_90(self) -> Dir8 {
        match self {
            Dir8::Dir4(d4) => Dir8::Dir4(d4.rotate_left()),
            Dir8::UpRight => Dir8::UpLeft,
            Dir8::UpLeft => Dir8::DownLeft,
            Dir8::DownRight => Dir8::UpRight,
            Dir8::DownLeft => Dir8::DownRight,
        }
    }

    fn opposite(self) -> Dir8 {
        match self {
            Dir8::Dir4(d4) => Dir8::Dir4(d4.opposite()),
            Dir8::UpRight => Dir8::DownLeft,
            Dir8::UpLeft => Dir8::DownRight,
            Dir8::DownRight => Dir8::UpLeft,
            Dir8::DownLeft => Dir8::UpRight,
        }
    }

    fn iter() -> impl Iterator<Item = Dir8> {
        let mut d = Dir8::Dir4(Dir4::Up);
        let mut first = true;
        iter::from_fn(move || {
            if d == Dir8::Dir4(Dir4::Up) && !first {
                return None;
            }
            first = false;

            let r = d;
            d = d.rotate_right();

            Some(r)
        })
    }
}
