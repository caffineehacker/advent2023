use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
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
    do_part2: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MachineType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

trait Machine {
    fn handle_pulse(&mut self, source: &str, is_high: bool) -> Vec<(String, bool)>;
    fn get_destinations(&self) -> &Vec<String>;
    fn machine_type(&self) -> MachineType;
    fn add_source(&mut self, source: String);
}

#[derive(PartialEq, Eq, Clone)]
struct FlipFlop {
    destinations: Vec<String>,
    state: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            destinations: Vec::new(),
            state: false,
        }
    }
}

impl Machine for FlipFlop {
    fn handle_pulse(&mut self, source: &str, is_high: bool) -> Vec<(String, bool)> {
        if !is_high {
            self.state = !self.state;

            self.destinations
                .iter()
                .cloned()
                .map(|dest| (dest, self.state))
                .collect_vec()
        } else {
            Vec::new()
        }
    }

    fn get_destinations(&self) -> &Vec<String> {
        return &self.destinations;
    }

    fn machine_type(&self) -> MachineType {
        MachineType::FlipFlop
    }

    fn add_source(&mut self, source: String) {}
}

#[derive(PartialEq, Eq, Clone)]
struct Conjunction {
    destinations: Vec<String>,
    sources_states: HashMap<String, bool>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            destinations: Vec::new(),
            sources_states: HashMap::new(),
        }
    }
}

impl Machine for Conjunction {
    fn handle_pulse(&mut self, source: &str, is_high: bool) -> Vec<(String, bool)> {
        *self.sources_states.get_mut(source).unwrap() = is_high;
        let output = !self.sources_states.values().all(|state| *state);
        self.destinations
            .iter()
            .cloned()
            .map(|dest| (dest, output))
            .collect_vec()
    }

    fn get_destinations(&self) -> &Vec<String> {
        return &self.destinations;
    }

    fn machine_type(&self) -> MachineType {
        MachineType::Conjunction
    }

    fn add_source(&mut self, source: String) {
        if !self.sources_states.contains_key(&source) {
            self.sources_states.insert(source, false);
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Broadcast {
    destinations: Vec<String>,
}

impl Broadcast {
    fn new() -> Self {
        Self {
            destinations: Vec::new(),
        }
    }
}

impl Machine for Broadcast {
    fn handle_pulse(&mut self, source: &str, is_high: bool) -> Vec<(String, bool)> {
        self.destinations
            .iter()
            .cloned()
            .map(|dest| (dest, is_high))
            .collect_vec()
    }

    fn get_destinations(&self) -> &Vec<String> {
        return &self.destinations;
    }

    fn machine_type(&self) -> MachineType {
        MachineType::Broadcast
    }

    fn add_source(&mut self, source: String) {}
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut machines: HashMap<String, Box<dyn Machine>> =
        lines.iter().map(|line| line_to_machine(line)).collect();
    for i in 0..machines.len() {
        let (name, machine) = machines.iter().nth(i).unwrap();
        let name = name.to_string();
        let destinations = machine.get_destinations().clone();
        destinations.into_iter().for_each(|destination| {
            let dest_machine = machines.get_mut(&destination);
            if dest_machine.is_some() {
                let dest_machine = dest_machine.unwrap();
                if dest_machine.machine_type() == MachineType::Conjunction {
                    dest_machine.as_mut().add_source(name.to_string());
                }
            }
        })
    }

    if !args.do_part2 {
        part1(machines, args.debug);
        return;
    }

    part2(machines, args.debug);
}

fn part1(mut machines: HashMap<String, Box<dyn Machine>>, debug: bool) {
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _ in 0..1000 {
        let mut to_process = VecDeque::new();
        to_process.push_back(("broadcaster".to_owned(), "source".to_owned(), false));
        low_pulses += 1;
        while !to_process.is_empty() {
            let (destination, source, is_high) = to_process.pop_front().unwrap();
            if debug {
                println!("{} {} -> {}", source, is_high, destination);
            }
            let machine = machines.get_mut(&destination);
            if machine.is_none() {
                continue;
            }
            let machine = machine.unwrap();
            let outputs = machine.handle_pulse(&source, is_high);
            for (new_destination, new_is_high) in outputs.into_iter() {
                if new_is_high {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
                to_process.push_back((new_destination, destination.to_owned(), new_is_high));
            }
        }
    }

    println!("Part 1: {}", high_pulses * low_pulses);
}

#[derive(Clone)]
struct SearchState {
    known_node_states: HashMap<String, bool>,
    button_presses: u64,
    to_process: VecDeque<(String, bool)>,
}

fn part2(mut machines: HashMap<String, Box<dyn Machine>>, debug: bool) {
    // We really probably want the conjunction cycle times. From manually analyzing the input we can see that there are a few key conjunctions that actually matter. I suspect they will cycle fairly quickly, but out of sync.
    let mut conjunction_cycles = HashMap::new();
    let number_of_conjunctions = machines
        .iter()
        .filter(|machine| machine.1.machine_type() == MachineType::Conjunction)
        .count();

    let mut button_presses = 0;
    loop {
        let mut to_process = VecDeque::new();
        to_process.push_back(("broadcaster".to_owned(), "source".to_owned(), false));
        button_presses += 1;
        while !to_process.is_empty() {
            let (destination, source, is_high) = to_process.pop_front().unwrap();
            if debug {
                println!("{} {} -> {}", source, is_high, destination);
            }
            let machine = machines.get_mut(&destination);
            if machine.is_none() {
                continue;
            }
            let machine = machine.unwrap();
            let outputs = machine.handle_pulse(&source, is_high);
            if machine.machine_type() == MachineType::Conjunction && !outputs[0].1 {
                if !conjunction_cycles.contains_key(&destination) {
                    conjunction_cycles.insert(destination.clone(), button_presses);
                    println!("{}: {}", destination, button_presses);
                }

                // We stop after 100000 button presses since we are guessing that our cycles are less than that
                // This is absolutely a cheat and a hack
                if conjunction_cycles.len() == number_of_conjunctions || button_presses > 100000 {
                    for (name, count) in conjunction_cycles.iter() {
                        println!("{}: {}", name, count);
                    }

                    println!(
                        "Part 2: {}",
                        lcm(conjunction_cycles.values().cloned().collect_vec())
                    );

                    return;
                }
            }
            for (new_destination, new_is_high) in outputs.into_iter() {
                to_process.push_back((new_destination, destination.to_owned(), new_is_high));
            }
        }
    }
}

fn line_to_machine(line: &String) -> (String, Box<dyn Machine>) {
    let (name, destinations) = line.split_once(" -> ").unwrap();
    let name = name
        .trim_start_matches("%")
        .trim_start_matches("&")
        .to_string();
    let mut destinations = destinations
        .split(",")
        .map(|dest| dest.trim().to_string())
        .collect_vec();
    if line.starts_with("&") {
        let mut output = Conjunction::new();
        output.destinations.append(&mut destinations);
        (name, Box::new(output))
    } else if line.starts_with("%") {
        let mut output = FlipFlop::new();
        output.destinations.append(&mut destinations);
        (name, Box::new(output))
    } else {
        let mut output = Broadcast::new();
        output.destinations.append(&mut destinations);
        (name, Box::new(output))
    }
}

pub fn lcm(nums: Vec<i64>) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums.iter().skip(1).cloned().collect_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
