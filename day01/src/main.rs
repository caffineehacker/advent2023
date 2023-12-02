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

    let values = lines
        .iter()
        .map(|line| {
            let filtered = line
                .chars()
                .filter(|c| *c >= '0' && *c <= '9')
                .collect_vec();
            let number = filtered[0].to_string() + &filtered[filtered.len() - 1].to_string();
            number.parse::<u32>().unwrap()
        })
        .collect_vec();

    let part1: u32 = values.iter().sum();
    println!("Part 1: {}", part1);

    let values = lines
        .iter()
        .map(|line| {
            let first_spelled_number = [
                line.find("one"),
                line.find("two"),
                line.find("three"),
                line.find("four"),
                line.find("five"),
                line.find("six"),
                line.find("seven"),
                line.find("eight"),
                line.find("nine"),
            ];
            let first_number = [
                line.find("1"),
                line.find("2"),
                line.find("3"),
                line.find("4"),
                line.find("5"),
                line.find("6"),
                line.find("7"),
                line.find("8"),
                line.find("9"),
            ];

            let last_spelled_number = [
                line.rfind("one"),
                line.rfind("two"),
                line.rfind("three"),
                line.rfind("four"),
                line.rfind("five"),
                line.rfind("six"),
                line.rfind("seven"),
                line.rfind("eight"),
                line.rfind("nine"),
            ];
            let last_number = [
                line.rfind("1"),
                line.rfind("2"),
                line.rfind("3"),
                line.rfind("4"),
                line.rfind("5"),
                line.rfind("6"),
                line.rfind("7"),
                line.rfind("8"),
                line.rfind("9"),
            ];

            let mut first_value = -1000;
            let mut first_index = 1000;
            let mut last_value = -1000;
            let mut last_index: i32 = -1;
            for i in 0..9 {
                let first_number = first_number.get(i).unwrap();
                if first_number.is_some() && first_number.unwrap() < first_index {
                    first_index = first_number.unwrap();
                    first_value = i as i32 + 1;
                }

                let first_spelled_number = first_spelled_number.get(i).unwrap();
                if first_spelled_number.is_some() && first_spelled_number.unwrap() < first_index {
                    first_index = first_spelled_number.unwrap();
                    first_value = i as i32 + 1;
                }

                let last_number = last_number.get(i).unwrap();
                if last_number.is_some() && last_number.unwrap() as i32 > last_index {
                    last_index = last_number.unwrap() as i32;
                    last_value = i as i32 + 1;
                }

                let last_spelled_number = last_spelled_number.get(i).unwrap();
                if last_spelled_number.is_some() && last_spelled_number.unwrap() as i32 > last_index
                {
                    last_index = last_spelled_number.unwrap() as i32;
                    last_value = i as i32 + 1;
                }
            }

            first_value * 10 + last_value
        })
        .collect_vec();

    let part2: i32 = values.iter().sum();
    println!("Part 2: {}", part2);
}
