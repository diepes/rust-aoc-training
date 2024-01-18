fn main() {
    println!("Hello, world!");
    let input = std::fs::read_to_string("in.txt").expect("Missing file?");
    aoc2023d12_hotsprings::run(&input);
}
