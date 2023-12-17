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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
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

    let part1 = solve(&grid, 1, 3, args.debug);
    println!("Part 1: {}", part1.0);

    let part2 = solve(&grid, 4, 10, args.debug);
    println!("Part 2: {}", part2.0);
    println!("{:?}", part2.1);
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    x: isize,
    y: isize,
    direction: Direction,
    heat: i32,
    direction_count: i32,
    history: Vec<(isize, isize, Direction, i32, i32)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat.cmp(&other.heat)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.heat.partial_cmp(&other.heat) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.direction.partial_cmp(&other.direction)
    }
}

fn solve(
    grid: &HashMap<(isize, isize), i32>,
    min_step: i32,
    max_step: i32,
    debug: bool,
) -> (i32, Vec<(isize, isize, Direction, i32, i32)>) {
    let max_x = grid.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_y = grid.iter().map(|((_, y), _)| *y).max().unwrap();

    if debug {
        println!("Max x, y: {}, {}", max_x, max_y);
    }

    let mut to_process = sorted_vec::ReverseSortedVec::new();
    to_process.push(std::cmp::Reverse(State {
        x: 0,
        y: 0,
        heat: 0,
        direction: Direction::Right,
        direction_count: 0,
        history: Vec::new(),
    }));
    let mut states_seen = HashSet::new();

    while !to_process.is_empty() {
        let mut state = to_process.pop().unwrap().0;

        if states_seen.contains(&((state.x, state.y), state.direction, state.direction_count)) {
            continue;
        }
        states_seen.insert(((state.x, state.y), state.direction, state.direction_count));
        let mut history = state.history;
        history.push((
            state.x,
            state.y,
            state.direction,
            state.direction_count,
            state.heat,
        ));
        state.history = history.clone();

        if debug {
            println!("({}, {}): {}", state.x, state.y, state.heat);
        }

        if state.x == max_x && state.y == max_y && state.direction_count >= min_step {
            return (state.heat, history);
        }

        if (state.direction == Direction::Right && state.direction_count < max_step)
            || (state.direction != Direction::Left
                && state.direction != Direction::Right
                && state.direction_count >= min_step)
            || state.direction_count == 0
        {
            if state.x + 1 <= max_x {
                let mut new_state = state.clone();
                new_state.heat += grid.get(&(state.x + 1, state.y)).unwrap();
                new_state.x += 1;
                new_state.direction = Direction::Right;
                if state.direction == Direction::Right {
                    new_state.direction_count += 1
                } else {
                    new_state.direction_count = 1;
                }
                to_process.push(std::cmp::Reverse(new_state));
            }
        }
        if (state.direction == Direction::Left && state.direction_count < max_step)
            || (state.direction != Direction::Right
                && state.direction != Direction::Left
                && state.direction_count >= min_step)
            || state.direction_count == 0
        {
            if state.x - 1 >= 0 {
                let mut new_state = state.clone();
                new_state.heat += grid.get(&(state.x - 1, state.y)).unwrap();
                new_state.x -= 1;
                new_state.direction = Direction::Left;
                if state.direction == Direction::Left {
                    new_state.direction_count += 1
                } else {
                    new_state.direction_count = 1;
                }
                to_process.push(std::cmp::Reverse(new_state));
            }
        }
        if (state.direction == Direction::Up && state.direction_count < max_step)
            || (state.direction != Direction::Down
                && state.direction != Direction::Up
                && state.direction_count >= min_step)
            || state.direction_count == 0
        {
            if state.y - 1 >= 0 {
                let mut new_state = state.clone();
                new_state.heat += grid.get(&(state.x, state.y - 1)).unwrap();
                new_state.y -= 1;
                new_state.direction = Direction::Up;
                if state.direction == Direction::Up {
                    new_state.direction_count += 1
                } else {
                    new_state.direction_count = 1;
                }
                to_process.push(std::cmp::Reverse(new_state));
            }
        }
        if (state.direction == Direction::Down && state.direction_count < max_step)
            || (state.direction != Direction::Up
                && state.direction != Direction::Down
                && state.direction_count >= min_step)
            || state.direction_count == 0
        {
            if state.y + 1 <= max_y {
                let mut new_state = state.clone();
                new_state.heat += grid.get(&(state.x, state.y + 1)).unwrap();
                new_state.y += 1;
                new_state.direction = Direction::Down;
                if state.direction == Direction::Down {
                    new_state.direction_count += 1
                } else {
                    new_state.direction_count = 1;
                }
                to_process.push(std::cmp::Reverse(new_state));
            }
        }
    }

    panic!("Not solved");
}
