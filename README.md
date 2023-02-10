# Pathology
Pathfinding code as an experiment for AI class built in Rust.

## Usage

- Build with `cargo build` or run with `cargo run -- (args)`
- Provide no arguments to get help on how to use Pathology.
- The `metrics.fish` can be run on any system that has the Fish shell installed to obtain a metrics CSV file.

## Abstract

Because exploring graphs and other 2D data structures in computer science can be a slow process, having a faster way than brute force to discover the distance and shortest path between two points is critical. This is such an important topic that there are many algorithms with which to solve the problem, each with a different set of advantages and disadvantages. However, the real trick to optimization is knowledge of the problem itself, and exploiting that knowledge to make assumptions. Individual measures of these assumptions are known as heuristics. This paper will examine the nuances of implementing heuristic algorithms and show how tradeoffs can be made to optimize for either speed or accuracy.