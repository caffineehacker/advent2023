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
}

#[derive(PartialEq, Eq)]
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

    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for i in 0..1000 {
        let mut to_process = VecDeque::new();
        to_process.push_back(("broadcaster".to_owned(), "source".to_owned(), false));
        low_pulses += 1;
        while !to_process.is_empty() {
            let (destination, source, is_high) = to_process.pop_front().unwrap();
            if args.debug {
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
