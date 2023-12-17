use clap::Parser;
use itertools::Itertools;
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
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

    let grid: HashMap<(isize, isize), i32> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x as isize, y as isize),
                        c.to_string().parse::<i32>().unwrap(),
                    )
                })
                .collect_vec()
        })
        .collect();

    let max_x = *grid.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *grid.iter().map(|((_, y), _)| y).max().unwrap();

    let mut to_process = Vec::new();
    // Coordinates, heat, direction, direction count, history
    to_process.push(((0, 0), 0, Direction::Right, 0, Vec::new()));
    let mut states_seen = HashSet::new();

    while !to_process.is_empty() {
        to_process.sort_by_key(|state| state.1);
        let ((x, y), heat, direction, direction_count, history) = to_process.remove(0);
        if states_seen.contains(&((x, y), direction, direction_count)) {
            continue;
        }
        states_seen.insert(((x, y), direction, direction_count));
        let mut history = history;
        history.push((x, y, direction, direction_count, heat));

        if args.debug {
            println!("({}, {}): {}", x, y, heat);
        }

        if x == max_x && y == max_y {
            println!("Part 1: {}", heat);
            if args.debug {
                println!("Path: {:?}", history);
            }
            break;
        }

        if (direction != Direction::Right || direction_count < 3) && direction != Direction::Left {
            let mut new_heat = heat;
            if x + 1 <= max_x {
                new_heat += grid.get(&(x + 1, y)).unwrap();
                to_process.push((
                    (x + 1, y),
                    new_heat,
                    Direction::Right,
                    if direction == Direction::Right {
                        direction_count + 1
                    } else {
                        1
                    },
                    history.clone(),
                ));
            }
        }
        if (direction != Direction::Left || direction_count < 3) && direction != Direction::Right {
            let mut new_heat = heat;
            if x - 1 >= 0 {
                new_heat += grid.get(&(x - 1, y)).unwrap();
                to_process.push((
                    (x - 1, y),
                    new_heat,
                    Direction::Left,
                    if direction == Direction::Left {
                        direction_count + 1
                    } else {
                        1
                    },
                    history.clone(),
                ));
            }
        }
        if (direction != Direction::Up || direction_count < 3) && direction != Direction::Down {
            let mut new_heat = heat;
            if y - 1 >= 0 {
                new_heat += grid.get(&(x, y - 1)).unwrap();
                to_process.push((
                    (x, y - 1),
                    new_heat,
                    Direction::Up,
                    if direction == Direction::Up {
                        direction_count + 1
                    } else {
                        1
                    },
                    history.clone(),
                ));
            }
        }
        if (direction != Direction::Down || direction_count < 3) && direction != Direction::Up {
            let mut new_heat = heat;
            if y + 1 <= max_y {
                new_heat += grid.get(&(x, y + 1)).unwrap();
                to_process.push((
                    (x, y + 1),
                    new_heat,
                    Direction::Down,
                    if direction == Direction::Down {
                        direction_count + 1
                    } else {
                        1
                    },
                    history.clone(),
                ));
            }
        }
    }

    let part2 = 0;
    println!("Part 2: {}", part2);
}
