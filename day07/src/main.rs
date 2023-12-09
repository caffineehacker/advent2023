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
    #[arg(long)]
    wildj: bool,
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
                            if args.wildj {
                                1
                            } else {
                                11
                            }
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
            let wilds = card_count.get(&1).map(|w| *w).unwrap_or(0);
            let non_wild_counts = card_count
                .iter()
                .filter(|(card, _)| ***card != 1)
                .map(|(_, count)| *count)
                .counts();
            if non_wild_counts.keys().max().unwrap_or(&0) + wilds == 5 {
                // 5 of a kind
                (6, hand.clone(), bid)
            } else if non_wild_counts.keys().max().unwrap() + wilds == 4 {
                // 4 of a kind
                (5, hand.clone(), bid)
            } else if (non_wild_counts.contains_key(&3) && non_wild_counts.contains_key(&2))
                // Only need one wild since it would match with any other single card to make the pair
                || (wilds >= 1 && non_wild_counts.contains_key(&3))
                || (wilds >= 1 && *non_wild_counts.get(&2).unwrap_or(&0) == 2)
                || (wilds >= 2 && non_wild_counts.contains_key(&2))
            {
                // Full house
                (4, hand.clone(), bid)
            } else if non_wild_counts.keys().max().unwrap() + wilds == 3 {
                // 3 of a kind
                (3, hand.clone(), bid)
                // There is no way for a wild to make a two pair and not any hand better
            } else if *non_wild_counts.get(&2).unwrap_or(&0) == 2 {
                // Two pair
                (2, hand.clone(), bid)
            } else if non_wild_counts.contains_key(&2) || wilds >= 1 {
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
