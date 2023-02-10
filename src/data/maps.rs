use std::{fs, process::{self, exit}};

use super::{MapPoint, Point, OutputCell};

pub struct MapRepresentation {
    pub x_size: usize,
    pub y_size: usize,
    pub points: Vec<MapPoint>,
}
impl MapRepresentation {
    pub fn new(from_file_path: &String) -> Option<MapRepresentation> {
        let opened_file = fs::read_to_string(from_file_path).ok()?;
        let lines = opened_file.lines().collect::<Vec<&str>>();
        let header_line_data = lines[0].split(' ').collect::<Vec<&str>>();
        // If this doesn't look like a valid header line, get out!
        if header_line_data.len() != 2 {
            return None;
        }
        let x_size: usize = header_line_data[0].parse().ok()?;
        let y_size: usize = header_line_data[1].parse().ok()?;

        let mut data = Vec::<MapPoint>::new();

        for (y, l) in lines[1..lines.len()].iter().enumerate() {
            let mut current_row_size = 0;
            for (x, c) in l.chars().enumerate() {
                current_row_size += 1;
                data.push(MapPoint::new(c, x, y));
            }
            if current_row_size != x_size {
                println!(
                    "Error while parsing. Declared row of size {} but got {}.",
                    x_size, current_row_size
                );
                process::exit(3);
            }
        }

        if lines.len() - 1 != y_size {
            println!(
                "Error while parsing. Declared columns of size {} but got {}.",
                y_size,
                lines.len() - 1
            );
            process::exit(3);
        }

        Some(MapRepresentation {
            x_size,
            y_size,
            points: data,
        })
    }

    pub fn get(&self, point: &Point) -> &MapPoint {
        &self.points[point.y * self.x_size + point.x]
    }

    pub fn available_moves(&self, point: &Point) -> Vec<Point> {
        let mut moves = Vec::<Point>::new();
        let x_plus_1 = Point {
            x: point.x + 1,
            y: point.y,
        };
        let y_plus_1 = Point {
            x: point.x,
            y: point.y + 1,
        };
        if point.x > 0
            && self
                .get(&Point {
                    x: point.x - 1,
                    y: point.y,
                })
                .map_value
                != 'W'
        {
            moves.push(Point {
                x: point.x - 1,
                y: point.y,
            })
        }
        if point.y > 0
            && self
                .get(&Point {
                    x: point.x,
                    y: point.y - 1,
                })
                .map_value
                != 'W'
        {
            moves.push(Point {
                x: point.x,
                y: point.y - 1,
            })
        }
        if point.x < &self.x_size - 1 && self.get(&x_plus_1).map_value != 'W' {
            moves.push(x_plus_1)
        }
        if point.y < &self.y_size - 1 && self.get(&y_plus_1).map_value != 'W' {
            moves.push(y_plus_1);
        }
        moves
    }

    pub fn output_string(self, path: &Vec<Point>) -> String {
        let mut output_str = String::new();
        let mut output_cells = self
            .points
            .into_iter()
            .map(|x| OutputCell::new(x.map_value))
            .collect::<Vec<_>>();
        if path.is_empty() {
            println!("Path is empty, no valid solution.");
            exit(1);
        }
        for (ix, curr) in path[1..path.len()].iter().enumerate() {
            let prev = &path[ix];
            if curr.x < prev.x {
                output_cells[curr.x + (self.x_size * curr.y)].has_right = true;
            } else if prev.x < curr.x {
                output_cells[prev.x + (self.x_size * prev.y)].has_right = true;
            } else if curr.y < prev.y {
                output_cells[curr.x + (self.x_size * curr.y)].has_down = true;
            } else if curr.y > prev.y {
                output_cells[prev.x + (self.x_size * prev.y)].has_down = true;
            }
        }
        let mut r_count = 0;
        let mut line1 = String::new();
        let mut line2 = String::new();
        for r in output_cells {
            r_count += 1;
            line1 += &r.cell_type.to_string();
            if r.has_right {
                line1 += " - ";
            } else {
                line1 += "   ";
            }
            if r.has_down {
                line2 += "|   ";
            } else {
                line2 += "    ";
            }
            if r_count == self.x_size {
                output_str += &line1;
                output_str += "\n";
                output_str += &line2;
                output_str += "\n";
                line1.clear();
                line2.clear();
                r_count = 0;
            }
        }
        output_str
    }
}
