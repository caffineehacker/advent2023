use clap::Parser;
use itertools::Itertools;
use multimap::MultiMap;
use petgraph::{algo, prelude::*};

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let grid: HashMap<(usize, usize), char> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect();

    part1(&grid, lines.len() - 1);
    part2(&grid, (lines[0].len() - 2, lines.len() - 1));
}

fn part1(grid: &HashMap<(usize, usize), char>, destination_y: usize) {
    let mut to_process: Vec<((usize, usize), HashSet<(usize, usize)>)> = Vec::new();
    to_process.push(((1, 0), HashSet::new()));

    let mut longest_path = 0;
    while !to_process.is_empty() {
        let (current_position, current_path) = to_process.pop().unwrap();
        let current_char = *grid.get(&current_position).unwrap();

        if current_char == '#' {
            continue;
        }

        if current_position.1 == destination_y {
            if current_path.len() > longest_path {
                longest_path = current_path.len();
            }
            continue;
        }

        let mut new_path = current_path.clone();
        new_path.insert(current_position);

        if (current_char == '>' || current_char == '.')
            && !current_path.contains(&(current_position.0 + 1, current_position.1))
        {
            to_process.push((
                (current_position.0 + 1, current_position.1),
                new_path.clone(),
            ));
        }
        if (current_char == '^' || current_char == '.')
            && current_position.1 > 0
            && !current_path.contains(&(current_position.0, current_position.1 - 1))
        {
            to_process.push((
                (current_position.0, current_position.1 - 1),
                new_path.clone(),
            ));
        }
        if (current_char == '<' || current_char == '.')
            && current_position.0 > 0
            && !current_path.contains(&(current_position.0 - 1, current_position.1))
        {
            to_process.push((
                (current_position.0 - 1, current_position.1),
                new_path.clone(),
            ));
        }
        if (current_char == 'v' || current_char == '.')
            && current_position.1 < destination_y
            && !current_path.contains(&(current_position.0, current_position.1 + 1))
        {
            to_process.push((
                (current_position.0, current_position.1 + 1),
                new_path.clone(),
            ));
        }
    }

    println!("Part 1: {}", longest_path);
}

fn part2(grid: &HashMap<(usize, usize), char>, destination: (usize, usize)) {
    // This is really a graph between decision points. So let's make a graph!
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    node_map.insert((1, 0), graph.add_node(1));
    // This is current_point, last_point, last decision point, distance since decision point
    let mut to_process = Vec::new();
    // Holds seen decision points
    let mut seen_points = HashSet::new();
    let mut decision_points: MultiMap<(usize, usize), ((usize, usize), usize)> = MultiMap::new();
    to_process.push(((1, 1), (1, 0), (1, 0), 1));
    seen_points.insert((1, 0));
    while !to_process.is_empty() {
        let (current_point, last_point, last_decision_point, distance) = to_process.pop().unwrap();

        // There are no decision points on the grid edge other than start / finish
        if current_point.1 == destination.1 {
            if !node_map.contains_key(&current_point) {
                node_map.insert(current_point, graph.add_node(1));
            }
            decision_points.insert(current_point, (last_decision_point, distance));
            graph.add_edge(
                *node_map.get(&last_decision_point).unwrap(),
                *node_map.get(&current_point).unwrap(),
                distance,
            );
            continue;
        }

        let mut open_destinations = Vec::new();
        if *grid.get(&(current_point.0 - 1, current_point.1)).unwrap() != '#' {
            open_destinations.push((current_point.0 - 1, current_point.1))
        }
        if *grid.get(&(current_point.0 + 1, current_point.1)).unwrap() != '#' {
            open_destinations.push((current_point.0 + 1, current_point.1))
        }
        if *grid.get(&(current_point.0, current_point.1 + 1)).unwrap() != '#' {
            open_destinations.push((current_point.0, current_point.1 + 1))
        }
        if current_point.1 > 0 && *grid.get(&(current_point.0, current_point.1 - 1)).unwrap() != '#'
        {
            open_destinations.push((current_point.0, current_point.1 - 1))
        }

        let mut new_distance = distance + 1;
        let mut new_last_decision_point = last_decision_point;
        if open_destinations.len() > 2 {
            if decision_points.contains_key(&current_point)
                && decision_points
                    .get_vec(&current_point)
                    .unwrap()
                    .contains(&(last_decision_point, distance))
            {
                // We've already processed this point / path
                continue;
            }
            if !node_map.contains_key(&current_point) {
                node_map.insert(current_point, graph.add_node(1));
            }
            graph.add_edge(
                *node_map.get(&last_decision_point).unwrap(),
                *node_map.get(&current_point).unwrap(),
                distance,
            );
            graph.add_edge(
                *node_map.get(&current_point).unwrap(),
                *node_map.get(&last_decision_point).unwrap(),
                distance,
            );
            decision_points.insert(current_point, (last_decision_point, distance));
            decision_points.insert(last_decision_point, (current_point, distance));

            new_distance = 1;
            new_last_decision_point = current_point;
        }

        for destination in open_destinations.into_iter() {
            if destination != last_point {
                to_process.push((
                    destination,
                    current_point,
                    new_last_decision_point,
                    new_distance,
                ));
            }
        }
    }

    let paths = algo::all_simple_paths::<Vec<_>, _>(
        &graph,
        *node_map.get(&(1, 0)).unwrap(),
        *node_map.get(&destination).unwrap(),
        1,
        None,
    )
    .collect_vec();

    let mut longest_path = 0;
    for path in paths.iter() {
        let cost = path
            .iter()
            .tuple_windows()
            .map(|(start, end)| {
                graph
                    .edge_weight(graph.find_edge(*start, *end).unwrap())
                    .unwrap()
            })
            .cloned()
            .sum();
        if cost > longest_path {
            longest_path = cost;
        }
    }

    println!("Part 2: {}", longest_path);
}
