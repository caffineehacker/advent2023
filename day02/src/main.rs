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

    let games = lines
        .iter()
        .map(|line| {
            let (game, results) = line.split(":").collect_tuple().unwrap();
            let game = game
                .split_ascii_whitespace()
                .collect_vec()
                .get(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let pulls = results.split(";").collect_vec();
            (
                game,
                pulls
                    .iter()
                    .map(|pull| {
                        let colors = pull.split(",").map(|color| color.to_string()).collect_vec();
                        (
                            colors
                                .iter()
                                .find(|color| color.contains("red"))
                                .map(|red| {
                                    red.trim()
                                        .split(" ")
                                        .collect_vec()
                                        .get(0)
                                        .unwrap()
                                        .parse::<u32>()
                                        .unwrap()
                                })
                                .unwrap_or(0),
                            colors
                                .iter()
                                .find(|color| color.contains("green"))
                                .map(|green| {
                                    green
                                        .trim()
                                        .split(" ")
                                        .collect_vec()
                                        .get(0)
                                        .unwrap()
                                        .parse::<u32>()
                                        .unwrap()
                                })
                                .unwrap_or(0),
                            colors
                                .iter()
                                .find(|color| color.contains("blue"))
                                .map(|blue| {
                                    blue.trim()
                                        .split(" ")
                                        .collect_vec()
                                        .get(0)
                                        .unwrap()
                                        .parse::<u32>()
                                        .unwrap()
                                })
                                .unwrap_or(0),
                        )
                    })
                    .collect_vec(),
            )
        })
        .collect_vec();

    // for part 1, find games possible with only 12 red cubes, 13 green cubes, and 14 blue cubes
    let possible_games = games.iter().filter(|(_, pulls)| {
        pulls
            .iter()
            .all(|pull| pull.0 <= 12 && pull.1 <= 13 && pull.2 <= 14)
    });

    let part1: u32 = possible_games.map(|(game_number, _)| game_number).sum();
    println!("Part 1: {}", part1);
}
