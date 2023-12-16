use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    f32::MAX_EXP,
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

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    let max_x = *grid.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *grid.iter().map(|((_, y), _)| y).max().unwrap();

    let mut energized = HashSet::new();
    let mut to_process = Vec::new();
    to_process.push((0, 0, Direction::Right));
    let mut processed = HashSet::new();

    while !to_process.is_empty() {
        let (x, y, direction) = to_process.pop().unwrap();
        if processed.contains(&(x, y, direction)) {
            continue;
        }
        if x < 0 || x > max_x || y < 0 || y > max_y {
            continue;
        }
        processed.insert((x, y, direction));
        energized.insert((x, y));

        let cell = *grid.get(&(x, y)).unwrap();

        match direction {
            Direction::Up => {
                if cell == '-' {
                    to_process.push((x - 1, y, Direction::Left));
                    to_process.push((x + 1, y, Direction::Right));
                } else if cell == '/' {
                    to_process.push((x + 1, y, Direction::Right));
                } else if cell == '\\' {
                    to_process.push((x - 1, y, Direction::Left));
                } else {
                    to_process.push((x, y - 1, Direction::Up));
                }
            }
            Direction::Down => {
                if cell == '-' {
                    to_process.push((x - 1, y, Direction::Left));
                    to_process.push((x + 1, y, Direction::Right));
                } else if cell == '/' {
                    to_process.push((x - 1, y, Direction::Left));
                } else if cell == '\\' {
                    to_process.push((x + 1, y, Direction::Right));
                } else {
                    to_process.push((x, y + 1, Direction::Down));
                }
            }
            Direction::Left => {
                if cell == '|' {
                    to_process.push((x, y + 1, Direction::Down));
                    to_process.push((x, y - 1, Direction::Up));
                } else if cell == '/' {
                    to_process.push((x, y + 1, Direction::Down));
                } else if cell == '\\' {
                    to_process.push((x, y - 1, Direction::Up));
                } else {
                    to_process.push((x - 1, y, Direction::Left));
                }
            }
            Direction::Right => {
                if cell == '|' {
                    to_process.push((x, y + 1, Direction::Down));
                    to_process.push((x, y - 1, Direction::Up));
                } else if cell == '/' {
                    to_process.push((x, y - 1, Direction::Up));
                } else if cell == '\\' {
                    to_process.push((x, y + 1, Direction::Down));
                } else {
                    to_process.push((x + 1, y, Direction::Right));
                }
            }
        };
    }

    let part1 = energized.len();
    println!("Part 1: {}", part1);

    let part2 = 0;
    println!("Part 2: {}", part2);
}
