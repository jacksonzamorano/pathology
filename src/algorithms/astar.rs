use std::{collections::BinaryHeap, env};

use crate::data::{MapPoint, MapRepresentation, Point, ShortListPoint, WeightedPoint, WeighetedHeuristicPoint};

use super::{Algorithm, Queue};

const K_WEIGHT: f64 = 2.0;

pub struct DistanceHeuristicAlgorithm {
    points: Vec<ShortListPoint>,
    x_size: usize,
}
impl DistanceHeuristicAlgorithm {
    pub fn new(from_map: &MapRepresentation) -> DistanceHeuristicAlgorithm {
        DistanceHeuristicAlgorithm {
            points: from_map
                .points
                .iter()
                .map(|x| ShortListPoint::from_point(&x.point, usize::MAX))
                .collect(),
            x_size: from_map.x_size,
        }
    }

    fn get_mut(&mut self, point: &Point) -> &mut ShortListPoint {
        &mut self.points[(point.y * self.x_size) + point.x]
    }
    fn get(&self, point: &Point) -> &ShortListPoint {
        &self.points[(point.y * self.x_size) + point.x]
    }
}
impl Algorithm<WeightedPoint, BinaryHeap<WeightedPoint>> for DistanceHeuristicAlgorithm {
    fn create_queue(&self) -> BinaryHeap<WeightedPoint> {
        BinaryHeap::new()
    }
    fn create_origin(&mut self, start: &Point) -> WeightedPoint {
        self.points[(start.y * self.x_size) + start.x].cost = 0;
        WeightedPoint::from_point(start, 0)
    }
    fn can_expand(
        &mut self,
        parent: &WeightedPoint,
        child: &MapPoint,
        target: &Point,
    ) -> Option<WeightedPoint> {
        let calc_cost = parent.cost
            + child.distance_cost
            + child.point.x.abs_diff(target.x)
            + child.point.y.abs_diff(target.y);
        if calc_cost < self.get(&child.point).cost
            && parent.cost <= self.get(&parent.point).cost
        {
            self.get_mut(&child.point).prev = Some(parent.point.clone());
            self.get_mut(&child.point).cost = calc_cost;
            Some(WeightedPoint::from_point(&child.point, calc_cost))
        } else {
            None
        }
    }
    fn build_solution(&mut self, node: &MapPoint) -> Vec<Point> {
        let mut solution = Vec::new();
        solution.push(node.point.clone());
        let mut prev = &Some(node.point.clone());
        while let Some(last) = prev {
            solution.push(last.clone());
            prev = &self.get(last).prev;
        }

        solution
    }
}

pub struct DistanceWeightHeuristicAlgorithm {
    points: Vec<ShortListPoint>,
    x_size: usize,
    weight: f64,
}
impl DistanceWeightHeuristicAlgorithm {
    pub fn new(from_map: &MapRepresentation) -> DistanceWeightHeuristicAlgorithm {
        DistanceWeightHeuristicAlgorithm {
            points: from_map
                .points
                .iter()
                .map(|x| ShortListPoint::from_point(&x.point, usize::MAX))
                .collect(),
            x_size: from_map.x_size,
            weight: env::args()
                .collect::<Vec<String>>()
                .last()
                .unwrap_or(&String::new())
                .parse()
                .unwrap_or(K_WEIGHT),
        }
    }

    fn get_mut(&mut self, point: &Point) -> &mut ShortListPoint {
        &mut self.points[(point.y * self.x_size) + point.x]
    }
    fn get(&self, point: &Point) -> &ShortListPoint {
        &self.points[(point.y * self.x_size) + point.x]
    }

}
impl Algorithm<WeighetedHeuristicPoint, BinaryHeap<WeighetedHeuristicPoint>> for DistanceWeightHeuristicAlgorithm {
    fn create_queue(&self) -> BinaryHeap<WeighetedHeuristicPoint> {
        BinaryHeap::new()
    }
    fn create_origin(&mut self, start: &Point) -> WeighetedHeuristicPoint {
        self.points[(start.y * self.x_size) + start.x].cost = 0;
        WeighetedHeuristicPoint::from_point(start, 0, 0)
    }
    fn can_expand(
        &mut self,
        parent: &WeighetedHeuristicPoint,
        child: &MapPoint,
        target: &Point,
    ) -> Option<WeighetedHeuristicPoint> {
        let x_diff = child.point.x.abs_diff(target.x) as f64;
        let y_diff = child.point.y.abs_diff(target.y) as f64;
        let mhtn_diff = (self.weight * (x_diff + y_diff)) as usize;

        let calc_cost = parent.path_cost + child.distance_cost + mhtn_diff;
        if calc_cost < self.get(&child.point).cost
            && parent.path_cost <= self.get(&parent.point).cost
        {
            self.get_mut(&child.point).prev = Some(parent.point.clone());
            self.get_mut(&child.point).cost = calc_cost;
            Some(WeighetedHeuristicPoint::from_point(&child.point, parent.path_cost + child.distance_cost, calc_cost))
        } else {
            None
        }
    }
    fn build_solution(&mut self, node: &MapPoint) -> Vec<Point> {
        let mut solution = Vec::new();
        solution.push(node.point.clone());
        let mut prev = &Some(node.point.clone());
        while let Some(last) = prev {
            solution.push(last.clone());
            prev = &self.get(last).prev;
        }

        solution
    }
}

impl<T: Ord> Queue<T> for BinaryHeap<T> {
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    fn push(&mut self, node: T) {
        self.push(node);
    }
}