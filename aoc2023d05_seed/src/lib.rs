pub mod nom_parse;

pub fn run(input: &str) {
    println!("input \n{}", input);
    let almanac = nom_parse::parse_map(&input);
    println!("map \n{:?}", almanac);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_get_num_cards() {
        let input = std::fs::read_to_string("in.test.txt").expect("Can't read input file!");
    }
}
