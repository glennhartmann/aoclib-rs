use std::{
    f64::consts::{FRAC_PI_2, PI},
    fmt::Debug,
    hash::Hash,
    ops::{AddAssign, Sub},
};

use num_traits::{NumCast, Zero};

/// A point capable of computing the straight-line distance between itself and another point of the
/// same type.
pub trait PointDist {
    /// Computes the straight-line distance between `self` and `other`.
    fn dist(&self, other: &Self) -> f64;
}

/// A point capable of computing the Manhattan distance between itself and another point of the
/// same type.
pub trait PointManhattan<T> {
    /// Computes the Manhattan distance between `self` and `other`.
    fn manhattan(&self, other: &Self) -> T;
}

/// 2-dimensional point.
/// For full functionality for floats, try the `ordered_float` crate
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point2d<T: Copy>(PointNd<T, 2>);

impl<T: Copy> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(PointNd { vals: [x, y] })
    }

    pub fn from_array(vals: [T; 2]) -> Self {
        Self(PointNd { vals })
    }

    pub fn x(&self) -> T {
        self.0.vals[0]
    }

    pub fn y(&self) -> T {
        self.0.vals[1]
    }
}

impl Point2d<i64> {
    /// Returns the slope between `self` and `other`. Can return an error in some cases (eg. divide
    /// by zero).
    pub fn get_slope(&self, other: &Self) -> anyhow::Result<Slope> {
        Slope::from_points_2d(self, other)
    }

    /// Returns the angle (from vertical) between `self` and `other` in radians. Can return an
    /// error in some cases (eg. divide by zero).
    pub fn get_angle(&self, other: &Self) -> anyhow::Result<f64> {
        self.get_slope(other)?.get_angle()
    }
}

impl<T> PointDist for Point2d<T>
where
    T: Copy + NumCast,
{
    /// Computes the straight-line distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{Point2d, PointDist};
    /// assert_eq!(Point2d::new(1, 1).dist(&Point2d::new(4, 5)), 5.0);
    /// ```
    fn dist(&self, other: &Self) -> f64 {
        self.0.dist(&other.0)
    }
}

impl<T> PointManhattan<T> for Point2d<T>
where
    T: AddAssign + Copy + PartialOrd + Sub<Output = T> + Zero,
{
    /// Computes the Manhattan distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{Point2d, PointManhattan};
    /// assert_eq!(Point2d::new(1, 1).manhattan(&Point2d::new(4, 5)), 7);
    /// ```
    fn manhattan(&self, other: &Self) -> T {
        self.0.manhattan(&other.0)
    }
}

// TODO: make generic? Or use Rationals?
/// A slope between two points. Generally should be assumed to only work properly with positive values.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Slope {
    horizontal: i64,
    vertical: i64,
}

impl Slope {
    pub fn new(horizontal: i64, vertical: i64) -> anyhow::Result<Self> {
        let mut s = Self {
            horizontal,
            vertical,
        };
        s.simplify()?;
        Ok(s)
    }

    // TODO: what about negatives and stuff?
    pub fn from_points_2d(pov: &Point2d<i64>, other: &Point2d<i64>) -> anyhow::Result<Self> {
        Self::new(other.x() - pov.x(), other.y() - pov.y())
    }

    fn simplify(&mut self) -> anyhow::Result<()> {
        let gcd = crate::gcd(self.horizontal.abs(), self.vertical.abs())?;
        self.horizontal /= gcd;
        self.vertical /= gcd;
        Ok(())
    }

    pub fn horizontal(&self) -> i64 {
        self.horizontal
    }

    pub fn vertical(&self) -> i64 {
        self.vertical
    }

    /// Returns the angle (from vertical) represented by this slope in radians. Can return an error
    /// in some cases (eg. divide by zero).
    pub fn get_angle(&self) -> anyhow::Result<f64> {
        match self {
            Slope {
                horizontal: 0,
                vertical: 0,
            } => Err(anyhow::anyhow!("0 slope")),

            // up
            Slope {
                horizontal: 0,
                vertical: i64::MIN..0,
            } => Ok(0.0),

            // right
            Slope {
                horizontal: 0..=i64::MAX,
                vertical: 0,
            } => Ok(FRAC_PI_2),

            // down
            Slope {
                horizontal: 0,
                vertical: 0..=i64::MAX,
            } => Ok(PI),

            // left
            Slope {
                horizontal: i64::MIN..0,
                vertical: 0,
            } => Ok(PI + FRAC_PI_2),

            // top-right
            Slope {
                horizontal: 0..=i64::MAX,
                vertical: i64::MIN..0,
            } => Ok((self.horizontal as f64 / -self.vertical as f64).atan()),

            // bottom-right
            Slope {
                horizontal: 0..=i64::MAX,
                vertical: 0..=i64::MAX,
            } => Ok((self.vertical as f64 / self.horizontal as f64).atan() + FRAC_PI_2),

            // bottom-left
            Slope {
                horizontal: i64::MIN..0,
                vertical: 0..=i64::MAX,
            } => Ok((-self.horizontal as f64 / self.vertical as f64).atan() + PI),

            // top-left
            Slope {
                horizontal: i64::MIN..0,
                vertical: i64::MIN..0,
            } => Ok((-self.vertical as f64 / -self.horizontal as f64).atan() + PI + FRAC_PI_2),
        }
    }
}

/// 2-dimensional point.
/// For full functionality for floats, try the `ordered_float` crate
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3d<T: Copy>(PointNd<T, 3>);

impl<T: Copy> Point3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(PointNd { vals: [x, y, z] })
    }

    pub fn from_array(vals: [T; 3]) -> Self {
        Self(PointNd { vals })
    }

    pub fn x(&self) -> T {
        self.0.vals[0]
    }

    pub fn y(&self) -> T {
        self.0.vals[1]
    }

    pub fn z(&self) -> T {
        self.0.vals[2]
    }
}

impl<T> PointDist for Point3d<T>
where
    T: Copy + NumCast,
{
    /// Computes the straight-line distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{Point3d, PointDist};
    /// assert_eq!(Point3d::new(2, 4, 3).dist(&Point3d::new(6, 9, 23)), 21.0);
    /// ```
    fn dist(&self, other: &Self) -> f64 {
        self.0.dist(&other.0)
    }
}

impl<T> PointManhattan<T> for Point3d<T>
where
    T: AddAssign + Copy + PartialOrd + Sub<Output = T> + Zero,
{
    /// Computes the Manhattan distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{Point3d, PointManhattan};
    /// assert_eq!(Point3d::new(2, 4, 3).manhattan(&Point3d::new(6, 9, 23)), 29);
    /// ```
    fn manhattan(&self, other: &Self) -> T {
        self.0.manhattan(&other.0)
    }
}

/// N-dimensional point.
/// For full functionality for floats, try the `ordered_float` crate
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointNd<T: Copy, const N: usize> {
    vals: [T; N],
}

impl<T: Copy, const N: usize> PointNd<T, N> {
    pub fn new(vals: [T; N]) -> Self {
        Self { vals }
    }

    pub fn n(&self, i: usize) -> T {
        self.vals[i]
    }
}

impl<T, const N: usize> PointDist for PointNd<T, N>
where
    T: Copy + NumCast,
{
    /// Computes the straight-line distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{PointNd, PointDist};
    /// assert_eq!(PointNd::new([6, 8, 2, 9]).dist(&PointNd::new([8, 12, 5, 23])), 15.0);
    /// ```
    fn dist(&self, other: &Self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 0..N {
            let v_f64: f64 = num_traits::cast(self.vals[i]).unwrap();
            let other_v_f64: f64 = num_traits::cast(other.vals[i]).unwrap();
            sum += (v_f64 - other_v_f64).powi(2);
        }
        sum.sqrt()
    }
}

impl<T, const N: usize> PointManhattan<T> for PointNd<T, N>
where
    T: AddAssign + Copy + PartialOrd + Sub<Output = T> + Zero,
{
    /// Computes the Manhattan distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{PointNd, PointManhattan};
    /// assert_eq!(PointNd::new([6, 8, 2, 9]).manhattan(&PointNd::new([8, 12, 5, 23])), 23);
    /// ```
    fn manhattan(&self, other: &Self) -> T {
        let mut total: T = T::zero();
        for i in 0..N {
            let v_abs = if self.vals[i] > other.vals[i] {
                self.vals[i] - other.vals[i]
            } else {
                other.vals[i] - self.vals[i]
            };
            total += v_abs;
        }
        total
    }
}
