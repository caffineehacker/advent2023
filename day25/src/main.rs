use clap::Parser;
use itertools::Itertools;
use petgraph::{
    algo::{self, DfsSpace},
    prelude::*,
};
use std::{
    collections::HashMap,
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

    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::new();

    for line in lines.iter() {
        let (left, right) = line.split_once(":").unwrap();
        if !node_map.contains_key(left) {
            node_map.insert(left.to_string(), graph.add_node(()));
        }

        for right in right.trim().split_ascii_whitespace() {
            if !node_map.contains_key(right) {
                node_map.insert(right.to_string(), graph.add_node(()));
            }

            graph.add_edge(
                *node_map.get(left).unwrap(),
                *node_map.get(right).unwrap(),
                (),
            );
        }
    }

    // let costs = node_map
    //     .iter()
    //     .map(|(key, ni)| {
    //         let cost = *algo::k_shortest_path(&graph, *ni, None, 1, |_| 1)
    //             .values()
    //             .max()
    //             .unwrap();
    //         (cost, key)
    //     })
    //     .sorted()
    //     .rev()
    //     .collect_vec();

    // print!("{:?}", costs);

    // let part1 = node_map
    //     .keys()
    //     .combinations(3)
    //     .map(|combos| {
    //         let mut graph = graph.clone();
    //         for node in combos.iter() {
    //             graph.remove_node(*node_map.get(*node).unwrap());
    //         }
    //         graph
    //     })
    //     .filter(|graph| algo::connected_components(graph) == 2)
    //     .map(|g| {
    //         let start = node_map.values().take(1).collect_vec()[0];
    //         let mut connected = 0;
    //         let mut space = DfsSpace::new(&g);
    //         for node in node_map.values() {
    //             if algo::has_path_connecting(&g, *start, *node, Some(&mut space)) {
    //                 connected += 1;
    //             }
    //         }

    //         connected * (node_map.len() - connected)
    //     })
    //     .take(1)
    //     .collect_vec()[0];
    let nodes_to_check = node_map
        .values()
        .combinations(2)
        .enumerate()
        .filter(|(index, _)| index % 1000 == 0)
        .map(|(_, key)| key.clone())
        .collect_vec();
    let path_edge_counts = nodes_to_check
        .iter()
        .flat_map(|nodes| {
            let path = algo::astar(
                &graph,
                *nodes[0],
                |finish| finish == *nodes[1],
                |e| 1,
                |_| 0,
            )
            .unwrap();
            path.1
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| graph.find_edge(a, b).unwrap())
        })
        .sorted()
        .dedup_with_count()
        .collect_vec();

    let removed = path_edge_counts.iter().max_by_key(|n| n.0).unwrap().1;
    if args.debug {
        let (a, b) = graph.edge_endpoints(removed).unwrap();
        let a_key = node_map
            .iter()
            .find(|(_, value)| **value == a)
            .unwrap()
            .0
            .clone();
        let b_key = node_map
            .iter()
            .find(|(_, value)| **value == b)
            .unwrap()
            .0
            .clone();

        println!("Removing {} <-> {}", a_key, b_key);
    }
    graph.remove_edge(removed);

    let nodes_to_check = node_map
        .values()
        .combinations(2)
        .enumerate()
        .filter(|(index, _)| index % 1000 == 0)
        .map(|(_, key)| key.clone())
        .collect_vec();
    let path_edge_counts = nodes_to_check
        .iter()
        .flat_map(|nodes| {
            let path = algo::astar(
                &graph,
                *nodes[0],
                |finish| finish == *nodes[1],
                |e| 1,
                |_| 0,
            );
            if path.is_none() {
                return vec![].into_iter();
            } else {
                return path
                    .unwrap()
                    .1
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| graph.find_edge(a, b).unwrap())
                    .collect_vec()
                    .into_iter();
            }
        })
        .sorted()
        .dedup_with_count()
        .collect_vec();

    let removed = path_edge_counts.iter().max_by_key(|n| n.0).unwrap().1;
    if args.debug {
        let (a, b) = graph.edge_endpoints(removed).unwrap();
        let a_key = node_map
            .iter()
            .find(|(_, value)| **value == a)
            .unwrap()
            .0
            .clone();
        let b_key = node_map
            .iter()
            .find(|(_, value)| **value == b)
            .unwrap()
            .0
            .clone();

        println!("Removing {} <-> {}", a_key, b_key);
    }
    graph.remove_edge(removed);

    let nodes_to_check = node_map
        .values()
        .combinations(2)
        .enumerate()
        .filter(|(index, _)| index % 1000 == 0)
        .map(|(_, key)| key.clone())
        .collect_vec();
    let path_edge_counts = nodes_to_check
        .iter()
        .flat_map(|nodes| {
            let path = algo::astar(
                &graph,
                *nodes[0],
                |finish| finish == *nodes[1],
                |e| 1,
                |_| 0,
            );
            if path.is_none() {
                return vec![].into_iter();
            } else {
                return path
                    .unwrap()
                    .1
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| graph.find_edge(a, b).unwrap())
                    .collect_vec()
                    .into_iter();
            }
        })
        .sorted()
        .dedup_with_count()
        .collect_vec();

    let removed = path_edge_counts.iter().max_by_key(|n| n.0).unwrap().1;
    if args.debug {
        let (a, b) = graph.edge_endpoints(removed).unwrap();
        let a_key = node_map
            .iter()
            .find(|(_, value)| **value == a)
            .unwrap()
            .0
            .clone();
        let b_key = node_map
            .iter()
            .find(|(_, value)| **value == b)
            .unwrap()
            .0
            .clone();

        println!("Removing {} <-> {}", a_key, b_key);
    }
    graph.remove_edge(removed);

    // Now we should hopefully have two parts

    let start = node_map.values().take(1).collect_vec()[0];
    let mut connected = 0;
    let mut space = DfsSpace::new(&graph);
    for node in node_map.values() {
        if algo::has_path_connecting(&graph, *start, *node, Some(&mut space)) {
            connected += 1;
        }
    }

    if args.debug {
        println!("Connected: {}", connected);
    }

    let part1 = connected * (node_map.len() - connected);
    //let part1 = "";

    println!("Part 1: {}", part1);
}
