use crate::algorithms::Coordinate;

#[derive(Debug)]
pub struct OutputCell {
    pub has_right: bool,
    pub has_down: bool,
    pub cell_type: char,
}
impl OutputCell {
    pub fn new(c: char) -> OutputCell {
        OutputCell {
            has_right: false,
            has_down: false,
            cell_type: c,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Coordinate for Point {
    fn point(&self) -> &Point {
        self
    }
}

#[derive(Clone)]
pub struct MapPoint {
    pub point: Point,
    pub map_value: char,
    pub distance_cost: usize
}
impl MapPoint {
    pub fn new(from_char: char, x: usize, y: usize) -> MapPoint {
        let cost_val: usize = match from_char {
            'R' => 1,
            'f' => 2,
            'F' => 4,
            'h' => 5,
            'r' => 7,
            'M' => 10,
            'W' => 100000,
            _ => {
                println!("Unknown character {}, using cost 100000", from_char);
                100000
            }
        };
        MapPoint {
            point: Point { x, y },
            map_value: from_char,
            distance_cost: cost_val
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct WeightedPoint {
    pub point: Point,
    pub cost: usize
}
impl WeightedPoint {
    pub fn from_point(p: &Point, cost: usize) -> WeightedPoint {
        WeightedPoint { point: p.clone(), cost }
    }
}
impl Coordinate for WeightedPoint {
    fn point(&self) -> &Point {
        &self.point
    }
}
impl PartialEq for WeightedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
impl PartialOrd for WeightedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}
impl Ord for WeightedPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

#[derive(Debug, Clone, Eq)]
pub struct WeighetedHeuristicPoint {
    pub point: Point,
    pub path_cost: usize,
    pub sort_cost: usize
}
impl WeighetedHeuristicPoint {
    pub fn from_point(p: &Point, path_cost: usize, sort_cost: usize) -> WeighetedHeuristicPoint {
        WeighetedHeuristicPoint { point: p.clone(), path_cost, sort_cost }
    }
}
impl Coordinate for WeighetedHeuristicPoint {
    fn point(&self) -> &Point {
        &self.point
    }
}
impl PartialEq for WeighetedHeuristicPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
impl PartialOrd for WeighetedHeuristicPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.sort_cost.cmp(&other.sort_cost).reverse())
    }
}
impl Ord for WeighetedHeuristicPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_cost.cmp(&other.sort_cost).reverse()
    }
}

#[derive(Clone)]
pub struct HistoricalPoint {
    pub point: Point,
    pub cost: usize,
    pub prev: Option<Point>
}
impl Coordinate for HistoricalPoint {
    fn point(&self) -> &Point {
        &self.point
    }
}


pub struct ShortListPoint {
    pub point: Point,
    pub cost: usize,
    pub prev: Option<Point>,
}
impl ShortListPoint {
    pub fn from_point(point: &Point, distance: usize) -> ShortListPoint {
        ShortListPoint {
            point: point.clone(),
            cost: distance,
            prev: None,
        }
    }
}
impl Coordinate for ShortListPoint {
    fn point(&self) -> &Point {
        &self.point
    }
}