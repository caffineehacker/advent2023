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

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let instructions = lines
        .iter()
        .map(|line| {
            let (dir, count, color) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let color = color.trim_start_matches("(").trim_end_matches(")");
            return (dir, count.parse::<i64>().unwrap(), color);
        })
        .collect_vec();

    println!(
        "Part 1: {}",
        solve(
            instructions
                .iter()
                .map(|(dir, count, _)| (*dir, *count))
                .collect_vec(),
            args.debug
        )
    );

    let instructions = instructions
        .iter()
        .map(|(_, _, color)| {
            let color = color.trim_start_matches("#");
            let dir = color.chars().last().unwrap();
            let dir = match dir {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!("Unexpected direction code"),
            };

            let count = i64::from_str_radix(color.get(0..(color.len() - 1)).unwrap(), 16).unwrap();

            (dir, count)
        })
        .collect_vec();
    println!("Part 2: {}", solve(instructions, args.debug));
}

fn solve(instructions: Vec<(&str, i64)>, debug: bool) -> isize {
    let mut x = 0;
    let mut y = 0;

    let mut dug = HashMap::new();

    let mut previous_direction = (0, 0);
    for instruction in instructions.iter() {
        let direction: (isize, isize) = match instruction.0 {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Unexpected direction"),
        };

        for _ in 0..instruction.1 {
            dug.insert((x, y), (previous_direction, direction));
            x += direction.0;
            y += direction.1;
            previous_direction = direction;
        }
    }

    // We need to fill in the previous direction for the start
    if dug.contains_key(&(-1, 0)) {
        let potential = dug.get(&(-1, 0)).unwrap();
        if potential.1 .0 == 1 {
            dug.get_mut(&(0, 0)).unwrap().0 = potential.1;
        }
    }
    if dug.contains_key(&(1, 0)) {
        let potential = dug.get(&(1, 0)).unwrap();
        if potential.1 .0 == -1 {
            dug.get_mut(&(0, 0)).unwrap().0 = potential.1;
        }
    }
    if dug.contains_key(&(0, -1)) {
        let potential = dug.get(&(0, -1)).unwrap();
        if potential.1 .1 == 1 {
            dug.get_mut(&(0, 0)).unwrap().0 = potential.1;
        }
    }
    if dug.contains_key(&(0, 1)) {
        let potential = dug.get(&(0, 1)).unwrap();
        if potential.1 .1 == -1 {
            dug.get_mut(&(0, 0)).unwrap().0 = potential.1;
        }
    }

    if debug {
        println!("Trench: {}", dug.len());
    }
    let max_y = dug.keys().map(|k| k.1).max().unwrap();
    let min_y = dug.keys().map(|k| k.1).min().unwrap();

    let mut row_cache = HashMap::new();
    let mut solution = 0;
    let mut y = min_y;
    while y <= max_y {
        let in_row = dug
            .iter()
            .filter(|((_, dy), _)| y == *dy)
            .sorted_by_key(|((x, _), _)| *x)
            .collect_vec();

        let cache_key = in_row
            .iter()
            .map(|((x, _), directions)| (*x, **directions))
            .collect_vec();

        if row_cache.contains_key(&cache_key) {
            solution += row_cache.get(&cache_key).unwrap();
            if debug {
                println!("Cache hit");
            }
            y += 1;
            continue;
        }

        let mut inside = false;
        let mut i = 0;
        let mut row_score = 0;
        while i < in_row.len() {
            if debug {
                println!("i: {}, {:?}, inside: {}", i, in_row[i], inside);
            }
            let cell = in_row[i];
            if cell.1 .0 .1 != 0 && cell.1 .1 .1 != 0 {
                // Vertical piece, we just count to the next piece no mater what it is
                inside = !inside;
                if inside == true {
                    row_score += in_row[i + 1].0 .0 - cell.0 .0;
                } else {
                    row_score += 1;
                }
                i += 1;
            } else if cell.1 .0 .1 != 0 && cell.1 .1 .0 != 0 {
                // Turn from above or below
                // We keep going until we get to a turn
                i += 1;
                row_score += 1;
                while i + 1 < in_row.len() && in_row[i].0 .0 + 1 == in_row[i + 1].0 .0 {
                    // Skip all of the consecutive pieces
                    row_score += 1;
                    i += 1;
                }

                // If the turn comes from the same direction then we are still inside / out, if from the opposite then we are switching
                // We negate because one is a previous direction and one is next
                if in_row[i].1 .1 .1 != -cell.1 .0 .1 {
                    inside = !inside;
                }
                if inside {
                    row_score += in_row[i + 1].0 .0 - in_row[i].0 .0 - 1;
                }
                row_score += 1;
                i += 1;
            } else if cell.1 .0 .0 != 0 && cell.1 .1 .1 != 0 {
                // Turn to above or below
                // We keep going until we get to a turn
                i += 1;
                row_score += 1;
                while i + 1 < in_row.len() && in_row[i].0 .0 + 1 == in_row[i + 1].0 .0 {
                    // Skip all of the consecutive pieces
                    row_score += 1;
                    i += 1;
                }

                // If the turn comes from the same direction then we are still inside / out, if from the opposite then we are switching
                // We negate because one is a previous direction and one is next
                if in_row[i].1 .0 .1 != -cell.1 .1 .1 {
                    inside = !inside;
                }
                if inside {
                    row_score += in_row[i + 1].0 .0 - in_row[i].0 .0 - 1;
                }
                row_score += 1;
                i += 1;
            }
        }
        row_cache.insert(cache_key, row_score);
        solution += row_score;
        if debug {
            println!("y: {}, solution: {}", y, solution);
        }

        y += 1;
    }

    return solution;
}

// Notes for tomorrow: We need to just record the lengths of everything and can key them to the min x and min y value to make everything easier
// Then we just carry previously seen vertical pieces with us as we walk down the layers and drop them once they are past their length
