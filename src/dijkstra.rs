use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    ops::Add,
};

/// An object on which we can run Dijkstra's algorithm to retrieve the multiple shortest paths.
pub trait Dijkstrable {
    /// "Point", "node", or "vertex" type.
    type Point: Copy;

    /// Type for the bounds of the search (useful if searching in a `Vec` or similar rather than a
    /// graph proper). This is only used in the implementation of `neighbours()`, so it can be
    /// effectively ignored if it's not relevant for that particular implementation.
    type Bounds: ?Sized + Copy;

    /// Type for the distance.
    type Dist: Copy + Add<Output = Self::Dist>;

    /// Type for the elements of a priority queue structure. `PqElement` is often a good choice.
    type PQE: PriorityQueueElement<Point = Self::Point, Dist = Self::Dist>;

    /// Returns an iterator to all the neighbours (or "adjacent points") of the provided `Point`.
    /// Implementations can ignore the `Bounds` if it's not relevant.
    fn neighbours(
        _: Self::Point,
        _: Self::Bounds,
    ) -> impl Iterator<Item = (Self::Point, Self::Dist)>;

    /// Returns whether it is impossible to get to the destination from the current `Point`.
    fn is_impossible(&self, _: Self::Point) -> bool;

    /// Returns the distance between the destination and the current `Point`, or `None` if it has
    /// not been computed yet or is impossible.
    fn dist(&self, _: Self::Point) -> Option<Self::Dist>;

    /// Sets the distance between the destination and the current `Point`. If the destination and
    /// the current `Point` are not connected, `None` will be passed in.
    fn set_dist(&mut self, _: Self::Point, _: Option<Self::Dist>);

    /// Performs Dijkstra's algorithm. The result is the ending `dist()` value for the starting
    /// point.
    fn dijkstra(&mut self, start: Self::Point, start_dist: Self::Dist, bounds: Self::Bounds) {
        let mut q = BinaryHeap::new();
        q.push(Reverse(Self::PQE::init(start, start_dist)));

        while !q.is_empty() {
            let curr = q.pop().unwrap();

            for n in Self::neighbours(curr.0.point(), bounds) {
                let d = if self.is_impossible(n.0) {
                    None
                } else {
                    Some(curr.0.dist() + n.1)
                };
                if let Some(dval) = d
                    && self.dist(n.0).is_none()
                {
                    self.set_dist(n.0, d);
                    q.push(Reverse(Self::PQE::init(n.0, dval)));
                }
            }
        }
    }
}

/// Element of a priority queue.
pub trait PriorityQueueElement: Ord + Copy + Sized {
    /// "Point", "node", or "vertex" type.
    type Point;

    /// Type for the distance or "value" of the point.
    type Dist: Add<Output = Self::Dist>;

    fn init(_: Self::Point, _: Self::Dist) -> Self;
    fn point(&self) -> Self::Point;
    fn dist(&self) -> Self::Dist;
}

/// Concrete implementation of `PriorityQueueElement`. Comparison and equality operations only care
/// about `val`.
#[derive(Copy, Clone)]
pub struct PqElement<Point, Value>
where
    Point: Copy,
    Value: Copy + Add<Output = Value> + Ord,
{
    point: Point,
    val: Value,
}

impl<Point, Value> Ord for PqElement<Point, Value>
where
    Point: Copy,
    Value: Copy + Add<Output = Value> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl<Point, Value> PartialOrd for PqElement<Point, Value>
where
    Point: Copy,
    Value: Copy + Add<Output = Value> + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Point, Value> PartialEq for PqElement<Point, Value>
where
    Point: Copy,
    Value: Copy + Add<Output = Value> + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<Point, Value> Eq for PqElement<Point, Value>
where
    Point: Copy,
    Value: Copy + Add<Output = Value> + Ord,
{
}

impl<Point, Value> PriorityQueueElement for PqElement<Point, Value>
where
    Point: Copy,
    Value: Copy + Add<Output = Value> + Ord,
{
    type Point = Point;
    type Dist = Value;

    fn init(p: Self::Point, d: Self::Dist) -> Self {
        PqElement { point: p, val: d }
    }

    fn point(&self) -> Self::Point {
        self.point
    }

    fn dist(&self) -> Self::Dist {
        self.val
    }
}
