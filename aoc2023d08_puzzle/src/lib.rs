use nom;
use std::time::Instant;
mod bruteforcetwo;

pub fn run() {
    let t_start = Instant::now();
    let input = get_aoc_data(Some("in.txt"));
    println!("input: {:?}", input);
    // println!("input: {:?}", std::str::from_utf8(input).unwrap());
    let (input, puzzle_map) = nom_parse(&input).expect("nom_parser_failed");
    assert_eq!(input, "");
    println!("puzzle_map: {puzzle_map:#?}");
    println!("directions: {:?}", (&puzzle_map.directions));
    let mut count = 0_u64;
    let next_names: Vec<String> = puzzle_map
        .nodes
        .iter()
        .filter(|n| n.node_name.ends_with('A'))
        .map(|n| n.node_name.to_string())
        .collect();
    let run_count: usize = next_names.len();
    // Create Runs Vec record everytime a run reaches the end **Z and the count.
    let mut runs: Vec<Run> = next_names
        .iter()
        .enumerate()
        .map(|(i, n)| Run::new(i, n))
        .collect();
    println!("Starting with #{} {:?}", next_names.len(), next_names);
    let mut pick = Directions::new(&puzzle_map.directions);
    let mut flag_warp = false;
    let mut show_next: u64 = 0;
    loop {
        let t_loop = Instant::now();
        let mut count_loop_z = run_count; //number of parralell runs
        let (p, _p_index) = pick.next().unwrap();
        let (mut next_step_small, mut next_step_large) = (0_u64, 0_u64);
        let mut debug_small = "".to_string();
        let mut count_at_end = 0;
        for i in 0..run_count {
            assert_eq!(runs[i].run_id, i, "Check run id");
            let (mut step, mut got_step_size, _run_name, mut _position) =
                runs[i].get_current_step();
            while step < count {
                (step, got_step_size, _position) = runs[i].get_next_step(&puzzle_map, p);
            }
            assert!(step >= count, "Step < count ??");
            if next_step_small == 0 || step <= next_step_small {
                next_step_small = step;
                if next_step_small == count && flag_warp {
                    debug_small.push_str(&format!("i={i}_s{},", step - count));
                };
            };
            if next_step_large == 0 || step > next_step_large {
                next_step_large = step;
            };
            if got_step_size {
                count_loop_z -= 1; //count down how many not found loop value
            }
            if step == count {
                count_at_end += 1;
            }
        }
        //
        if (count_at_end > 1 && flag_warp) || count > show_next {
            show_next += 50_000_000;
            if count_loop_z == 0 {
                flag_warp = true;
            };
            let mut time_total = t_start.elapsed().as_secs();
            if time_total == 0 {
                time_total = 1_u64;
            } // prevent initial divide by zero
            let time_loop = t_loop.elapsed();
            let run_names = &runs.iter().map(|r| &r.node_current).collect::<Vec<_>>();
            println!(
            "{count:>4} jump={jump_small:>3} {debug_small} /jl={jump_large:>4} warp:{flag_warp} z's: {count_loop_z}  p={p:<5} {run_names:?}  t_l={time_loop:.3?} t_t={time_total:.0?} rate={rate:.2}",
            rate = count/time_total,
            jump_small = next_step_small - count,
            jump_large = next_step_large - count,

        );
        }
        if runs
            .iter()
            .all(|n| n.node_current.ends_with("Z") && n.step_current == count)
        {
            println!(
                "Found ZZZ, for all. count: {count}  {run_names:?}",
                run_names = &runs.iter().map(|r| &r.node_current).collect::<Vec<_>>()
            );
            break;
        }
        if flag_warp {
            println!(" Now bruteforcing the awnser from discoverd cycles !!!");
            bruteforcetwo::doit(runs);
            panic!("Done");
        } else {
            count += 1;
        }
        //for i in 0..new_names.len() {
        //next_names = new_names;
        //}
    }
    println!("The END.");
}

#[derive(Debug)]
pub struct Run {
    // single run
    run_id: usize,
    node_current: String,
    step_current: u64,
    got_step_size: bool,
    // steps to get to repeating Z
    step_initial: u64,
    // steps between repeating Z's
    step_size: u64,
    // vec of node ((name, RulePos) , total_count) and possition in instruction L,R list.
    node_ends: Vec<((String, u64), u64)>,
    loop_position: u64, // position in L,R instructions for end ..Z
}
impl Run {
    fn new(run_id: usize, node_start: &str) -> Run {
        Run {
            run_id,
            node_current: node_start.to_string(),
            got_step_size: false,
            step_current: 0, //total of all steps
            step_initial: 0, //steps up to repeat ..Z loop start
            step_size: 0,    //steps to loop from ..Z to ..Z
            node_ends: vec![],
            loop_position: 0,
        }
    }
    fn get_current_step(&self) -> (u64, bool, &str, u64) {
        (
            self.step_current,
            self.got_step_size,
            &self.node_current,
            self.loop_position,
        )
    }
    fn get_next_step(&mut self, puzzle: &Puzzle, lookup_direction: char) -> (u64, bool, u64) {
        // lookup_direction only used during single step part of search.
        if self.got_step_size {
            self.step_current += self.step_size;
            return (self.step_current, self.got_step_size, self.loop_position);
        }
        // let l = self.node_ends.len();
        let (new_name, position) = puzzle.lookup(&self.node_current, lookup_direction);
        self.node_current = new_name.clone();
        self.step_current += 1;
        self.loop_position = position;
        if new_name.ends_with("Z") {
            self.node_ends
                .push(((new_name, position), self.step_current));
            // check if we have a loop yet
            let l = self.node_ends.len();
            if l > 1 {
                // check if we have loop yet
                for j in (0..(l - 1)).rev() {
                    if self.node_ends[l - 1].0 == self.node_ends[j].0 {
                        // we got a repeat.
                        self.step_size = (self.node_ends[l - 1].0 .1) as u64;
                        self.step_initial = self.node_ends[j].1;
                        self.got_step_size = true;
                        self.loop_position = position;
                        break;
                    };
                }
            };
        };
        return (self.step_current, self.got_step_size, position);
    }

    fn jump_step_to(&mut self, goal: u64) -> u64 {
        if self.got_step_size {
            while self.step_current < goal {
                self.step_current += self.step_size;
            }
            return self.step_current;
        } else {
            panic!(" Cant jump_step_to before we know our loop step size.")
        }
    }
}

#[derive(Debug)]
struct Puzzle<'a> {
    directions: &'a str,
    nodes: Vec<Node<'a>>,
}
impl Puzzle<'_> {
    fn lookup(&self, node_name: &str, dir: char) -> (String, u64) {
        let (new_name, position) = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_j, n)| n.node_name == node_name)
            .map(|(j, n)| {
                if dir == 'L' {
                    (n.l.to_string(), j)
                } else if dir == 'R' {
                    (n.r.to_string(), j)
                } else {
                    panic!("lookup dir should be 'L' or 'R' ")
                }
            })
            .expect("Missing name ?");
        (new_name, position as u64)
    }
}

struct Directions {
    iter_chars: Vec<char>,
    iter_index: usize,
}
impl Directions {
    fn new(dir_str: &str) -> Directions {
        Directions {
            iter_chars: dir_str.chars().collect(),
            iter_index: 0,
        }
    }
}
impl Iterator for Directions {
    type Item = (char, usize);
    fn next(&mut self) -> Option<(char, usize)> {
        if self.iter_index >= self.iter_chars.len() {
            self.iter_index = 0;
        }
        let result = Some((self.iter_chars[self.iter_index], self.iter_index));
        self.iter_index += 1;
        result
    }
}
#[derive(Debug)]
struct Node<'b> {
    node_name: &'b str,
    l: &'b str,
    r: &'b str,
}

fn nom_parse(input: &str) -> nom::IResult<&str, Puzzle> {
    let (input, directions) = nom::character::complete::alpha1(input)?;
    let (input, _break_nl) = nom::character::complete::multispace1(input)?;
    let (input, nodes) =
        nom::multi::separated_list0(nom::character::complete::multispace1, parse_node)(input)?;
    Ok((
        input,
        Puzzle {
            directions: directions,
            nodes: nodes,
        },
    ))
}
fn parse_node(input: &str) -> nom::IResult<&str, Node> {
    let (input, node_name) = nom::character::complete::alpha1(input)?;
    let (input, _tag) = nom::bytes::complete::tag(" = (")(input)?;
    let (input, l) = nom::character::complete::alpha1(input)?;
    let (input, _tag) = nom::bytes::complete::tag(", ")(input)?;
    let (input, r) = nom::character::complete::alpha1(input)?;
    let (input, _tag) = nom::bytes::complete::tag(")")(input)?;
    Ok((input, Node { node_name, l, r }))
}

fn get_aoc_data(file: Option<&str>) -> String {
    let file_name = file.unwrap_or_else(|| "in.txt");
    println!("Load file {file_name}");
    let input = std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Problem reading `{file_name}` "));
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_puzzle(input: &mut String) -> Puzzle {
        //let input = get_aoc_data(Some("in.test.txt"));
        *input = std::fs::read_to_string("in.test.txt")
            .unwrap_or_else(|_| panic!("Problem reading `in.test.txt` "));
        println!("input: {:?}", input);
        // println!("input: {:?}", std::str::from_utf8(input).unwrap());
        let (out, puzzle_map) = nom_parse(input).expect("nom_parser_failed");
        assert_eq!(out, "");
        puzzle_map
    }
    #[test]
    fn test_run() {
        let mut input: String = Default::default();
        let puzzle = get_puzzle(&mut input);
        let mut pick = Directions::new(&puzzle.directions);
        let mut r = Run::new(1, "AAA");
        assert_eq!(r.node_current, "AAA");

        assert_eq!(r.get_current_step(), (0, false, "AAA", 0));
        //
        assert_eq!(pick.next().unwrap().0, 'L');
        assert_eq!(r.get_next_step(&puzzle, 'L'), (1, false, 0));
        assert_eq!(r.get_current_step(), (1, false, "BBB", 0));
        assert_eq!(r.node_current, "BBB");
        //
        assert_eq!(r.get_next_step(&puzzle, 'L'), (2, false, 1));
        assert_eq!(r.node_current, "AAA");
        // add entries
        assert_eq!(r.get_next_step(&puzzle, 'R'), (3, false, 0));
        assert_eq!(r.node_current, "BBB");
        assert_eq!(r.get_next_step(&puzzle, 'R'), (4, false, 1)); // false as p=3 was 2 before
        assert_eq!(r.get_current_step(), (4, false, "ZZZ", 1));
        assert_eq!(r.get_next_step(&puzzle, 'L'), (5, false, 2));
        assert_eq!(r.get_next_step(&puzzle, 'L'), (6, true, 2));
        assert_eq!(r.get_next_step(&puzzle, 'L'), (8, true, 2));
        assert_eq!(r.get_next_step(&puzzle, 'R'), (10, true, 2));
        assert_eq!(r.node_current, "ZZZ");
        assert_eq!(r.jump_step_to(21), 22);
        assert_eq!(r.get_current_step(), (22, true, "ZZZ", 2));
    }
    #[test]
    fn test_direction_iter() {
        let mut dir = Directions::new("LLRL");
        assert_eq!(dir.next(), Some(('L', 0)));
        assert_eq!(dir.next(), Some(('L', 1)));
        assert_eq!(dir.next(), Some(('R', 2)));
        assert_eq!(dir.next(), Some(('L', 3)));
        assert_eq!(dir.next(), Some(('L', 0)));
        assert_eq!(dir.next(), Some(('L', 1)));
        assert_eq!(dir.next(), Some(('R', 2)));
    }
}
