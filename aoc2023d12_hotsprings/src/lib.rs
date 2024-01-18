use std::collections::HashMap;

pub fn run(input: &str) {
    println!("{}", input);
    let data = SpringRow::new(&input);
    //println!("run: {:?}", data);
    let mut total = 0;
    for n in 0..data.len() {
        let data_uf = unfold_springs(&data[n]);
        let calc = calc_options(data_uf);
        total += calc;
        // println!(
        //     "run: {nn}/{len} calc_optons: {calc}",
        //     nn = n + 1,
        //     len = data.len()
        // );
    }
    println!("Total: {total}  correct=1_672_318_386_674");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpringRow {
    springs: Vec<char>,
    counts: Vec<usize>,
}
impl SpringRow {
    fn new(input: &str) -> Vec<SpringRow> {
        let mut hot_springs = vec![];
        for line in input.lines() {
            let (springs, counts) = line.split_once(&" ").expect("Broken input ?");
            let springs: Vec<char> = springs.chars().collect();
            let counts: Vec<usize> = counts.split(&",").map(|s| s.parse().unwrap()).collect();
            hot_springs.push(SpringRow { springs, counts });
        }
        hot_springs
    }
}
fn calc_options(mut springrow: SpringRow) -> usize {
    let mut cache = HashMap::new();
    // add dot '.' at end of springs, always end on '.'
    springrow.springs.push('.');
    let answer = calc_recursive(springrow, &mut cache);
    // println!(" done {answer}");
    answer
}
fn calc_recursive(mut springrow: SpringRow, cache: &mut HashMap<SpringRow, usize>) -> usize {
    let debug = false;
    if debug {
        println!("\ndebug: calc_recursive {:?}", springrow);
    };
    if let Some(result) = cache.get(&springrow) {
        if debug { println!("cache hit {result}"); };
        return result.clone();
    };

    // //short circuit if we require more counts than we have spring options
    // let counts_len = counts.len();
    // let counts_total = if counts_len > 1 {
    //     springs_required + counts[1..].iter().sum::<usize>()
    // } else {
    //     springs_required
    // };
    // // +2 = 1-spring1 and 1-counts_len 3 is only 2 gaps
    // if (springs.len()) < (counts_total + counts_len) {
    //     if debug {
    //         println!("    debug: return 0 - can't satisfy remaining {springs_len} < counts_total:{counts_total} + {counts_len}-1",springs_len=springs.len());
    //     };
    //     return 0;
    // }

    // corner case last char is needed '#' spring
    // if (spring1 == &'#') && (springs_required == 1) && (counts.len() == 0) && (springs.len() == 0) {
    //     //we got last spring.
    //     if debug {
    //         println!("    debug: return 1 - got last spring for last count :)");
    //     };
    //     return 1;
    // }

    if springrow.counts.len() == 0 {
        //search done
        if springrow.springs.iter().any(|ch| ch == &'#') {
            if debug {
                println!("    debug: return 0 - counts matched, but springs left :(");
            };
            return 0; // fail
        } else {
            if debug {
                println!("    debug: return 1 - all counts matched, no more # springs :)");
            };
            return 1;
        };
    };

    if springrow.springs.len() < 2 {
        // know counts>0 but no springs left.
        // !todo!("Add to cache");
        return 0;
    }
    // Step 1
    // '.' strip
    if springrow.springs[0] == '.' {
        //strip all '.' dot's
        // not spring
        if debug {
            print!("    debug: start='.'  ");
        };
        while (springrow.springs.len() > 0) && (springrow.springs[0] == '.') {
            if debug { print!(" +1."); };
            springrow.springs.remove(0);
        }
        if springrow.springs.len() <= 1 {
            if debug {
                println!(" ==> 0, springs.len to short now");
            };
            return 0;
        };
    }; // '.'s removed
       // look if first char '?'
    let mut alt_count = 0;  // '?'
    if springrow.springs[0] == '?' {
        // could be '.' recurse as .
        if debug {
            println!(" next char '?' drop as '.'");
        };
        let springr = SpringRow {
            springs: springrow.springs[1..].to_vec(),
            counts: springrow.counts.clone(),
        };
        let value = calc_recursive(springr.clone(), cache);
        alt_count += value;
        cache.insert(springr, value);
    }; //striped '.'s from start
    if debug {
        println!();
    };

    // step 2
    //'#' or '?' ('.' possible taken care of above.)
    if debug {
        print!("    debug: match !='.' x cnt:{}", springrow.counts[0]);
    };
    // see if next counts[0] is broken spring '#' or unknown '?' followed by '.'
    if springrow.springs.len() <= springrow.counts[0] {
        // not enough char left + '.'
        // !todo!("Add to cache");
        if debug {
            println!(" => to_short=0 alt:{alt_count}");
        };
        return alt_count;
    }
    //
    // match counts //
    //
    // check !. xcnt followed by !#   ( '?' is ok anywhere )
    if springrow.springs[0] != '.' && springrow.springs[springrow.counts[0]] != '#' //not followed by ????'#'  '1'||'?' OK
        && springrow
            .springs
            .iter()
            .take(springrow.counts[0])
            .all(|ch| ch != &'.')
    {
        let springr_cache = springrow.clone();
        // remove match counts from strings + 1x!'#' following
        springrow.springs.drain(0..(springrow.counts[0] + 1));
        springrow.counts.remove(0);
        // search rest
        alt_count += calc_recursive(springrow, cache);
        cache.insert(springr_cache,alt_count);
        if debug {
            println!(" =#+> alt_count:{alt_count}");
        };
        return alt_count;
    } else {
        // step 1 handled [0] as '.' and '?'=>'.'
        // step 2 handled [0] as '#' and '?'=>'#'
        if debug {
            println!(" =>> 0 alt:{alt_count}");
        };
        return alt_count;
    }
}

fn unfold_springs(data: &SpringRow) -> SpringRow {
    SpringRow {
        springs: data
            .springs
            .iter()
            .copied()
            .chain(['?'])
            .cycle()
            .take(5 * data.springs.len() + 4)
            .collect(),
        counts: data
            .counts
            .iter()
            .cycle()
            .cycle()
            .take(5 * data.counts.len())
            .map(|v| v.clone())
            .collect(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn get_data() -> Vec<SpringRow> {
        let input = std::fs::read_to_string("in.tst.txt").expect("Missing file?");
        let data = SpringRow::new(&input);
        data
    }
    #[test]
    fn test0_simple_data() {
        let data: SpringRow = SpringRow {
            springs: vec!['.', '?', '#'],
            counts: vec![1],
        };
        assert_eq!(calc_options(data), 1, "simple_1");
        let data: SpringRow = SpringRow {
            springs: vec!['.', '#', '?'],
            counts: vec![1],
        };
        assert_eq!(calc_options(data), 1, "simple_2");
    }
    #[test]
    fn test1_calc_test_data() {
        let data = get_data();
        for (n, r) in [(0, 1), (1, 4), (2, 1), (3, 1), (4, 4), (5, 10)] {
            assert_eq!(calc_options(data[n].clone()), r, "assert n:{n} r:{r}");
        }
    }
    #[test]
    fn test2_unfold() {
        let data = get_data();
        for (n, r) in [(0, 1)] {
            //, (1, 4), (2, 1), (3, 1), (4, 4), (5, 10)] {
            assert_eq!(
                unfold_springs(&data[n]).springs,
                [
                    '?', '?', '?', '.', '#', '#', '#', '?', '?', '?', '?', '.', '#', '#', '#', '?',
                    '?', '?', '?', '.', '#', '#', '#', '?', '?', '?', '?', '.', '#', '#', '#', '?',
                    '?', '?', '?', '.', '#', '#', '#',
                ],
                "a ssert n:{n} r:{r} springs:{:?}",
                data[n].springs
            );
            assert_eq!(
                unfold_springs(&data[n]).counts,
                [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3,],
                "a ssert n:{n} r:{r} springs:{:?}",
                data[n].springs
            );
        }
    }
}
