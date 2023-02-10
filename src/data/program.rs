use super::Point;

pub struct ProgramArgs {
    pub map_file_path: String,
    pub start: Point,
    pub end: Point,
    pub solve_flag: String,
    pub quiet: bool,
    pub metrics: bool
}