use super::Algorithm;
use std::collections::VecDeque;

use crate::data::{HistoricalPoint, MapPoint, Point};

use super::Queue;

pub struct IDDFSAlgorithm {
    pub stack: Vec<Point>,
    pub limit: usize,
    pub avoid_repeats: bool,
}
impl IDDFSAlgorithm {
    pub fn new(avoid_repeats: bool, limit: usize) -> IDDFSAlgorithm {
        IDDFSAlgorithm {
            stack: Vec::new(),
            limit,
            avoid_repeats,
        }
    }
}
impl Algorithm<HistoricalPoint, FILOQueue<HistoricalPoint>> for IDDFSAlgorithm {
    fn create_queue(&self) -> FILOQueue<HistoricalPoint> {
        FILOQueue::new()
    }

    fn create_origin(&mut self, start: &Point) -> HistoricalPoint {
        HistoricalPoint {
            point: start.clone(),
            cost: 0,
            prev: None,
        }
    }

    fn can_expand(
        &mut self,
        parent: &HistoricalPoint,
        child: &MapPoint,
        _: &Point,
    ) -> Option<HistoricalPoint> {
        let calc_cost = parent.cost + child.distance_cost;
        if calc_cost > self.limit {
            return None;
        }
        if self.avoid_repeats && self.stack.contains(&child.point) {
            return None;
        }
        Some(HistoricalPoint {
            point: child.point.clone(),
            cost: calc_cost,
            prev: Some(parent.point.clone()),
        })
    }

    fn build_solution(&mut self, _: &MapPoint) -> Vec<Point> {
        self.stack.clone()
    }

    fn evaluate_cell(&mut self, node: &HistoricalPoint) {
        if let Some(last) = &node.prev {
            while let Some(top) = self.stack.last() {
                if top == last {
                    break;
                } else {
                    self.stack.pop();
                }
            }
        }
        self.stack.push(node.point.clone());
    }
}

pub struct FILOQueue<T> {
    queue: VecDeque<T>,
}
impl<T> FILOQueue<T> {
    fn new() -> FILOQueue<T> {
        FILOQueue {
            queue: VecDeque::new(),
        }
    }
}
impl<T> Queue<T> for FILOQueue<T> {
    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn push(&mut self, node: T) {
        self.queue.push_front(node);
    }
}
