use crate::data::{MapPoint, MapRepresentation, Point, ShortListPoint};

use super::{Algorithm, Coordinate, Queue};

pub struct BreadthFirstAlgorithm {
    points: Vec<ShortListPoint>,
    x_size: usize,
}
impl BreadthFirstAlgorithm {
    pub fn new(map: &MapRepresentation) -> BreadthFirstAlgorithm {
        BreadthFirstAlgorithm {
            points: map
                .points
                .iter()
                .map(|x| ShortListPoint::from_point(&x.point, usize::MAX))
                .collect(),
            x_size: map.x_size,
        }
    }
    fn get(&self, point: &Point) -> &ShortListPoint {
        &self.points[(point.y * self.x_size) + point.x]
    }
    fn get_mut(&mut self, point: &Point) -> &mut ShortListPoint {
        &mut self.points[(point.y * self.x_size) + point.x]
    }
}
impl Algorithm<Point, FIFOQueue<Point>> for BreadthFirstAlgorithm {
    fn create_queue(&self) -> FIFOQueue<Point> {
        FIFOQueue::new()
    }

    fn create_origin(&mut self, start: &Point) -> Point {
    	self.get_mut(start).cost = 0;
        start.clone()
    }

    fn can_expand(&mut self, parent: &Point, child: &MapPoint, _: &Point) -> Option<Point> {
        if self.get(&child.point).prev.is_none() && self.get(&child.point).cost > 0 {
            self.get_mut(&child.point).prev = Some(parent.point().clone());
            Some(child.point.clone())
        } else {
            None
        }
    }

    fn build_solution(&mut self, node: &MapPoint) -> Vec<Point> {
        let mut solution = Vec::new();
        let mut prev = &Some(node.point.clone());
        while let Some(last) = prev {
            solution.push(last.clone());
            prev = &self.get(last).prev;
        }

        solution
    }
}

pub struct FIFOQueue<T> {
    queue: Vec<T>,
}
impl<T> FIFOQueue<T> {
    fn new() -> FIFOQueue<T> {
        FIFOQueue { queue: Vec::new() }
    }
}
impl<T> Queue<T> for FIFOQueue<T> {
    fn pop(&mut self) -> Option<T> {
        self.queue.pop()
    }
    fn push(&mut self, node: T) {
        self.queue.push(node);
    }
}
