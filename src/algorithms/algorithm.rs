use std::time::Instant;

use crate::{
    data::{MapPoint, MapRepresentation, Point},
    ProgramArgs,
};

pub trait Queue<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, node: T);
}

pub trait Coordinate {
    fn point(&self) -> &Point;
}

pub trait Algorithm<T: Coordinate, Q: Queue<T>> {
    fn create_queue(&self) -> Q;
    fn create_origin(&mut self, start: &Point) -> T;
    fn can_expand(&mut self, parent: &T, child: &MapPoint, target_point: &Point) -> Option<T>;
    fn build_solution(&mut self, node: &MapPoint) -> Vec<Point>;
    fn evaluate_cell(&mut self, _: &T) {}
}

pub struct AlgorithmExecutor;
impl AlgorithmExecutor {
    pub fn execute<T: Coordinate, Q: Queue<T>, A: Algorithm<T, Q>>(
        mut algo: A,
        args: &ProgramArgs,
        map: &MapRepresentation,
    ) -> AlgorithmResults {
        let mut cells_visited = 0;
        let execution_start = Instant::now();
        let mut queue = algo.create_queue();
        queue.push(algo.create_origin(&args.start));
        while let Some(node) = queue.pop() {
            cells_visited += 1;
            algo.evaluate_cell(&node);
            if node.point() == &args.end {
                let path = algo.build_solution(map.get(node.point()));
                return AlgorithmResults {
                    execution_ns: execution_start.elapsed().as_nanos(),
                    cells_visited,
                    path: Some(Path {
                        cost: path[0..path.len() - 1].iter().map(|x| map.get(x).distance_cost).sum(),
                        size: path.len(),
                        nodes: path
                    })
                };
            }
            for option in map.available_moves(node.point()) {
                if let Some(child) = algo.can_expand(&node, map.get(&option), &args.end) {
                    queue.push(child);
                }
            }
        }
        AlgorithmResults {
            execution_ns: execution_start.elapsed().as_nanos(),
            cells_visited,
            path: None
        }
    }
}

#[derive(Debug)]
pub struct AlgorithmResults {
    pub execution_ns: u128,
    pub cells_visited: usize,
    pub path: Option<Path>
}
#[derive(Debug)]
pub struct Path {
    pub nodes: Vec<Point>,
    pub size: usize,
    pub cost: usize
}