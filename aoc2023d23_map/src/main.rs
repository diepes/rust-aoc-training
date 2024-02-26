use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let data = read_file("in.tst.txt");
    println!("data:\n{}", data);
    let data = parse_into_hash_map(&data);
    // println!("HashMap\n{:?}", data);
    print_hashmap(&data);
    let mut routes: Vec<Data> = vec![data.clone()];
    let mut not_end = true;
    while not_end {
        for n in 0..routes.len() {
            let step = take_step(&mut routes, n);
            if step == 0 {
                not_end = false
            }
        }
    }
}

fn read_file(file_name: &str) -> String {
    let data = std::fs::read_to_string(file_name).expect("Can't read file.");
    data
}

fn take_step(mut routes: &Vec<Data>, n: usize) -> usize {
    let mut data = &routes[n];
    if data.pos == data.end {
        // return steps if we reached the end.
        return data.steps;
    };
    // take a step.
    // see if current pos has a directions
    let next = find_next_steps(&data);

    0
}
fn find_next_steps(data: &Data) -> Vec<XY> {
    let mut next = Vec::new();
    let directions = match data.hm.get(&data.pos) {
        None => panic!("data.pos for empty XY ?"),
        Some(Road::Down) => vec![XY {
            x: data.pos.x,
            y: data.pos.y + 1,
        }],
        Some(Road::Right) => vec![
            XY {
                x: data.pos.x + 1,
                y: data.pos.y,
            },
            XY { x: 1, y: 2 },
        ],
        Some(Road::Normal(_)) => {
            //array check all 4 directions.
            for (d_x, d_y) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                // check for negative out of bounds
                if (d_x < 0 && data.pos.x == 0) || (d_y < 0 && data.pos.y == 0) {
                    continue;
                };
                //match  data.hm.get(& XY{ x: (d_x + data.pos.x as isize) as usize, y: (d_y + data.pos.y as isize) as usize } ) {
                //   None => { println!("abc"); },
                //   }
                // }
            }
            vec![]
        }
    };

    next
}
fn print_hashmap(data: &Data) {
    println!();
    for y in 0..data.max_y {
        for x in 0..data.max_x {
            let p = XY { x, y };
            match data.hm.get(&p) {
                Some(Road::Normal(v)) => print!("{r}", r = v % 10),
                Some(Road::Right) => print!(">"),
                Some(Road::Down) => print!("v"),
                None => print!("#"),
            }
        }
        println!();
    }
    println!();
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct XY {
    x: usize,
    y: usize,
}

// PartialEq for test
#[derive(Debug, Clone, PartialEq)]
enum Road {
    Normal(usize),
    Down,
    Right,
}

impl Road {
    fn new(ch: char) -> Option<Road> {
        match ch {
            '.' => Some(Road::Normal(0)),
            '>' => Some(Road::Right),
            'v' => Some(Road::Down),
            '#' => None,
            _ => panic!("Unknown char '{}'", ch),
        }
    }
}
#[derive(Debug, Clone)]
struct Data {
    hm: HashMap<XY, Road>,
    max_x: usize,
    max_y: usize,
    pos: XY,
    end: XY,
    steps: usize,
}
fn _parse_input(input: &str) -> HashMap<XY, Road> {
    let mut map = HashMap::new();
    for (row, line) in input.split('\n').enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if let Some(road) = Road::new(ch) {
                map.insert(XY { y: row, x: col }, road);
            }
        }
    }
    map
}
fn parse_into_hash_map(data: &str) -> Data {
    let mut max_x = 0;
    let mut max_y = 0;
    let hm = data
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            if max_y == 0 {
                max_x = line.len();
            }
            max_y += 1;
            line.trim().chars().enumerate().filter_map(move |(x, ch)| {
                let p = XY { x, y };
                Road::new(ch).map(|road| (p, road))
            })
        })
        .collect();
    Data {
        hm,
        max_x,
        max_y,
        pos: XY { x: 1, y: 0 },
        end: XY {
            x: max_x - 2,
            y: max_y - 1,
        },
        steps: 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
        #.#########
        #....v....#
        #####.#.#.#
        #####.>...#
        #########.#
        ";
    #[test]
    fn gen_hash_map() {
        let data = parse_into_hash_map(&INPUT);
        assert_eq!(
            data.hm.get(&XY {
                x: data.pos.x,
                y: data.pos.y
            }),
            Some(&Road::Normal(0)),
            " Entry is road."
        );
        assert_eq!(
            data.hm.get(&XY {
                x: data.end.x,
                y: data.end.y
            }),
            Some(&Road::Normal(0)),
            " Exit is road."
        );
        assert_eq!(
            data.hm.get(&XY { x: 6, y: 3 }),
            Some(&Road::Right),
            " Expect Road::Right"
        );
        assert_eq!(
            data.hm.get(&XY {
                x: data.pos.x,
                y: data.pos.y
            }),
            Some(&Road::Normal(0)),
            " Entry is road."
        );
        // start possition guessed
        assert_eq!(data.pos.x, 1);
        assert_eq!(data.pos.y, 0);
        // end possions guessed
        assert_eq!(data.end.x, 9);
        assert_eq!(data.end.y, 4);
    }
}
