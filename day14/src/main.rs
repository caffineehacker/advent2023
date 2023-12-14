use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter,
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

    let grid = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Part 1 is just slide north, easiest to reverse the grid and do slide south
    let mut part1_grid = grid.clone();
    let mut part1 = 0;
    part1_grid.reverse();
    for y in 0..part1_grid.len() {
        for x in 0..part1_grid[0].len() {
            if part1_grid[y][x] == 'O' {
                if args.debug {
                    println!("Processing rock at {}, {}", x, y);
                }
                let mut additional_rocks = 0;
                let mut processed = false;
                for i in (y + 1)..part1_grid.len() {
                    if part1_grid[i][x] == 'O' {
                        additional_rocks += 1;
                    } else if part1_grid[i][x] == '#' {
                        if args.debug {
                            println!(
                                "{}, {} -> {}, {} with {} additional rocks",
                                x, y, x, i, additional_rocks
                            );
                        }
                        part1 += i - additional_rocks;
                        processed = true;
                        break;
                    }
                }

                if !processed {
                    if args.debug {
                        println!(
                            "{}, {} -> {}, {} with {} additional rocks",
                            x, y, x, 0, additional_rocks
                        );
                    }
                    part1 += part1_grid.len() - additional_rocks;
                }
            }
            if args.debug {
                println!("Score: {}", part1);
            }
        }
    }

    println!("Part 1: {}", part1);

    let mut grid = grid.clone();
    let mut history: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    let mut iteration = 0;
    'outer: while iteration < 1000000000 {
        if args.debug {
            println!("-----------------");
            println!("Loop {}", iteration);
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    print!("{}", grid[y][x]);
                }
                println!();
            }

            println!("Score: {}", score(&grid));
        }
        let mut grid_ref = &grid;
        let original_iteration = iteration;
        while history.contains_key(grid_ref) && iteration < 1000000000 {
            grid_ref = history.get(grid_ref).unwrap();
            if args.debug {
                println!("History match");
            }
            iteration += 1;

            if *grid_ref == grid {
                let loop_size = iteration - original_iteration;
                if args.debug {
                    println!("Loop detected of size {}", loop_size);
                }

                if loop_size == 0 {
                    break 'outer;
                }

                iteration += ((1000000000 - iteration) / loop_size) * loop_size + 1;
            }
        }
        grid = grid_ref.clone();

        let old_grid = grid.clone();
        slide_rocks(&mut grid);
        history.insert(old_grid, grid.clone());

        iteration += 1;
    }

    let part2 = score(&grid);
    println!("Part 2: {}", part2);
}

fn slide_rocks(grid: &mut Vec<Vec<char>>) {
    for x in 0..grid[0].len() {
        let mut slide_to_index = 0;
        let mut rocks = 0;
        let mut index = 0;

        while index <= grid.len() {
            if index < grid.len() && grid[index][x] == 'O' {
                rocks += 1;
                grid[index][x] = '.'
            } else if index == grid.len() || grid[index][x] == '#' {
                for i in 0..rocks {
                    grid[slide_to_index + i][x] = 'O';
                }
                rocks = 0;
                slide_to_index = index + 1;
            }

            index += 1;
        }
    }

    for y in 0..grid.len() {
        let mut slide_to_index = 0;
        let mut rocks = 0;
        let mut x = 0;

        while x <= grid[0].len() {
            if x < grid.len() && grid[y][x] == 'O' {
                rocks += 1;
                grid[y][x] = '.'
            } else if x == grid.len() || grid[y][x] == '#' {
                for i in 0..rocks {
                    grid[y][slide_to_index + i] = 'O';
                }
                rocks = 0;
                slide_to_index = x + 1;
            }

            x += 1;
        }
    }

    grid.reverse();
    for x in 0..grid[0].len() {
        let mut slide_to_index = 0;
        let mut rocks = 0;
        let mut index = 0;

        while index <= grid.len() {
            if index < grid.len() && grid[index][x] == 'O' {
                rocks += 1;
                grid[index][x] = '.'
            } else if index == grid.len() || grid[index][x] == '#' {
                for i in 0..rocks {
                    grid[slide_to_index + i][x] = 'O';
                }
                rocks = 0;
                slide_to_index = index + 1;
            }

            index += 1;
        }
    }
    grid.reverse();

    for y in 0..grid.len() {
        let mut slide_to_index = 0;
        let mut rocks = 0;
        let mut x = 0;

        // Make positive be west
        grid[y].reverse();
        while x <= grid[0].len() {
            if x < grid.len() && grid[y][x] == 'O' {
                rocks += 1;
                grid[y][x] = '.'
            } else if x == grid.len() || grid[y][x] == '#' {
                for i in 0..rocks {
                    grid[y][slide_to_index + i] = 'O';
                }
                rocks = 0;
                slide_to_index = x + 1;
            }

            x += 1;
        }
        grid[y].reverse();
    }
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                score += grid.len() - y;
            }
        }
    }

    return score;
}
