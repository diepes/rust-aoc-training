use aoc2023d05_seed::run;

fn main() {
    println!("Hello, world!");
    let f_name = "in.test.txt".to_string(); //
    let input = std::fs::read_to_string(f_name).expect("Can't read input file!");
    run(&input);
}
