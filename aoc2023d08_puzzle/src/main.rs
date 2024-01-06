// Gave up with the Math.
// copy sollution from: https://nickymeuleman.netlify.app/garden/aoc2023-day08
// lcm

use std::collections::HashMap;

enum Instruction {
    Left,
    Right,
}

struct Destinations<'a> {
    left: &'a str,
    right: &'a str,
}

struct Ghost<'a> {
    pos: &'a str,
    cycles: Option<u64>,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part_2(input: &str) -> u64 {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = instructions
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("at the disco"),
        })
        .collect();
    let map: HashMap<&str, Destinations> = map
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(" = ").unwrap();
            let destinations = destinations
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap();
            let destinations = destinations.split_once(", ").unwrap();
            (
                source,
                Destinations {
                    left: destinations.0,
                    right: destinations.1,
                },
            )
        })
        .collect();

    let mut cycle_count = 0;
    let mut ghosts: Vec<Ghost> = map
        .keys()
        // start from all positions ending in 'A'
        .filter(|source| source.ends_with('A'))
        // map every location to a location with a cycle count
        .map(|pos| Ghost { pos, cycles: None })
        .collect();

    while ghosts.iter().any(|ghost| ghost.cycles.is_none()) {
        // Do a full cycle of instructions
        for ins in &instructions {
            for Ghost { pos, cycles } in ghosts.iter_mut() {
                if cycles.is_some() {
                    // this loop already has a known cycle length, no need to simulate further
                    continue;
                }
                let Destinations { left, right } = map.get(pos).unwrap();
                *pos = match ins {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }
        }
        cycle_count += 1;

        // after a full cycle of instructions, save any found cycles (ghosts that arrived at a destination)
        for Ghost { pos, cycles: cycle } in ghosts.iter_mut() {
            if cycle.is_some() {
                // already has a known cycle, no need to update
                continue;
            }
            if pos.ends_with('Z') {
                *cycle = Some(cycle_count);
            }
        }
    }

    let min_shared_cycles = ghosts
        .into_iter()
        .filter_map(|ghost| {
            println!(
                "Debug {} {:?} {} %{}",
                ghost.pos,
                ghost.cycles,
                277 / ghost.cycles.unwrap(),
                277 % ghost.cycles.unwrap()
            );
            ghost.cycles
        })
        .fold(1, |acc, item| lcm(acc, item));

    println!("Debug instructions.len={}", instructions.len());
    min_shared_cycles * instructions.len() as u64
}

fn get_aoc_data(file: Option<&str>) -> String {
    let file_name = file.unwrap_or_else(|| "in.txt");
    println!("Load file {file_name}");
    let input = std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Problem reading `{file_name}` "));
    input
}

fn main() {
    println!("Hello");
    let input = get_aoc_data(Some("in.txt"));
    let ans = part_2(&input);
    println!("part2 {ans}");
}
