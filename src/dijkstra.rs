use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    ops::Add,
};

pub trait Dijkstrable {
    type Point: Copy;
    type Bounds: ?Sized + Copy;
    type Dist: Copy + Add<Output = Self::Dist>;
    type PQE: PriorityQueueElement<Point = Self::Point, Dist = Self::Dist>;

    fn neighbours(
        _: Self::Point,
        _: Self::Bounds,
    ) -> impl Iterator<Item = (Self::Point, Self::Dist)>;

    fn is_impossible(&self, _: Self::Point) -> bool;
    fn dist(&self, _: Self::Point) -> Option<Self::Dist>;
    fn set_dist(&mut self, _: Self::Point, _: Option<Self::Dist>);

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
                if let Some(dval) = d {
                    if self.dist(n.0).is_none() {
                        self.set_dist(n.0, d);
                        q.push(Reverse(Self::PQE::init(n.0, dval)));
                    }
                }
            }
        }
    }
}

pub trait PriorityQueueElement: Ord + Copy + Sized {
    type Point;
    type Dist: Add<Output = Self::Dist>;

    fn init(_: Self::Point, _: Self::Dist) -> Self;
    fn point(&self) -> Self::Point;
    fn dist(&self) -> Self::Dist;
}

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
