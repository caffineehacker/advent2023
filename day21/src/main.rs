use clap::Parser;
use itertools::Itertools;
use sorted_vec::SortedVec;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

    let grid: HashMap<(isize, isize), char> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect_vec()
        })
        .collect();

    let start_point = *grid.iter().find(|cell| *cell.1 == 'S').unwrap().0;

    let mut visited_points = HashSet::new();
    let mut to_process = SortedVec::new();
    to_process.insert((0, start_point));
    let mut visited_states = HashSet::new();

    while !to_process.is_empty() {
        let (steps, position) = to_process.remove_index(0);
        if visited_states.contains(&(steps, position)) {
            continue;
        }
        visited_states.insert((steps, position));
        let grid_point = grid.get(&position);
        if grid_point.is_none() {
            continue;
        }
        let grid_point = grid_point.unwrap();
        if *grid_point == '#' {
            continue;
        }
        if steps == 64 {
            visited_points.insert(position);
            continue;
        }

        to_process.insert((steps + 1, (position.0 + 1, position.1)));
        to_process.insert((steps + 1, (position.0 - 1, position.1)));
        to_process.insert((steps + 1, (position.0, position.1 + 1)));
        to_process.insert((steps + 1, (position.0, position.1 - 1)));
    }

    if args.debug {
        println!("{:?}", visited_points);
    }

    println!("Part 1: {}", visited_points.len());
}
