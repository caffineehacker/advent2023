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
    #[arg(long, default_value("2"))]
    expand_by: usize,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut galaxy_positions = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect_vec();

    let mut i = galaxy_positions.iter().map(|(x, _)| *x).max().unwrap() as isize - 1;
    while i >= 0 {
        if !galaxy_positions.iter().any(|(x, _)| *x == i as usize) {
            // Expand the column
            if args.debug {
                println!("Expanding column {}", i);
            }
            galaxy_positions
                .iter_mut()
                .filter(|(x, _)| *x > i as usize)
                .for_each(|(x, _)| *x += args.expand_by - 1);
        }

        i -= 1;
    }

    i = galaxy_positions.iter().map(|(_, y)| *y).max().unwrap() as isize - 1;
    while i >= 0 {
        if !galaxy_positions.iter().any(|(_, y)| *y == i as usize) {
            // Expand the row
            if args.debug {
                println!("Expanding row {}", i);
            }
            galaxy_positions
                .iter_mut()
                .filter(|(_, y)| *y > i as usize)
                .for_each(|(_, y)| *y += args.expand_by - 1);
        }

        i -= 1;
    }

    let part1_distances = galaxy_positions
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let (x1, y1) = galaxies[0];
            let (x2, y2) = galaxies[1];

            let distance = x1.abs_diff(*x2) + y1.abs_diff(*y2);

            if args.debug {
                println!("{}, {} <-> {}, {}: {}", x1, y1, x2, y2, distance);
            }

            distance
        })
        .collect_vec();

    if args.debug {
        println!("{:?}", part1_distances);
    }

    println!("Part 1: {}", part1_distances.iter().cloned().sum::<usize>());
}
