fn main() {
    println!("Hello, world!");
    let input = std::fs::read_to_string("in.txt").expect("Err reading input file!");
    aoc2023d11_stars::run(&input, 1000000);
    println!("The END.");
}
