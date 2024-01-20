use std::collections::HashMap;
struct Cycle {
    start: usize,
    jump: usize,
}
pub fn run() {
    let mut map = read_input("in.txt");
    println!("1:");
    map.print();
    println!("{}", map.calc_weight());
    println!("2:");
    let mut old_map = map.clone();
    let mut hash: HashMap<Map,Cycle> = HashMap::new();
    let mut current_step = 1;
    let cycle_max = 1_000_000_000;
    while current_step <= cycle_max {
        map = map.shift_north();
        map = map.shift_west();
        map = map.shift_south();
        map = map.shift_east();
        if let Some(cycle) = hash.get(&map) {
            // println!(" match current_step: {current_step} old_cycle: {old_cycle} ");
            if cycle.jump == 0 {
                hash.insert(map.clone(), Cycle{ start: cycle.start, jump: current_step - cycle.start });
            } else {
                assert_eq!( (current_step - cycle.start) % cycle.jump, 0 , " Incorrect jump calc");
                //jump
                let jumps = ( cycle_max - current_step) / cycle.jump;
                current_step += cycle.jump * jumps;

            }
        } else {
            hash.insert(map.clone(), Cycle{ start: current_step, jump: 0 });
            println!("insert hash not found current_step: {current_step}");
        };
        if current_step % 100_000 == 0 {
            println!("at current_step {current_step}");
            if map == old_map {
                println!(" map not changing !");
            } else {
                old_map = map.clone();
            };
        };
        current_step += 1;
    }
    println!("After cycle weight: {} = 64 & 91286", map.calc_weight());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rock {
    R,
    S,
    E,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    data: Vec<Vec<Rock>>,
}
impl Map {
    fn print(&self) {
        for line in &self.data {
            for rock in line {
                match rock {
                    Rock::E => print!("."),
                    Rock::R => print!("O"),
                    Rock::S => print!("#"),
                }
            }
            println!();
        }
    }

    fn calc_weight(&self) -> usize {
        let (lx, ly) = (self.data[0].len(), self.data.len());
        let mut weight = 0;
        for x in 0..lx {
            for y in 0..ly {
                match self.data[y][x] {
                    Rock::R => weight += ly - y,
                    _ => (),
                }
            }
        }
        weight
    }

    fn shift_north(&self) -> Map {
        // shift Round rocks north
        let mut data = self.data.clone();
        let (lx, ly) = (data[0].len(), data.len());
        for x in 0..lx {
            for y in 0..ly {
                if data[y][x] == Rock::E {
                    // find something to move north
                    for ys in (y + 1)..ly {
                        match data[ys][x] {
                            Rock::E => continue, // look further
                            Rock::S => break,
                            Rock::R => {
                                Self::swap_rocks(&mut data, x, y, x, ys);
                                break;
                            }
                        }
                    }
                }
            }
        }
        Map { data }
    }

    fn shift_east(&self) -> Map {
        // shift Round rocks to Right
        let mut data = self.data.clone();
        let (lx, ly) = (data[0].len(), data.len());
        for y in 0..ly {
            for x in (1..lx).rev() {
                if data[y][x] == Rock::E {
                    // find something to move Right
                    for xs in (0..x).rev() {
                        match data[y][xs] {
                            Rock::E => continue, // look further
                            Rock::S => break,
                            Rock::R => {
                                Self::swap_rocks(&mut data, x, y, xs, y);
                                break;
                            }
                        }
                    }
                }
            }
        }
        Map { data }
    }

    fn shift_west(&self) -> Map {
        // shift Round rocks to Left
        let mut data = self.data.clone();
        let (lx, ly) = (data[0].len(), data.len());
        for y in 0..ly {
            for x in 0..lx - 1 {
                if data[y][x] == Rock::E {
                    // find something to move north
                    for xs in x + 1..lx {
                        match data[y][xs] {
                            Rock::E => continue, // look further
                            Rock::S => break,
                            Rock::R => {
                                Self::swap_rocks(&mut data, x, y, xs, y);
                                break;
                            }
                        }
                    }
                }
            }
        }
        Map { data }
    }

    fn shift_south(&self) -> Map {
        // shift Round rocks south
        let mut data = self.data.clone();
        let (lx, ly) = (data[0].len(), data.len());
        for x in 0..lx {
            for y in (1..ly).rev() {
                //start at bottom
                if data[y][x] == Rock::E {
                    // find something to move south
                    for ys in (0..y).rev() {
                        match data[ys][x] {
                            Rock::E => continue, // look further
                            Rock::S => break,
                            Rock::R => {
                                Self::swap_rocks(&mut data, x, y, x, ys);
                                break;
                            }
                        }
                    }
                }
            }
        }
        Map { data }
    }

    fn swap_rocks(data: &mut Vec<Vec<Rock>>, x1: usize, y1: usize, x2: usize, y2: usize) {
        let t1 = &data[y1][x1].clone();
        let t2 = std::mem::replace(&mut data[y2][x2], t1.clone());
        let _t1 = std::mem::replace(&mut data[y1][x1], t2);
    }
}
fn read_input(file_name: &str) -> Map {
    let input = std::fs::read_to_string(file_name).expect("Cant read file");
    let map: Map = Map {
        data: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        'O' => Rock::R,
                        '#' => Rock::S,
                        '.' => Rock::E,
                        _ => panic!(" Unknow rock type {ch}"),
                    })
                    .collect::<Vec<Rock>>()
            })
            .collect::<Vec<Vec<Rock>>>(),
    };
    map // only one map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_rocks_east_right() {
        let map = Map {
            data: vec![vec![Rock::R, Rock::E], vec![Rock::E, Rock::R]],
        };
        assert_eq!(
            map.shift_east(),
            Map {
                data: vec![vec![Rock::E, Rock::R], vec![Rock::E, Rock::R]]
            }
        );
    }
    #[test]
    fn test_shift_rocks_west_left() {
        let map = Map {
            data: vec![vec![Rock::R, Rock::E], vec![Rock::E, Rock::R]],
        };
        assert_eq!(
            map.shift_west(),
            Map {
                data: vec![vec![Rock::R, Rock::E], vec![Rock::R, Rock::E]]
            }
        );
    }
    #[test]
    fn test_shift_rocks_south() {
        let map = Map {
            data: vec![vec![Rock::R, Rock::E], vec![Rock::E, Rock::R]],
        };
        assert_eq!(
            map.shift_south(),
            Map {
                data: vec![vec![Rock::E, Rock::E], vec![Rock::R, Rock::R]]
            }
        );
    }
    #[test]
    fn test_shift_rocks_north() {
        let map = Map {
            data: vec![vec![Rock::E, Rock::E], vec![Rock::R, Rock::R]],
        };
        assert_eq!(
            map.shift_north(),
            Map {
                data: vec![vec![Rock::R, Rock::R], vec![Rock::E, Rock::E]]
            }
        );
    }
}
