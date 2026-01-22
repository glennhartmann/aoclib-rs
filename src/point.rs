use std::{
    f64::consts::{FRAC_PI_2, PI},
    fmt::Debug,
    hash::Hash,
    iter::zip,
    ops::{DivAssign, Rem, Sub},
};

use crate::abs;

use num_traits::{NumCast, Zero};

/// A point that can be initialized with an array of the appropriate size.
pub trait PointFromArray<T, const N: usize> {
    /// Initializes a point from an array.
    fn from_array(vals: [T; N]) -> Self;
}

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

    pub fn x(&self) -> T {
        self.0.vals[0]
    }

    pub fn y(&self) -> T {
        self.0.vals[1]
    }
}

impl<T> Point2d<T>
where
    T: Copy + DivAssign + PartialOrd + Rem<Output = T> + Sub<Output = T> + Zero,
{
    /// Returns the slope between `self` and `other`. Can return an error if the x values of both
    /// points are the same (to avoid dividing by zero).
    pub fn get_slope(&self, other: &Self) -> anyhow::Result<Slope<T>> {
        Slope::from_points_2d(self, other)
    }
}

impl<T> Point2d<T>
where
    T: Copy + NumCast + PartialOrd + Sub<Output = T> + Zero,
{
    /// Returns the angle (from straight up, going clockwise) between `self` and `other` in
    /// radians. Can return an error if both points are the same.
    ///
    /// ```
    /// use aoclib_rs::point::Point2d;
    /// assert_eq!(
    ///     Point2d::new(0, 0).get_angle(&Point2d::new(1, 0)).unwrap(),
    ///     std::f64::consts::FRAC_PI_2
    /// );
    /// ```
    pub fn get_angle(&self, other: &Self) -> anyhow::Result<f64> {
        get_angle(other.x() - self.x(), other.y() - self.y())
    }
}

impl<T: Copy> PointFromArray<T, 2> for Point2d<T> {
    fn from_array(vals: [T; 2]) -> Self {
        Self(PointNd { vals })
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
    T: Copy + PartialOrd + Sub<Output = T> + Zero,
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

/// 3-dimensional point.
/// For full functionality for floats, try the `ordered_float` crate
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3d<T: Copy>(PointNd<T, 3>);

impl<T: Copy> Point3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(PointNd { vals: [x, y, z] })
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

impl<T: Copy> PointFromArray<T, 3> for Point3d<T> {
    fn from_array(vals: [T; 3]) -> Self {
        Self(PointNd { vals })
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
    T: Copy + PartialOrd + Sub<Output = T> + Zero,
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

impl<T: Copy, const N: usize> PointFromArray<T, N> for PointNd<T, N> {
    fn from_array(vals: [T; N]) -> Self {
        Self::new(vals)
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
        zip(self.vals.iter(), other.vals.iter())
            .fold(0.0, |acc, (&s, &o)| {
                let v_f64: f64 = num_traits::cast(s).unwrap();
                let other_v_f64: f64 = num_traits::cast(o).unwrap();
                acc + (v_f64 - other_v_f64).powi(2)
            })
            .sqrt()
    }
}

impl<T, const N: usize> PointManhattan<T> for PointNd<T, N>
where
    T: Copy + PartialOrd + Sub<Output = T> + Zero,
{
    /// Computes the Manhattan distance between `self` and `other`.
    ///
    /// ```
    /// use aoclib_rs::point::{PointNd, PointManhattan};
    /// assert_eq!(PointNd::new([6, 8, 2, 9]).manhattan(&PointNd::new([8, 12, 5, 23])), 23);
    /// ```
    fn manhattan(&self, other: &Self) -> T {
        zip(self.vals.iter(), other.vals.iter()).fold(T::zero(), |acc, (&s, &o)| acc + abs(s - o))
    }
}

/// A simplified slope between two integer points.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Slope<T>
where
    T: Copy + DivAssign + PartialOrd + Rem<Output = T> + Sub<Output = T> + Zero,
{
    horizontal: T,
    vertical: T,
}

impl<T> Slope<T>
where
    T: Copy + DivAssign + PartialOrd + Rem<Output = T> + Sub<Output = T> + Zero,
{
    /// Can return an error if `horizontal` and `vertical` are both `0`.
    ///
    /// ```
    /// let slope = aoclib_rs::point::Slope::new(2, 4).unwrap();
    /// assert_eq!(slope.horizontal(), 1);
    /// assert_eq!(slope.vertical(), 2);
    /// ```
    pub fn new(horizontal: T, vertical: T) -> anyhow::Result<Self> {
        let mut s = Self {
            horizontal,
            vertical,
        };
        s.simplify()?;
        Ok(s)
    }

    /// Can return an error if both points are the same.
    ///
    /// ```
    /// use aoclib_rs::point::{Point2d, Slope};
    /// let slope = Slope::from_points_2d(&Point2d::new(0, 0), &Point2d::new(2, 4)).unwrap();
    /// assert_eq!(slope.horizontal(), 1);
    /// assert_eq!(slope.vertical(), 2);
    /// ```
    pub fn from_points_2d(pov: &Point2d<T>, other: &Point2d<T>) -> anyhow::Result<Self> {
        Self::new(other.x() - pov.x(), other.y() - pov.y())
    }

    pub fn horizontal(&self) -> T {
        self.horizontal
    }

    pub fn vertical(&self) -> T {
        self.vertical
    }

    /// Reduces the numerator and denominator of the slope by dividing them by the GCD.
    fn simplify(&mut self) -> anyhow::Result<()> {
        let gcd = crate::gcd(abs(self.horizontal), abs(self.vertical));
        if gcd == T::zero() {
            anyhow::bail!("gcd == 0 - avoiding divide-by-zero");
        }
        self.horizontal /= gcd;
        self.vertical /= gcd;
        Ok(())
    }
}

impl<T> Slope<T>
where
    T: Copy + DivAssign + NumCast + PartialOrd + Rem<Output = T> + Sub<Output = T> + Zero,
{
    /// Returns the angle (from straight up, going clockwise) of the slope in radians.
    ///
    /// ```
    /// use aoclib_rs::point::Slope;
    /// assert_eq!(Slope::new(1, 0).unwrap().get_angle(), std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn get_angle(&self) -> f64 {
        // Should be impossible for get_angle() to fail since it would already have failed in
        // simplify().
        get_angle(self.horizontal, self.vertical).unwrap()
    }
}

/// Returns the angle (from straight up, going clockwise) represented by this slope in radians.
///
/// ```
/// assert_eq!(aoclib_rs::point::get_angle(1, 0).unwrap(), std::f64::consts::FRAC_PI_2);
/// ```
pub fn get_angle<T>(horizontal: T, vertical: T) -> anyhow::Result<f64>
where
    T: Copy + NumCast + PartialOrd + Zero,
{
    if horizontal == T::zero() && vertical == T::zero() {
        anyhow::bail!("0 slope");
    }

    if horizontal == T::zero() {
        // up
        if vertical < T::zero() {
            return Ok(0.0);
        }

        // down
        return Ok(PI);
    }

    if vertical == T::zero() {
        // right
        if horizontal > T::zero() {
            return Ok(FRAC_PI_2);
        }

        // left
        return Ok(PI + FRAC_PI_2);
    }

    let horizontal_as_f64: f64 = num_traits::cast(horizontal).unwrap();
    let vertical_as_f64: f64 = num_traits::cast(vertical).unwrap();

    if horizontal > T::zero() {
        // top-right
        if vertical < T::zero() {
            return Ok((horizontal_as_f64 / -vertical_as_f64).atan());
        }

        // bottom-right
        return Ok((vertical_as_f64 / horizontal_as_f64).atan() + FRAC_PI_2);
    }

    // bottom-left
    if vertical > T::zero() {
        return Ok((-horizontal_as_f64 / vertical_as_f64).atan() + PI);
    }

    // top-left
    Ok((-vertical_as_f64 / -horizontal_as_f64).atan() + PI + FRAC_PI_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::FRAC_PI_4;

    fn get_angle_good_helper<T>(func: fn(x: T, y: T) -> f64)
    where
        T: NumCast,
    {
        let testcases = [
            ((0, -1), 0.0),
            ((1, -1), FRAC_PI_4),
            ((1, 0), FRAC_PI_2),
            ((1, 1), FRAC_PI_2 + FRAC_PI_4),
            ((0, 1), PI),
            ((-1, 1), PI + FRAC_PI_4),
            ((-1, 0), PI + FRAC_PI_2),
            ((-1, -1), PI + FRAC_PI_2 + FRAC_PI_4),
        ];
        for ((x, y), want) in testcases {
            let x_as_t: T = num_traits::cast(x).unwrap();
            let y_as_t: T = num_traits::cast(y).unwrap();
            assert_eq!(func(x_as_t, y_as_t), want);
        }
    }

    #[test]
    fn test_point2d_get_slope_good() {
        let slope = Point2d::new(1, 1).get_slope(&Point2d::new(1, 2)).unwrap();
        assert_eq!(slope.horizontal(), 0);
        assert_eq!(slope.vertical(), 1);
    }

    #[test]
    fn test_point2d_get_slope_same_point() {
        assert!(Point2d::new(1, 1).get_slope(&Point2d::new(1, 1)).is_err());
    }

    #[test]
    fn test_point2d_get_angle_good() {
        get_angle_good_helper(|x: i64, y: i64| {
            Point2d::new(0, 0).get_angle(&Point2d::new(x, y)).unwrap()
        });
        get_angle_good_helper(|x: f64, y: f64| {
            Point2d::new(0.0, 0.0)
                .get_angle(&Point2d::new(x, y))
                .unwrap()
        });
    }

    #[test]
    fn test_point2d_get_angle_same_point() {
        assert!(Point2d::new(1, 1).get_angle(&Point2d::new(1, 1)).is_err());
    }

    #[test]
    fn test_slope_new_zero() {
        assert!(Slope::new(0, 0).is_err());
    }

    #[test]
    fn test_slope_from_points_2d_same_point() {
        assert!(Slope::from_points_2d(&Point2d::new(1, 1), &Point2d::new(1, 1)).is_err());
    }

    #[test]
    fn test_slope_get_angle_good() {
        get_angle_good_helper(|x: i64, y: i64| Slope::new(x, y).unwrap().get_angle());
    }

    #[test]
    fn test_get_angle_good() {
        get_angle_good_helper(|x: i64, y: i64| get_angle(x, y).unwrap());
        get_angle_good_helper(|x: f64, y: f64| get_angle(x, y).unwrap());
    }

    #[test]
    fn test_get_angle_zero() {
        assert!(get_angle(0, 0).is_err());
    }
}
