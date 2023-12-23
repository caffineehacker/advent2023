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

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let grid: HashMap<(usize, usize), char> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect();

    let mut to_process: Vec<((usize, usize), Vec<(usize, usize)>)> = Vec::new();
    to_process.push(((1, 0), vec![]));

    let mut longest_path = 0;
    while !to_process.is_empty() {
        let (current_position, current_path) = to_process.pop().unwrap();
        let current_char = *grid.get(&current_position).unwrap();

        if current_char == '#' {
            continue;
        }

        if current_position.1 == lines.len() - 1 && current_position.0 == lines[0].len() - 2 {
            if current_path.len() > longest_path {
                longest_path = current_path.len();
            }
            continue;
        }

        let mut new_path = current_path.clone();
        new_path.push(current_position);

        if (current_char == '>' || current_char == '.')
            && current_position.0 < lines[0].len() - 1
            && !current_path.contains(&(current_position.0 + 1, current_position.1))
        {
            to_process.push((
                (current_position.0 + 1, current_position.1),
                new_path.clone(),
            ));
        }
        if (current_char == '^' || current_char == '.')
            && current_position.1 > 0
            && !current_path.contains(&(current_position.0, current_position.1 - 1))
        {
            to_process.push((
                (current_position.0, current_position.1 - 1),
                new_path.clone(),
            ));
        }
        if (current_char == '<' || current_char == '.')
            && current_position.0 > 0
            && !current_path.contains(&(current_position.0 - 1, current_position.1))
        {
            to_process.push((
                (current_position.0 - 1, current_position.1),
                new_path.clone(),
            ));
        }
        if (current_char == 'v' || current_char == '.')
            && current_position.1 < lines.len() - 1
            && !current_path.contains(&(current_position.0, current_position.1 + 1))
        {
            to_process.push((
                (current_position.0, current_position.1 + 1),
                new_path.clone(),
            ));
        }
    }

    println!("Part 1: {}", longest_path);
}
