use super::algorithm::Algorithm;
use crate::data::{MapPoint, MapRepresentation, Point, ShortListPoint, WeightedPoint};
use std::collections::BinaryHeap;

pub struct DjikstraAlgorithm {
    points: Vec<ShortListPoint>,
    x_size: usize,
}
impl DjikstraAlgorithm {
    pub fn new(from_map: &MapRepresentation) -> DjikstraAlgorithm {
        DjikstraAlgorithm {
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
impl Algorithm<WeightedPoint, BinaryHeap<WeightedPoint>> for DjikstraAlgorithm {
    fn create_queue(&self) -> BinaryHeap<WeightedPoint> {
        BinaryHeap::new()
    }
    fn create_origin(&mut self, start: &Point) -> WeightedPoint {
        self.points[(start.y * self.x_size) + start.x].cost = 0;
        WeightedPoint::from_point(start, 0)
    }
    fn can_expand(&mut self, parent: &WeightedPoint, child: &MapPoint, _: &Point) -> Option<WeightedPoint> {
        let calc_cost = parent.cost + child.distance_cost;
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
        let mut prev = &Some(node.point.clone());
        while let Some(last) = prev {
            solution.push(last.clone());
            prev = &self.get(last).prev;
        }

        solution
    }
}
