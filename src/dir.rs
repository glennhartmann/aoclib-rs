use std::iter;

use crate::usize_plus_i32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

    pub fn iter() -> impl Iterator<Item = Direction> {
        let mut d = Direction::Up;
        let mut first = true;
        iter::from_fn(move || {
            if d == Direction::Up && !first {
                return None;
            }
            first = false;

            let r = d;
            d = d.rotate_right();

            Some(r)
        })
    }

    pub fn iter_valid_usizes_deltas(
        curr: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut dir_iter = Direction::iter();
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
