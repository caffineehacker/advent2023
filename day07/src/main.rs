use clap::Parser;
use itertools::Itertools;
use std::{
    cmp::Ordering,
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

    let hands_bids = lines
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
            (
                hand.chars()
                    .map(|c| {
                        if c == 'A' {
                            14
                        } else if c == 'K' {
                            13
                        } else if c == 'Q' {
                            12
                        } else if c == 'J' {
                            11
                        } else if c == 'T' {
                            10
                        } else {
                            c.to_string().parse::<u32>().unwrap()
                        }
                    })
                    .collect_vec(),
                bid.parse::<u32>().unwrap(),
            )
        })
        .collect_vec();

    let type_hand_bids = hands_bids
        .iter()
        .map(|(hand, bid)| {
            let card_count = hand.iter().counts();
            if card_count.len() == 1 {
                // 5 of a kind
                (6, hand.clone(), bid)
            } else if card_count.iter().any(|(_, count)| *count == 4) {
                // 4 of a kind
                (5, hand.clone(), bid)
            } else if card_count.iter().any(|(_, count)| *count == 3)
                && card_count.iter().any(|(_, count)| *count == 2)
            {
                // Full house
                (4, hand.clone(), bid)
            } else if card_count.iter().any(|(_, count)| *count == 3) {
                // 3 of a kind
                (3, hand.clone(), bid)
            } else if card_count.iter().filter(|(_, count)| **count == 2).count() == 2 {
                // Two pair
                (2, hand.clone(), bid)
            } else if card_count.iter().any(|(_, count)| *count == 2) {
                // One pair
                (1, hand.clone(), bid)
            } else {
                // High card
                (0, hand.clone(), bid)
            }
        })
        .sorted_by(|a, b| {
            let mut ordering = a.0.cmp(&b.0);
            if ordering == Ordering::Equal {
                for i in 0..5 {
                    ordering = a.1[i].cmp(&b.1[i]);
                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
            }
            return ordering;
        })
        .collect_vec();

    let mut part1 = 0;
    for i in 0..type_hand_bids.len() {
        if args.debug {
            println!(
                "{}: {:?} = {}",
                i,
                type_hand_bids[i].1,
                (i as u32 + 1) * type_hand_bids[i].2
            )
        }
        part1 += (i as u32 + 1) as u32 * type_hand_bids[i].2;
    }

    println!("Part 1: {}", part1);
}
