use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
};
use z3::{
    ast::{Ast, Int},
    SatResult,
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

    let hailstones = lines
        .iter()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").unwrap();
            let (px, py, pz) = position.split(",").collect_tuple().unwrap();
            let (vx, vy, vz) = velocity.split(",").collect_tuple().unwrap();
            (
                (
                    px.trim().parse::<i64>().unwrap(),
                    py.trim().parse::<i64>().unwrap(),
                    pz.trim().parse::<i64>().unwrap(),
                ),
                (
                    vx.trim().parse::<i64>().unwrap(),
                    vy.trim().parse::<i64>().unwrap(),
                    vz.trim().parse::<i64>().unwrap(),
                ),
            )
        })
        .collect_vec();

    // I think I can just figure out an equation for each hailstone and then see if the hailstons will cross
    // If we have 1, 2, 3 and velocity of 10, 20, 30 then we can represent y with y = 2x since we grow at 2x and we actually start at 2x
    // A more complicated formula of 5, 6, 7 and 10, 20, 30 would be y = 2x - 4
    // You can determine if they cross by setting 2x - 4 = 2x and solving for x. In this case x disappears and is anything which means they do not cross.

    let equations = hailstones
        .iter()
        .map(|((px, py, _), (vx, vy, _))| {
            let x_mult = *vy as f64 / *vx as f64;
            let constant = *py as f64 - (x_mult * *px as f64);

            // // Let's double check
            // let actual_y2 = py + vy;
            // let actual_x2 = px + vx;
            // let equation_y2 = x_mult * actual_x2 as f64 + constant;
            // println!(
            //     "{}, {} == {}, {}?",
            //     actual_x2, equation_y2, actual_x2, actual_y2
            // );

            (constant, x_mult, *px, *py, *vx, *vy)
        })
        .collect_vec();

    let min_range: i64 = 200000000000000;
    let max_range: i64 = 400000000000000;

    // let min_range: i64 = 7;
    // let max_range: i64 = 27;

    let part1 = equations
        .iter()
        .combinations(2)
        .filter(|e| will_cross(*e[0], *e[1], min_range as f64, max_range as f64))
        .count();
    println!("Part 1: {}", part1);

    // Part 2:
    // We need to find points where x = starting_x + vx*t == my_starting_x + vx*t for all snowballs. We can do this independently for x, y, and z, but the T has to be the same for the crossing
    // Maybe the equation is starting_x + vx*t + starting_y + vy*t + starting_z + vz*t such that for some t this equals my line
    // Or treat each dimension independently y = starting_y + vy*t == my_y + my_vy * t for all entries. The problem is the same T needs to apply for other equations...
    // 3D line is apparently (x-x1)/a = (y-y1)/b = (z-z1)/c
    // So we're getting a relationship between x, y, and z
    // Maybe we just use a linear algebra solver outside of this...
    // Actually, z3 is apparently a thing that can be used
    let config = z3::Config::new();
    let context = z3::Context::new(&config);
    let solver = z3::Solver::new(&context);
    let fx = z3::ast::Int::new_const(&context, "fx");
    let ref_fx = &fx;
    let fy = z3::ast::Int::new_const(&context, "fy");
    let ref_fy = &fy;
    let fz = z3::ast::Int::new_const(&context, "fz");
    let ref_fz = &fz;
    let fdx = z3::ast::Int::new_const(&context, "fdx");
    let ref_fdx = &fdx;
    let fdy = z3::ast::Int::new_const(&context, "fdy");
    let ref_fdy = &fdy;
    let fdz = z3::ast::Int::new_const(&context, "fdz");
    let ref_fdz = &fdz;

    hailstones
        .iter()
        .map(|((px, py, pz), (vx, vy, vz))| {
            let t = z3::ast::Int::new_const(&context, format!("t{}_{}_{}", px, py, pz));
            solver.assert(&t.ge(&z3::ast::Int::from_i64(&context, 0)));
            solver.assert(
                &Int::from_i64(&context, *px)
                    .add(&Int::from_i64(&context, *vx).mul(&t.clone()))
                    ._eq(&ref_fx.add(&ref_fdx.mul(t.clone()))),
            );
            solver.assert(
                &Int::from_i64(&context, *py)
                    .add(&Int::from_i64(&context, *vy).mul(&t.clone()))
                    ._eq(&ref_fy.add(&ref_fdy.mul(t.clone()))),
            );
            solver.assert(
                &Int::from_i64(&context, *pz)
                    .add(&Int::from_i64(&context, *vz).mul(&t.clone()))
                    ._eq(&ref_fz.add(&ref_fdz.mul(t))),
            );
        })
        .collect_vec();
    let result = solver.check();
    if result != SatResult::Sat {
        println!("Can't find answer??");
    }

    let part2 = solver
        .get_model()
        .unwrap()
        .eval(&fx.add(fy).add(fz), true)
        .unwrap();
    println!("Part 2: {}", part2);
}

fn will_cross(
    (ac, ax, pax, pay, vax, vay): (f64, f64, i64, i64, i64, i64),
    (bc, bx, pbx, pby, vbx, vby): (f64, f64, i64, i64, i64, i64),
    min_range: f64,
    max_range: f64,
) -> bool {
    if ax == bx && ac != bc {
        return false;
    }

    // y = 1 + 5x
    // y = 10 - 2x
    // 1 + 5x = 10 - 2x
    // 5x - (-2x) = 10 - 1
    // ac + ax*x = bc + bx*x
    // ax*x = bc - ac + bx*x
    // (ax - bx)*x = bc - ac
    // x = (bc - ac) / (ax - bx)

    let crossing_x = (bc - ac) / (ax - bx);
    let crossing_y = crossing_x * ax + ac;

    print!(
        "{}, {} and {}, {} - Crossing @ {}, {}",
        pax, pay, pbx, pby, crossing_x, crossing_y
    );

    let valid = crossing_x <= max_range
        && crossing_x >= min_range
        && crossing_y <= max_range
        && crossing_y >= min_range;

    print!(": {}", valid);

    if valid {
        // I also need to look at when they crossed as it needs to be in the future. This will be if the x and y have the same direction as the vx and vy
        let still_valid = (pax as f64 - crossing_x).is_sign_positive() == vax.is_negative()
            && (pay as f64 - crossing_y).is_sign_positive() == vay.is_negative()
            && (pbx as f64 - crossing_x).is_sign_positive() == vbx.is_negative()
            && (pby as f64 - crossing_y).is_sign_positive() == vby.is_negative();
        println!(": / {}", still_valid);

        return still_valid;
    }
    println!();

    return valid;
}
