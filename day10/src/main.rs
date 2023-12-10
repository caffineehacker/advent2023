use clap::Parser;
use itertools::Itertools;
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

    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    paths.resize(current_positions.len(), vec![start]);

    let mut steps = 0;
    while !current_positions
        .iter()
        .any(|(y, x, _)| *y == start.0 && *x == start.1)
    {
        for i in 0..current_positions.len() {
            paths[i].push((current_positions[i].0, current_positions[i].1));
        }

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

    let winning_path_index = current_positions
        .iter()
        .enumerate()
        .find(|(_, position)| position.0 == start.0 && position.1 == start.1)
        .unwrap()
        .0;
    let path = paths[winning_path_index].clone();

    if args.debug {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if path.contains(&(y, x)) {
                    //print!("{}", grid[y][x]);
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    // This gives me the information to know which side I care about
    println!("{:?} -> {:?}", path[0], path[1]);

    let mut covered: HashSet<(usize, usize)> = HashSet::new();
    // Let's cheat and assume right hand direction
    let mut direction = (
        path[path.len() - 1].0 as isize - path[0].0 as isize,
        path[path.len() - 1].1 as isize - path[0].1 as isize,
    );

    for i in 1..path.len() {
        direction = (
            path[i].0 as isize - path[i - 1].0 as isize,
            path[i].1 as isize - path[i - 1].1 as isize,
        );

        // Right hand rule
        if direction.0 == 1 {
            // Down
            flood_fill((path[i].0, path[i].1 - 1), &path, &grid, &mut covered);
            flood_fill(
                (path[i - 1].0, path[i - 1].1 - 1),
                &path,
                &grid,
                &mut covered,
            );
        } else if direction.0 == -1 {
            // Up
            flood_fill((path[i].0, path[i].1 + 1), &path, &grid, &mut covered);
            flood_fill(
                (path[i - 1].0, path[i - 1].1 + 1),
                &path,
                &grid,
                &mut covered,
            );
        } else if direction.1 == 1 {
            // Right
            flood_fill((path[i].0 + 1, path[i].1), &path, &grid, &mut covered);
            flood_fill(
                (path[i - 1].0 + 1, path[i - 1].1),
                &path,
                &grid,
                &mut covered,
            );
        } else if direction.1 == -1 {
            // Left
            flood_fill((path[i].0 - 1, path[i].1), &path, &grid, &mut covered);
            flood_fill(
                (path[i - 1].0 - 1, path[i - 1].1),
                &path,
                &grid,
                &mut covered,
            );
        }
    }

    if args.debug {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if path.contains(&(y, x)) {
                    // print!("{}", grid[y][x]);
                    print!("X");
                } else if covered.contains(&(y, x)) {
                    print!("!");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    println!("Part 2: {}", covered.len());
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

fn flood_fill(
    start: (usize, usize),
    path: &Vec<(usize, usize)>,
    grid: &Vec<Vec<char>>,
    checked: &mut HashSet<(usize, usize)>,
) {
    let mut to_check = VecDeque::new();
    to_check.push_back(start);

    while !to_check.is_empty() {
        let cell = to_check.pop_back().unwrap();
        if !checked.contains(&cell) && !path.contains(&cell) {
            checked.insert(cell.clone());
            if cell.0 < grid.len() - 1 {
                to_check.push_back((cell.0 + 1, cell.1));
            }
            if cell.0 > 0 {
                to_check.push_back((cell.0 - 1, cell.1));
            }
            if cell.1 < grid[0].len() - 1 {
                to_check.push_back((cell.0, cell.1 + 1));
            }
            if cell.1 > 0 {
                to_check.push_back((cell.0, cell.1 - 1));
            }
        }
    }
}
