use std::{
    env::{self, args},
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
};

use algorithms::{
    AlgorithmExecutor, AlgorithmResults, BreadthFirstAlgorithm, DistanceHeuristicAlgorithm,
    DistanceWeightHeuristicAlgorithm, DjikstraAlgorithm, IDDFSAlgorithm,
};
use data::{MapRepresentation, Point, ProgramArgs};

mod algorithms;
mod data;

const K_DEEPENING_START_VALUE: usize = 20;
const K_DEEPENING_IC_VALUE: usize = 20;
const K_DEEPENING_MAX_VALUE: usize = 200;

fn output_path(map: MapRepresentation, results: &AlgorithmResults) {
    println!(
        "{}",
        map.output_string(&results.path.as_ref().unwrap().nodes)
    );
    println!(
        "Time: {}ns = {:.5}s",
        results.execution_ns,
        ((results.execution_ns as f64) / 1000000000.0)
    );
    println!("Visited states: {}", results.cells_visited);
    println!("Path cost: {}", results.path.as_ref().unwrap().cost);
    println!("Path size: {}", results.path.as_ref().unwrap().size);
}
fn write_metrics_csv(map: &MapRepresentation, args: &ProgramArgs, result: &AlgorithmResults) {
    let header =
        "Map Size,Nanoseconds,Start - End Difference,Evaluated Cells,Path Distance,Path Cost,Algorithm,Algorithm Weight\n";
    let algo = match args.solve_flag.as_str() {
        "-b" => "Breadth",
        "-l" => "Djikstra",
        "-ia" => "Iterative Deepening (AD)",
        "-i" => "Iterative Deepening (KD)",
        "-a1" => "A* — Cost + Distance",
        "-a2" => "A* — Cost + Weighted Distance",
        _ => "Unknown",
    };
    let output_path = Path::new("Metrics.csv");
    if !output_path.exists() {
        _ = fs::write(output_path, header);
    }
    if fs::read_to_string(output_path).unwrap().len() < header.len() {
        _ = fs::write(output_path, header);
    }
    let metrics = [
        &(map.x_size * map.y_size).to_string(),
        &result.execution_ns.to_string(),
        &(args.end.x.abs_diff(args.start.x) + args.end.y.abs_diff(args.start.y)).to_string(),
        &result.cells_visited.to_string(),
        &result.path.as_ref().unwrap().size.to_string(),
        &result.path.as_ref().unwrap().cost.to_string(),
        algo,
        &if args.solve_flag == "-a2" {
            env::args()
                .collect::<Vec<String>>()
                .last()
                .unwrap_or(&String::new())
                .parse()
                .unwrap_or(2.0)
        } else {
            0.0
        }
        .to_string(),
    ];
    let mut output = metrics.join(",");
    output += "\n";
    _ = File::options()
        .append(true)
        .open(output_path)
        .unwrap()
        .write(output.as_bytes());
}

fn main() {
    if let Some((params, map, result)) = evaluate() {
        if result.path.is_some() {
            if params.metrics {
                write_metrics_csv(&map, &params, &result);
            }
            if !params.quiet {
                output_path(map, &result);
            }
        } else {
            println!("No path found!")
        }
    } else {
        // Args are not correct, send message and exit
        command_line_help();
        exit(-1);
    }
}

fn evaluate() -> Option<(ProgramArgs, MapRepresentation, AlgorithmResults)> {
    let params = parse_args()?;
    let map = MapRepresentation::new(&params.map_file_path)?;
    let path = match params.solve_flag.as_str() {
        "-l" => AlgorithmExecutor::execute(DjikstraAlgorithm::new(&map), &params, &map),
        "-b" => AlgorithmExecutor::execute(BreadthFirstAlgorithm::new(&map), &params, &map),
        "-i" => iterative_deepening(false, &params, &map),
        "-ia" => iterative_deepening(true, &params, &map),
        "-a1" => AlgorithmExecutor::execute(DistanceHeuristicAlgorithm::new(&map), &params, &map),
        "-a2" => {
            AlgorithmExecutor::execute(DistanceWeightHeuristicAlgorithm::new(&map), &params, &map)
        }
        _ => panic!("invalid algorithm"),
    };
    Some((params, map, path))
}

fn iterative_deepening(
    avoid_repeat: bool,
    params: &ProgramArgs,
    map: &MapRepresentation,
) -> AlgorithmResults {
    let mut result = AlgorithmResults {
        execution_ns: 0,
        cells_visited: 0,
        path: None,
    };
    let mut depth = K_DEEPENING_START_VALUE;
    loop {
        let iteration =
            AlgorithmExecutor::execute(IDDFSAlgorithm::new(avoid_repeat, depth), params, map);
        result.execution_ns += iteration.execution_ns;
        result.cells_visited += iteration.cells_visited;
        println!(
            "Evaluated {} cells at depth {}",
            iteration.cells_visited, depth
        );
        if iteration.path.is_some() {
            result.path = iteration.path;
            return result;
        }
        depth += K_DEEPENING_IC_VALUE;
        println!("Deepening to {}", depth);
        if depth > K_DEEPENING_MAX_VALUE {
            return result;
        }
    }
}

fn parse_args() -> Option<ProgramArgs> {
    let mut arg_vec = args().collect::<Vec<String>>();
    arg_vec = arg_vec[1..arg_vec.len()].to_vec();

    if arg_vec.len() < 6 {
        None
    } else {
        Some(ProgramArgs {
            map_file_path: arg_vec[0].to_string(),
            start: Point {
                x: arg_vec[1].parse().ok()?,
                y: arg_vec[2].parse().ok()?,
            },
            end: Point {
                x: arg_vec[3].parse().ok()?,
                y: arg_vec[4].parse().ok()?,
            },
            solve_flag: arg_vec[5].to_string(),
            quiet: arg_vec.contains(&"--quiet".to_string()),
            metrics: arg_vec.contains(&"--metrics".to_string()),
        })
    }
}

fn command_line_help() {
    println!(
        "\nUsage: pathology [path to map] [start x] [start y] [goal x] [goal y] [-b/-l/-i/-ia/-a1/-a2] (--quiet [optional]) (--metrics [optional])\n"
    );
    println!("[path to map]: Map file. Must have a header with x size and y size.");
    println!("[start x] [start y]: Coordinates to begin from. Must be > 0 and < bounds of map.");
    println!("[end x] [end y]: Coordinates to navigate to. Must be > 0 and < bounds of map.");
    println!("[-b/-l/-i/-ia/-a1/-a2]: choice of algorithms.");
    println!("\t-b: Breadth-first search.");
    println!("\t-l: Derivation of Djikstra's algorithm.");
    println!("\t-i: Iterative deepening approach that allows for repeated states. Takes forever.");
    println!("\t-ia: Iterative deepening approach that prevents repeated states.");
    println!("\t-a1: A* algorithm that uses the heuristic of Manhattan distance.");
    println!("\t-a2: A* algorithm that uses a weighted heuristic of Manhattan distance.");
    println!("\t\t(optional) specify a float weight to use as the last argument. Defaults to 2 otherwise.");
    println!("[--quiet]: do not print any output to console.");
    println!("[--metrics]: append metrics to Metrics.csv in current directory.");
}
