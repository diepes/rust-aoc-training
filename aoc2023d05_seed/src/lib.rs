pub mod map_seeds;
pub mod nom_parse;

pub fn run(input: &str) {
    //println!("input \n{}", input);
    println!("inputs ^^^^ \n");
    let almanac = nom_parse::parse_map(&input);
    //println!("map \n{:?}", almanac);
    let mut lowest = (u64::MAX, 0_u64);
    let mut cnt_down: i64 = almanac.seeds.len() as i64;
    for (seed_begin, seed_range) in &almanac.seeds {
        cnt_down -= 1;
        let mut cnt_range: u64 = 0;
        let seed_start: u64 = seed_begin.clone();
        let seed_end: u64 = seed_start + seed_range;
        for seed in seed_start ..  seed_end {
            cnt_range += 1;
            // println!("seed: {seed}");
            let (result_val, result_type) =
                map_seeds::map(seed.clone(), "seed", "location", &almanac, false);
            //println!("{cnt_down} seed: {seed} >> {result_type}: {result_val}");
            if result_val < lowest.0 {
                lowest = (result_val, seed.clone());
            }
        }
    println!("cnt:{cnt_down} range:{cnt_range} Lowest so far seed:{} at:{}", lowest.1, lowest.0);
    }
    println!();
    println!(" Lowest found seed:{} at:{}", lowest.1, lowest.0);
    //debug
    //let (result_val, result_type) = map_seeds::map(lowest.1, "seed", "location", &almanac, true);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_get_num_cards() {
        let input = std::fs::read_to_string("in.test.txt").expect("Can't read input file!");
    }
}
