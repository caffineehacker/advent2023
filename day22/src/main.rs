use clap::Parser;
use itertools::Itertools;
use sorted_vec::SortedVec;
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

    // Blocks will actually be (z, x, y) for better sorting
    let blocks = SortedVec::from_unsorted(
        lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                let (start, end) = line.split("~").collect_tuple().unwrap();
                let (sx, sy, sz) = start.split(",").collect_tuple().unwrap();
                let sx = sx.parse::<i64>().unwrap();
                let sy = sy.parse::<i64>().unwrap();
                let sz = sz.parse::<i64>().unwrap();
                let (ex, ey, ez) = end.split(",").collect_tuple().unwrap();
                let ex = ex.parse::<i64>().unwrap();
                let ey = ey.parse::<i64>().unwrap();
                let ez = ez.parse::<i64>().unwrap();

                if sz <= ez {
                    ((sz, sx, sy), (ez, ex, ey), index)
                } else {
                    ((ex, ex, ey), (sz, sx, sy), index)
                }
            })
            .collect_vec(),
    );

    // Since blocks are sorted we should be able to just walk from bottom to top and insert them in a final grid
    // We just project a downward shadow to see if they will hit any existing block.
    let mut settled_blocks: SortedVec<((i64, i64, i64), (i64, i64, i64), usize, Vec<usize>)> =
        SortedVec::new();
    for i in 0..blocks.len() {
        let mut block = blocks[i];

        let mut fully_settled = None;
        for s in (0..settled_blocks.len()).rev() {
            let settled_block = (settled_blocks[s].0, settled_blocks[s].1);
            if blocks_overlap_xy((block.0, block.1), (settled_block.0, settled_block.1)) {
                println!("Overlaps: {:?}, {:?}", settled_blocks[s], block);
                if fully_settled.is_none() {
                    block.1 .0 -= block.0 .0;
                    block.0 .0 = settled_block.0 .0 + 1;
                    block.1 .0 += block.0 .0;
                    fully_settled = Some((block.1, block.0, block.2, vec![settled_blocks[s].2]));
                } else if settled_block.0 .0 + 1 == fully_settled.as_ref().unwrap().1 .0 {
                    fully_settled.as_mut().unwrap().3.push(settled_blocks[s].2);
                } else {
                    println!(
                        "Breaking: {:?}, {:?}",
                        settled_blocks[s],
                        fully_settled.as_ref().unwrap()
                    );
                    break;
                }
                // We make the higher Z first since for settled blocks we need to look at the higher Z
            }
        }
        if fully_settled.is_some() {
            settled_blocks.insert(fully_settled.unwrap());
            continue;
        }

        // If we don't hit any blocks on the way down, then we're at the bottom.
        block.1 .0 -= block.0 .0;
        block.0 .0 = 1;
        block.1 .0 += block.0 .0;
        // We make the higher Z first since for settled blocks we need to look at the higher Z
        settled_blocks.insert((block.1, block.0, block.2, vec![]));
    }
    if args.debug {
        println!("{:?}", settled_blocks);
    }

    // How many blocks can be removed. Each block has a list of all of the blocks supporting it. Any block which only appears in lists with more than one block can be removed.
    let removable_blocks = (0..blocks.len())
        .filter(|i| {
            settled_blocks
                .iter()
                .all(|sb| !sb.3.contains(i) || sb.3.len() > 1)
        })
        .collect_vec();
    if args.debug {
        println!("{:?}", removable_blocks);
    }
    let part1 = removable_blocks.len();
    println!("Part 1: {}", part1);

    let blocks_supp = settled_blocks
        .iter()
        .map(|(_, _, block_id, supported_by)| (block_id, supported_by))
        .sorted()
        .collect_vec();
    let blocks_supported = &blocks_supp;

    let part2: usize = (0..blocks.len())
        .map(|index| {
            let (block_id, _) = blocks_supported[index];
            if removable_blocks
                .iter()
                .any(|removable| *removable == *block_id)
            {
                // If the block was removable then it didn't support anything.
                return 0;
            }
            if args.debug {
                println!("Processing block {}", block_id);
            }

            let mut dropped = vec![*block_id];
            loop {
                let mut blocks_shifted = false;
                for block in blocks_supported {
                    if !dropped.contains(block.0)
                        && !block.1.is_empty()
                        && block.1.iter().all(|s| dropped.contains(s))
                    {
                        blocks_shifted = true;
                        dropped.push(*block.0);
                    }
                }
                if !blocks_shifted {
                    break;
                }
            }

            dropped.len() - 1
        })
        .sum();

    println!("Part 2: {}", part2);
}

fn blocks_overlap_xy(
    block1: ((i64, i64, i64), (i64, i64, i64)),
    block2: ((i64, i64, i64), (i64, i64, i64)),
) -> bool {
    // Blocks overlap if one contains the other's X and Y

    // X
    let x_overlaps = if block1.0 .1 >= block2.0 .1.min(block2.1 .1)
        && block1.0 .1 <= block2.0 .1.max(block2.1 .1)
    {
        true
    } else if block1.1 .1 >= block2.0 .1.min(block2.1 .1)
        && block1.1 .1 <= block2.0 .1.max(block2.1 .1)
    {
        true
    } else if block2.0 .1 >= block1.0 .1.min(block1.1 .1)
        && block2.0 .1 <= block1.0 .1.max(block1.1 .1)
    {
        true
    } else if block2.1 .1 >= block1.0 .1.min(block1.1 .1)
        && block2.1 .1 <= block1.0 .1.max(block1.1 .1)
    {
        true
    } else {
        false
    };

    // Y
    let y_overlaps = if block1.0 .2 >= block2.0 .2.min(block2.1 .2)
        && block1.0 .2 <= block2.0 .2.max(block2.1 .2)
    {
        true
    } else if block1.1 .2 >= block2.0 .2.min(block2.1 .2)
        && block1.1 .2 <= block2.0 .2.max(block2.1 .2)
    {
        true
    } else if block2.0 .2 >= block1.0 .2.min(block1.1 .2)
        && block2.0 .2 <= block1.0 .2.max(block1.1 .2)
    {
        true
    } else if block2.1 .2 >= block1.0 .2.min(block1.1 .2)
        && block2.1 .2 <= block1.0 .2.max(block1.1 .2)
    {
        true
    } else {
        false
    };

    return x_overlaps && y_overlaps;
}
