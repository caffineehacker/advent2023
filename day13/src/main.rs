use clap::Parser;
use itertools::Itertools;
use std::{
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

    let mut grids = Vec::new();

    grids.push(Vec::new());
    for line in lines.iter() {
        if line.is_empty() {
            grids.push(Vec::new());
        } else {
            grids.last_mut().unwrap().push(line.chars().collect_vec());
        }
    }

    let part1 = grids
        .iter()
        .map(|grid| score_reflection(grid, args.debug))
        .sum::<usize>();

    println!("Part 1: {}", part1);
}

fn score_reflection(grid: &Vec<Vec<char>>, debug: bool) -> usize {
    if debug {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                print!("{}", grid[y][x]);
            }
            println!();
        }
    }
    // First horizontal since the data is already y, x
    for y in 1..grid.len() {
        let mut bottom_index = if y > grid.len() / 2 {
            grid.len() - 1
        } else {
            (y * 2) - 1
        };
        let mut top_index = if y <= grid.len() / 2 {
            0
        } else {
            y - (grid.len() - y)
        };

        if debug {
            println!(
                "Testing y = {} with top = {}, bottom = {}",
                y, top_index, bottom_index
            );
        }

        let mut found_mismatch = false;
        'top_bottom_loop: while top_index < bottom_index {
            for x in 0..grid[0].len() {
                if grid[top_index][x] != grid[bottom_index][x] {
                    found_mismatch = true;
                    break 'top_bottom_loop;
                }
            }
            top_index += 1;
            bottom_index -= 1;
        }

        if !found_mismatch {
            if debug {
                println!("y == {}", y);
            }
            return y * 100;
        }
    }

    for x in 1..grid[0].len() {
        let mut left_index = if x <= grid[0].len() / 2 {
            0
        } else {
            x - (grid[0].len() - x)
        };
        let mut right_index = if x > grid[0].len() / 2 {
            grid[0].len() - 1
        } else {
            (x * 2) - 1
        };

        if debug {
            println!(
                "Testing x = {} with left = {}, right = {}",
                x, left_index, right_index
            );
        }

        let mut found_mismatch = false;
        'left_right_loop: while left_index < right_index {
            for y in 0..grid.len() {
                if grid[y][left_index] != grid[y][right_index] {
                    found_mismatch = true;
                    break 'left_right_loop;
                }
            }
            left_index += 1;
            right_index -= 1;
        }

        if !found_mismatch {
            if debug {
                println!("x == {}", x);
            }
            return x;
        }
    }

    //panic!("No reflection found!");
    println!("No reflection?");
    0
}
