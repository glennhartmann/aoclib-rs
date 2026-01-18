use std::iter;

use crate::usize_plus_i;

pub trait Direction: Sized + PartialEq + Copy {
    /// Returns an iterator through all values of the `Direction`, starting with `Up` and
    /// proceeding clockwise.
    fn iter() -> impl Iterator<Item = Self>;

    /// Returns an iterator through all values of the `Direction`, starting with the given
    /// `initial_dir` and proceeding clockwise.
    ///
    /// ```
    /// use aoclib_rs::dir::{Dir4, Dir8, Direction};
    /// assert_eq!(
    ///     Dir4::iter_from(Dir4::Left).collect::<Vec<_>>(),
    ///     [Dir4::Left, Dir4::Up, Dir4::Right, Dir4::Down]
    /// );
    /// assert_eq!(
    ///     Dir8::iter_from(Dir8::Dir4(Dir4::Left)).collect::<Vec<_>>(),
    ///     [
    ///         Dir8::Dir4(Dir4::Left),
    ///         Dir8::UpLeft,
    ///         Dir8::Dir4(Dir4::Up),
    ///         Dir8::UpRight,
    ///         Dir8::Dir4(Dir4::Right),
    ///         Dir8::DownRight,
    ///         Dir8::Dir4(Dir4::Down),
    ///         Dir8::DownLeft
    ///     ]
    /// );
    /// ```
    fn iter_from(initial_dir: Self) -> impl Iterator<Item = Self> {
        let mut d = initial_dir;
        let mut first = true;
        iter::from_fn(move || {
            if d == initial_dir && !first {
                return None;
            }
            first = false;

            let r = d;
            d = d.rotate_right();

            Some(r)
        })
    }

    /// Similar to `apply_delta_to_usizes()`, but iterates through all valid directions. A valid
    /// direction is defined as any whose result is non-negative and is less than `size` in the
    /// respective dimension.
    ///
    /// ```
    /// use aoclib_rs::dir::{Dir8, Direction};
    /// assert_eq!(
    ///     Dir8::iter_valid_usizes_deltas((0, 5), (4, 6)).collect::<Vec<_>>(),
    ///     [(0, 4), (1, 4), (1, 5)]
    /// );
    /// ```
    fn iter_valid_usizes_deltas(
        curr: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut dir_iter = Self::iter();
        iter::from_fn(move || {
            loop {
                match dir_iter.next() {
                    None => return None,
                    Some(d) => {
                        let (dx, dy) = d.delta();
                        let next = (
                            i64::try_from(curr.0).unwrap() + i64::from(dx),
                            i64::try_from(curr.1).unwrap() + i64::from(dy),
                        );
                        if next.0 >= 0
                            && next.0 < i64::try_from(size.0).unwrap()
                            && next.1 >= 0
                            && next.1 < i64::try_from(size.1).unwrap()
                        {
                            return Some((
                                usize::try_from(next.0).unwrap(),
                                usize::try_from(next.1).unwrap(),
                            ));
                        }
                    }
                }
            }
        })
    }

    /// Returns the `(x, y)` delta for one unit of the `Direction`.
    fn delta(self) -> (i8, i8);

    /// Returns a new `Direction` which represents the given one being rotated "right" or clockwise by one unit.
    fn rotate_right(self) -> Self;

    /// Returns a new `Direction` which represents the given one being rotated "left" or counter-clockwise by one unit.
    fn rotate_left(self) -> Self;

    /// Returns a new `Direction` which represents the given one being rotated "right" or clockwise by 90 degrees.
    fn rotate_right_90(self) -> Self;

    /// Returns a new `Direction` which represents the given one being rotated "left" or counter-clockwise by 90 degrees.
    fn rotate_left_90(self) -> Self;

    /// Returns a new `Direction` which represents the "opposite" of the given one. Can also be
    /// seen as rotating the given one by 180 degrees.
    fn opposite(self) -> Self;

    /// Takes in a pair of `usize`s representing (x, y) coordinates, and returns the result of
    /// "moving" one unit in the given direction. Can panic (see `usizes_plus_i()` documentation).
    ///
    /// ```
    /// use aoclib_rs::dir::{Dir4, Dir8, Direction};
    /// assert_eq!(Dir4::Right.apply_delta_to_usizes((4, 5)), (5, 5));
    /// assert_eq!(Dir8::UpRight.apply_delta_to_usizes((4, 5)), (5, 4));
    ///
    /// // Dir4::Left.apply_delta_to_usizes((0, 0));
    /// // panics: -1 is an invalid usize
    /// ```
    fn apply_delta_to_usizes(self, usizes: (usize, usize)) -> (usize, usize) {
        let (d_x, d_y) = self.delta();
        (
            usize_plus_i(usizes.0, i64::from(d_x)),
            usize_plus_i(usizes.1, i64::from(d_y)),
        )
    }
}

/// The four cardinal directions.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Direction for Dir4 {
    fn delta(self) -> (i8, i8) {
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
        Self::iter_from(Dir4::Up)
    }
}

/// The four cardinal directions, plus the 4 diagonals in between.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir8 {
    Dir4(Dir4),
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction for Dir8 {
    fn delta(self) -> (i8, i8) {
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
            Dir8::Dir4(d4) => Dir8::Dir4(d4.rotate_right_90()),
            Dir8::UpRight => Dir8::DownRight,
            Dir8::UpLeft => Dir8::UpRight,
            Dir8::DownRight => Dir8::DownLeft,
            Dir8::DownLeft => Dir8::UpLeft,
        }
    }

    fn rotate_left_90(self) -> Dir8 {
        match self {
            Dir8::Dir4(d4) => Dir8::Dir4(d4.rotate_left_90()),
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
        Self::iter_from(Dir8::Dir4(Dir4::Up))
    }
}
