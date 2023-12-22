use clap::Parser;
use itertools::Itertools;
use sorted_vec::SortedVec;
use std::{
    collections::{HashMap, HashSet},
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

    let grid: HashMap<(isize, isize), char> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect_vec()
        })
        .collect();

    let start_point = *grid.iter().find(|cell| *cell.1 == 'S').unwrap().0;

    let visited_points = get_visited_points(&grid, lines.len() as isize, start_point, 64, false);

    println!("Part 1: {}", visited_points.len());

    // For Part 2
    // We need to know how many parallel universes we can visit since we can cover every square in every universe until we are down to the last 100 or so steps.
    // First find the fastest way to each perimeter cell from center. Note that the outside cells are all open so we just need the fastest way into a universe.
    // This means we can treat universes as a step and instead figure out how many universes we can visit.
    // 26501365 is an odd number...
    // Universe is 131 cells across and 131 tall
    // 26501365 / 131 gives approximately 202300 which means we can travel 202300 universes in any direction and reach a center with 65 steps left. That's really close to the part one 64 steps and is suspicious...
    // The universe is an odd number of squares wide so we'll switch parity each grid
    // Let's start by figuring out how many squares can be covered for even and odd parity. We'll do that by calling the part 1 code with a large enough number to be sure we'll cover everything.
    let even = get_visited_points(&grid, lines.len() as isize, start_point, 1000, false).len();
    let odd = get_visited_points(&grid, lines.len() as isize, start_point, 1001, false).len();

    println!("Odd: {}, Even: {}", odd, even);

    // So is it valid to say we have a diamond 202300 * 2 wide and 202300 * 2 tall which encompases both even and odd squares.

    // Lets try math on the example
    let universe_width = 131;
    let steps = 26501365;
    let universe_radius = ((steps as f64) / universe_width as f64).floor();
    println!("Radius: {}", universe_radius);
    let universe_count =
        ((universe_radius as f64 * 2.0) * (universe_radius as f64 * 2.0) / 2.0).floor();
    println!("Universe count: {}", universe_count);
    let empty_cells_in_universe = grid.iter().filter(|entry| *entry.1 != '#').count();
    println!("Empty cells in universe: {}", empty_cells_in_universe);
    let upper_bound = empty_cells_in_universe as f64 * universe_count;
    println!("Upper bound: {}", upper_bound);

    // It looks like the diagonal cut across the input makes it so we can be pretty safe about the limit cutting a path through there

    // I think this means we have 202300+ total squares including some partial squares.
    // We get 65 steps left over at the edge so we can only reach places within 65 steps
    // So the corners would be the amount with 65 steps starting from a corner
    // Technically I should figure out from each corner but they come out the same
    let odd_corner =
        odd - get_visited_points(&grid, lines.len() as isize, start_point, 65, false).len();
    let even_corner = get_visited_points(&grid, lines.len() as isize, (0, 0), 64, false).len();
    println!("Corners, odd: {}, even: {}", odd_corner, even_corner);
    let upper_bound = (202301 * 202301) * odd + (202300 * 202300) * even;
    let missing_partial = 202301 * odd_corner;
    let added_corners = 202300 * even_corner * 2;
    let part2 = upper_bound - missing_partial + added_corners;
    println!("Part 2: {}", part2);

    // Never mind all that, my numbers are way too big, let's print out a few numbers and using a solver
    let step65 = get_visited_points(&grid, lines.len() as isize, start_point, 65, true).len();
    let step196 = get_visited_points(&grid, lines.len() as isize, start_point, 196, true).len();
    let step327 = get_visited_points(&grid, lines.len() as isize, start_point, 327, true).len();
    let step458 = get_visited_points(&grid, lines.len() as isize, start_point, 458, true).len();

    println!(
        "Use a polynomial solver: 65 = {}, 196 = {}, 327 = {}, 458 = {}",
        step65, step196, step327, step458
    );
}

fn get_visited_points(
    grid: &HashMap<(isize, isize), char>,
    grid_size: isize,
    start_point: (isize, isize),
    target_steps: isize,
    allow_loop: bool,
) -> HashSet<(isize, isize)> {
    let mut visited_points = HashSet::new();
    let mut to_process = SortedVec::new();
    to_process.insert((0, start_point));

    while !to_process.is_empty() {
        let (steps, position) = to_process.remove_index(0);
        if visited_points.contains(&position) {
            continue;
        }
        if steps % 2 == target_steps % 2 {
            visited_points.insert(position);
        }
        let effective_position = if allow_loop {
            (
                position.0.rem_euclid(grid_size),
                position.1.rem_euclid(grid_size),
            )
        } else {
            position
        };
        let grid_point = grid.get(&effective_position);
        if grid_point.is_none() {
            continue;
        }
        let grid_point = grid_point.unwrap();
        if *grid_point == '#' {
            continue;
        }
        if steps == target_steps {
            visited_points.insert(position);
            continue;
        }

        to_process.insert((steps + 1, (position.0 + 1, position.1)));
        to_process.insert((steps + 1, (position.0 - 1, position.1)));
        to_process.insert((steps + 1, (position.0, position.1 + 1)));
        to_process.insert((steps + 1, (position.0, position.1 - 1)));
    }

    return visited_points;
}
