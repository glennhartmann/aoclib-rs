use std::{
    f64::consts::{FRAC_PI_2, PI},
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Sub},
};

use num_traits::{NumCast, Zero};

/// For floats, try the ordered_float crate
pub trait Point<T>: Clone + Debug + Eq + Hash + PartialEq
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    fn dist(&self, other: &Self) -> f64;
    fn manhattan(&self, other: &Self) -> T;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    p: PointNd<T, 2>,
}

impl<T> Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    pub fn new(x: T, y: T) -> Self {
        Self {
            p: PointNd { vals: [x, y] },
        }
    }

    pub fn from_array(vals: [T; 2]) -> Self {
        Self {
            p: PointNd { vals },
        }
    }

    pub fn x(&self) -> T {
        self.p.vals[0]
    }

    pub fn y(&self) -> T {
        self.p.vals[1]
    }
}

impl Point2d<i64> {
    pub fn get_slope(&self, other: &Self) -> anyhow::Result<Slope> {
        Slope::from_points_2d(self, other)
    }

    pub fn get_angle(&self, other: &Self) -> anyhow::Result<f64> {
        self.get_slope(other)?.get_angle()
    }
}

impl<T> Point<T> for Point2d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    fn dist(&self, other: &Self) -> f64 {
        self.p.dist(&other.p)
    }

    fn manhattan(&self, other: &Self) -> T {
        self.p.manhattan(&other.p)
    }
}

// TODO: make generic?
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

    // TODO: more safety (no 'as')
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    p: PointNd<T, 3>,
}

impl<T> Point3d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            p: PointNd { vals: [x, y, z] },
        }
    }

    pub fn from_array(vals: [T; 3]) -> Self {
        Self {
            p: PointNd { vals },
        }
    }

    pub fn x(&self) -> T {
        self.p.vals[0]
    }

    pub fn y(&self) -> T {
        self.p.vals[1]
    }

    pub fn z(&self) -> T {
        self.p.vals[2]
    }
}

impl<T> Point<T> for Point3d<T>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    fn dist(&self, other: &Self) -> f64 {
        self.p.dist(&other.p)
    }

    fn manhattan(&self, other: &Self) -> T {
        self.p.manhattan(&other.p)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointNd<T, const N: usize>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    vals: [T; N],
}

impl<T, const N: usize> PointNd<T, N>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    pub fn new(vals: [T; N]) -> Self {
        Self { vals }
    }

    pub fn n(&self, i: usize) -> T {
        self.vals[i]
    }
}

impl<T, const N: usize> Point<T> for PointNd<T, N>
where
    T: Add<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Debug
        + Eq
        + Hash
        + NumCast
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Zero,
{
    fn dist(&self, other: &Self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 0..N {
            let v_f64: f64 = num_traits::cast(self.vals[i]).unwrap();
            let other_v_f64: f64 = num_traits::cast(other.vals[i]).unwrap();
            sum += (v_f64 - other_v_f64).powi(2);
        }
        sum.sqrt()
    }

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
