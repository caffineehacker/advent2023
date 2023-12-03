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

    let schematic = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut part1_total = 0;
    let mut gear_ratios = HashMap::new();

    for row in 0..schematic.len() {
        let current_row = schematic.get(row).unwrap();
        let mut column = 0;
        while column < current_row.len() {
            if current_row[column] >= '0' && current_row[column] <= '9' {
                let start_column = column;
                let mut char_count = 1;
                let mut value = (current_row[column] as u8 - '0' as u8) as u32;
                column += 1;
                'collecting: while column < current_row.len() {
                    if current_row[column] >= '0' && current_row[column] <= '9' {
                        value *= 10;
                        value += (current_row[column] as u8 - '0' as u8) as u32;
                        char_count += 1;
                    } else {
                        break 'collecting;
                    }
                    column += 1;
                }

                let mut is_near_symbol = false;
                'check: for check_row in
                    ((row as isize - 1).max(0) as usize)..(row + 2).min(schematic.len())
                {
                    for check_column in ((start_column as isize - 1).max(0) as usize)
                        ..(start_column + char_count + 1).min(current_row.len())
                    {
                        // if args.debug {
                        //     println!("Checking {}, {}", check_row, check_column);
                        // }
                        let check_value = schematic[check_row][check_column];
                        if !(check_value >= '0' && check_value <= '9') && check_value != '.' {
                            is_near_symbol = true;
                        }
                        if check_value == '*' {
                            if gear_ratios.contains_key(&(check_row, check_column)) {
                                let ratio: &mut (i32, u32) =
                                    gear_ratios.get_mut(&(check_row, check_column)).unwrap();
                                ratio.0 += 1;
                                ratio.1 *= value;
                            } else {
                                gear_ratios.insert((check_row, check_column), (1, value));
                            }
                        }
                    }
                }

                if is_near_symbol {
                    if args.debug {
                        println!("{}", value);
                    }
                    part1_total += value;
                }
            }
            column += 1;
        }
    }

    println!("Part 1: {}", part1_total);

    let part2 = gear_ratios
        .iter()
        .filter(|(_, (count, _))| *count == 2)
        .map(|(_, (_, value))| *value)
        .sum::<u32>();
    println!("Part 2: {}", part2);
}
