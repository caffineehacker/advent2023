use clap::Parser;
use itertools::Itertools;
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

#[derive(Copy, Clone)]
struct Pipe {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Pipe {
    fn new(north: bool, east: bool, south: bool, west: bool) -> Self {
        return Pipe {
            north,
            east,
            south,
            west,
        };
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    // grid[y][x]. Up is negative, down positive
    let grid = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut pipe_directions = HashMap::new();
    pipe_directions.insert('|', Pipe::new(true, false, true, false));
    pipe_directions.insert('-', Pipe::new(false, true, false, true));
    pipe_directions.insert('L', Pipe::new(true, true, false, false));
    pipe_directions.insert('J', Pipe::new(true, false, false, true));
    pipe_directions.insert('7', Pipe::new(false, false, true, true));
    pipe_directions.insert('F', Pipe::new(false, true, true, false));
    pipe_directions.insert('.', Pipe::new(false, false, false, false));

    let start = get_start(&grid);

    let mut current_positions = Vec::new();
    if pipe_directions[&grid[start.0 - 1][start.1]].south {
        current_positions.push((start.0 - 1, start.1, start));
    }
    if pipe_directions[&grid[start.0 + 1][start.1]].north {
        current_positions.push((start.0 + 1, start.1, start));
    }
    if pipe_directions[&grid[start.0][start.1 - 1]].east {
        current_positions.push((start.0, start.1 - 1, start));
    }
    if pipe_directions[&grid[start.0][start.1 + 1]].west {
        current_positions.push((start.0, start.1 + 1, start));
    }

    let mut steps = 0;
    while !current_positions
        .iter()
        .any(|(y, x, _)| *y == start.0 && *x == start.1)
    {
        steps += 1;
        current_positions = current_positions
            .iter()
            .map(|(y, x, previous)| {
                let x = *x;
                let y = *y;
                let previous = *previous;
                let pipe = pipe_directions[&grid[y][x]];

                if pipe.north && previous.0 != y - 1 {
                    (y - 1, x, (y, x))
                } else if pipe.south && previous.0 != y + 1 {
                    (y + 1, x, (y, x))
                } else if pipe.east && previous.1 != x + 1 {
                    (y, x + 1, (y, x))
                } else {
                    (y, x - 1, (y, x))
                }
            })
            .collect_vec();
    }

    println!("Part 1: {}", (steps / 2) + 1);
}

fn get_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (y, x);
            }
        }
    }

    panic!("Can't find Start");
}
