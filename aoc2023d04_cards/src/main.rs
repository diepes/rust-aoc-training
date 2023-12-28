use aoc2023d04_cards::run;

fn main() {
    println!("Hello, world!");
    let f_name = "in.txt".to_string();  //9997537
    let input = std::fs::read_to_string(f_name).expect("Can't read input file!");
    run(&input);
}
