fn main() {
    println!("Hello, world!");
    let file_name = "in.txt";
    let input = std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Problem reading `{file_name}` "));
    //expect("Problem reading file {file_name}");
    aoc2023d06_race::run(&input);
}
