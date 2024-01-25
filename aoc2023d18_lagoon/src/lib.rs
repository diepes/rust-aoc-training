//use std::{collections::HashMap};

use std::ops::Deref;

pub fn run() {
    let mut instructions = Instructions::new("in.txt", false);
    let mut data = Data::new(&instructions);
    //println!("data: {:?}", data);
    data.print_map();
    println!();
    // for y in 0..2 {
    //     for x in 0..data.map[0].len() {
    //         if data.map[y][x].trench {
    //             println!("x:{x},{y} {:?}", data.map[y][x]);
    //         };
    //     }
    //     println!();
    // }
    // // println!("inst:{:?}", instructions.0[172]);
    // println!();

    // let total_p1 = data.fill();
    // data.print_map();
    // println!("Total_p1: {total_p1} = 62(tst)/53300");

    let total_p2 = instructions.calc_p2(false);
    println!("Total_p2: {total_p2} ");

    //instructions.print_map();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data {
    map: Vec<Vec<Spot>>,
    map_max_x: usize,
    map_max_y: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Spot {
    pos: Pos,
    trench: bool,
    rgb: Option<String>,
    inst_num: usize,
    cross_out_in: bool,
}
impl Data {
    fn fill(&mut self) -> i32 {
        let mut cnt_trench = 0;
        for y in 0..self.map_max_y {
            let mut flag_prev_trench_ch = false;
            let mut prev_trench_id = usize::MAX;
            let mut trench_crossed = 0;
            // walk left to right track if we are inside our outside
            //    #          #      #                                        ##
            // >  #       ####      ##                           > ###  or > ## <
            // 1. #    2. #      3.  #   All cross wall, but not   # #

            for x in 0..self.map_max_x {
                if self.map[y][x].trench {
                    // in trench block
                    cnt_trench += 1;
                    if !flag_prev_trench_ch || prev_trench_id != self.map[y][x].inst_num {
                        if self.map[y][x].cross_out_in {
                            trench_crossed += 1;
                        };
                    };
                    flag_prev_trench_ch = true;
                    prev_trench_id = self.map[y][x].inst_num;
                } else {
                    //flat not trench
                    prev_trench_id = usize::MAX; //reset
                    flag_prev_trench_ch = false;
                    if trench_crossed % 2 == 0 {
                        // even crossings = outside
                    } else {
                        // uneven num crossings trenches = inside
                        cnt_trench += 1;
                        self.map[y][x].trench = true; //display
                    };
                }
            }
        }
        cnt_trench
    }
    fn new(instructions: &Instructions) -> Data {
        let (map_size_x, map_size_y, map_start) = Data::get_map_size_p1(instructions);
        let mut x: isize = map_start.x as isize;
        let mut y: isize = map_start.y as isize;
        // fill data with "."'s
        let mut data: Data = Data {
            map_max_x: (map_size_x + 1) as usize,
            map_max_y: (map_size_y + 1) as usize,
            map: (0..=map_size_y)
                .map(|y| {
                    (0..=map_size_x)
                        .map(|x| Spot {
                            pos: Pos {
                                x: x as usize,
                                y: y as usize,
                            },
                            trench: false,
                            rgb: None,
                            inst_num: 0,
                            cross_out_in: false,
                        })
                        .collect()
                })
                .collect(),
        };
        for (inst_num, entry) in instructions.0.iter().enumerate() {
            let mut dx: isize = 0;
            let mut dy: isize = 0;
            let mut x_start = x;
            let mut y_start = y;
            let mut x_end = x;
            let mut y_end = y;
            let mut cross_out_in = false;

            match entry.direction_part1 {
                Direction::R => {
                    dx = entry.distance_part1 as isize;
                    x_start = x; // all of horizontal same instruction number.
                    x_end = x + dx;
                    x += dx;
                    if entry.direction_previous != entry.direction_next {
                        cross_out_in = false;
                    } else {
                        cross_out_in = true;
                    }
                }
                Direction::L => {
                    dx = entry.distance_part1 as isize;
                    x_start = x - dx;
                    x_end = x; // all of horizontal same instruction number.
                    x -= dx;
                    if entry.direction_previous != entry.direction_next {
                        cross_out_in = false;
                    } else {
                        cross_out_in = true;
                    }
                }
                Direction::D => {
                    dy = entry.distance_part1 as isize;
                    y_start = y + 1;
                    y_end = y + dy - 1; //one short
                    y += dy;
                    cross_out_in = true;
                }
                Direction::U => {
                    dy = entry.distance_part1 as isize;
                    y_start = y - dy + 1; //one short
                    y_end = y - 1;
                    y -= dy;
                    cross_out_in = true;
                }
                _ => panic!("Invalid direction"),
            };
            // update
            println!(
                "#{inst_num} xs:{x_start},ys:{y_start}  ex:{x_end}, ey:{y_end} , cross:{cross_out_in} {:?}",
                x..(x + dx)
            );
            for py in y_start..=y_end {
                for px in x_start..=x_end {
                    if px < 0
                        || py < 0
                        || px >= (data.map_max_x as isize)
                        || py >= (data.map_max_y as isize)
                    {
                        panic!("instructions left the map area !! {px},{py}");
                    };
                    data.map[py as usize][px as usize].trench = true;
                    data.map[py as usize][px as usize].inst_num = inst_num;
                    data.map[py as usize][px as usize].cross_out_in = cross_out_in;
                }
            }
        }
        data
    }

    fn get_map_size_p1(instructions: &Instructions) -> (isize, isize, Pos) {
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut x_max: isize = 0;
        let mut x_min: isize = 0;
        let mut y_max: isize = 0;
        let mut y_min: isize = 0;
        for entry in instructions.0.iter() {
            match entry.direction_part1 {
                Direction::R => x += entry.distance_part1 as isize,
                Direction::L => x -= entry.distance_part1 as isize,
                Direction::D => y += entry.distance_part1 as isize,
                Direction::U => y -= entry.distance_part1 as isize,
                _ => panic!("Invalid direction"),
            };
            if x > x_max {
                x_max = x
            };
            if x < x_min {
                x_min = x
            };
            if y > y_max {
                y_max = y
            };
            if y < y_min {
                y_min = y
            };
        }
        let map_size_x = x_max - x_min;
        let map_size_y = y_max - y_min;
        let map_start = Pos {
            x: (-x_min) as usize,
            y: (-y_min) as usize,
        };
        (map_size_x, map_size_y, map_start)
    }

    fn print_map(&self) {
        for row in &self.map {
            for spot in row {
                if spot.trench {
                    print!("#");
                } else {
                    print!(".");
                };
            }
            println!();
        }
        println!();
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    R,
    L,
    D,
    U,
    NotSet,
}
impl Direction {
    fn new(dir: &str) -> Direction {
        match dir {
            "R" | "0" => Direction::R,
            "D" | "1" => Direction::D,
            "L" | "2" => Direction::L,
            "U" | "3" => Direction::U,
            _ => panic!("Invalid direction '{dir}'"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Instruction {
    start_left_x: i64,
    direction_part2: Direction,
    start_top_y: i64,
    end_bottom_y: i64,
    distance_part2: i64,
    direction_part1: Direction,
    direction_previous: Direction, // segment before this one
    direction_next: Direction,     // segment after this one
    distance_part1: usize,
    rgb: String, //part2 hex distance & dir
}
struct Instructions(Vec<Instruction>);
impl Instructions {
    fn get_map_size_p2(instructions: &Instructions) -> (i64, i64, i64, i64) {
        let mut x = 0;
        let mut y = 0;
        let mut x_max = 0;
        let mut x_min = 0;
        let mut y_max = 0;
        let mut y_min = 0;
        for entry in instructions.0.iter() {
            match entry.direction_part2 {
                Direction::R => x += entry.distance_part2,
                Direction::L => {
                    x -= entry.distance_part2;
                    assert_eq!(
                        entry.start_left_x, x,
                        "L?:{:?} dist:{} dir:{:?}",
                        entry.rgb, entry.distance_part2, entry.direction_part2
                    );
                }
                Direction::D => y += entry.distance_part2,
                Direction::U => y -= entry.distance_part2,
                _ => panic!("Invalid direction"),
            };
            if x > x_max {
                x_max = x
            };
            if x < x_min {
                x_min = x
            };
            if y > y_max {
                y_max = y
            };
            if y < y_min {
                y_min = y
            };
        }
        (x_min, x_max, y_min, y_max)
    }

    fn calc_p2(&mut self, swap: bool) -> i64 {
        let mut cnt_trench: i64 = 0;
        let (x_min, x_max, y_min, y_max) = Instructions::get_map_size_p2(&self);
        let mut active: Vec<Instruction> = vec![];
        println!(
            "calc_p2 loop y_min:{y_min} , y_max:{y_max} = {}",
            y_max - y_min
        );
        // reverse sort smallest y at end of vec ready for pop
        self.0
            .sort_by(|a, b| b.start_top_y.partial_cmp(&a.start_top_y).unwrap());
        println!("sort done start map count ... inst.len()={}", self.0.len());
        // for i in 0..4 {
        //     println!("{i}_pop {:?}\n\n", self.0.pop());
        // }
        let mut y: i64 = y_min;
        while y <= y_max {
            if y % 1000 == 0 {
                println!("y:{y} {}/{}-l:{}  ", y - y_min, y_max - y_min, self.0.len());
            };
            // move instructions to active
            for inst in (0..self.0.len()).rev() {
                if self.0[inst].start_top_y == y {
                    active.push(self.0.pop().unwrap());
                    // print!("[[ pop:{} ]]", active.len());
                } else {
                    break;
                };
            }
            let next_y: i64;
            if self.0.len() > 0 {
                next_y = self.0[self.0.len() - 1].start_top_y; //sorted by y, end of vec is next #y
            } else {
                next_y = y + 1
            };
            while y < next_y {
                //count # horizontal
                // println!("calc_p2 loop x_min:{x_min} , x_max:{x_max}");
                // min 3 e.g. DRU at any level.
                active.retain(|a| a.end_bottom_y >= y);
                assert!(
                    active.len() > 1,
                    "As we calc map size, there should always be >1 active instruction till last y"
                );
                // sort by left_x to use in order
                // println!("Active: {:?}\n", active);
                active.sort(); //use derive Ord
                               //sort_by(|a, b| a.start_left_x.partial_cmp(&b.start_left_x).unwrap());
                               // println!("Active: y:{y} Sorted: {:?}\n", active);
                               // if swap { println!(); };
                let mut outside = true;
                let mut outside_x = x_min;
                let mut a_x = 0; //step through active
                while a_x < active.len() {
                    let mut a_a = &active[a_x]; //sorted to get horizontal first
                    if swap {
                        //debug
                        if outside {
                            print!("{}", ".".repeat((a_a.start_left_x - outside_x) as usize));
                        } else {
                            print!("{}", "#".repeat((a_a.start_left_x - outside_x) as usize));
                        }
                    }
                    match a_a.direction_part2 {
                        Direction::U | Direction::D => {
                            if outside == false {
                                cnt_trench += a_a.start_left_x - outside_x;
                            }
                            outside = !outside; //cross vertical
                            outside_x = a_a.start_left_x + 1;
                            cnt_trench += 1; // Vertical
                            if swap {
                                print!("|");
                            }
                        }
                        Direction::L | Direction::R => {
                            if outside == false {
                                cnt_trench += a_a.start_left_x - outside_x;
                            }
                            outside_x = a_a.start_left_x + a_a.distance_part2 + 1;
                            cnt_trench += a_a.distance_part2 + 1;
                            let a_left_vert = &active[a_x + 1];
                            assert_eq!(
                                a_a.start_left_x, a_left_vert.start_left_x,
                                "Horizontal should meet vertical at start."
                            );
                            let a_right_vert = &active[a_x + 2];
                            assert_eq!(
                                a_a.start_left_x + a_a.distance_part2,
                                a_right_vert.start_left_x,
                                "Horizontal should meet vertical at end."
                            );
                            if a_left_vert.direction_part2 != a_right_vert.direction_part2 {
                                // U turn, no change in outside inside
                                if swap {
                                    print!("{}U", "#".repeat((a_a.distance_part2) as usize));
                                }
                            } else {
                                // we crossed vert line
                                outside = !outside;
                                if swap {
                                    print!("{}", "#".repeat((a_a.distance_part2 + 1) as usize));
                                }
                            };

                            a_x += 2; // used verticals
                        }
                        _ => panic!("Err?"),
                    }
                    a_x += 1;
                }
                y += 1;
                if swap {
                    println!(" << {y:2} {cnt_trench:2} {outside:?}");
                };
            }
        }
        cnt_trench
    }

    fn new(file_name: &str, swap: bool) -> Instructions {
        let input = std::fs::read_to_string(file_name).expect("Missing file ?");
        let mut end_x: i64 = 0; // start of next instruction
        let mut end_y: i64 = 0;
        let mut instructions: Instructions = Instructions(
            input
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|line| {
                    // println!("line: {:?}",line);
                    let mut lsplit = line.split(" ");
                    // println!("lsplit: {:?}",lsplit);
                    let direction_part1 =
                        Direction::new(&(lsplit.next().expect("Instruct 1").to_string()));
                    let distance_part1 = lsplit
                        .next()
                        .expect("Instruct 2")
                        .parse()
                        .expect("Instruct 2 not number?");
                    let rgb = lsplit.next().expect("Instruct 3").to_string();
                    // println!("Instruction {rgb} {}", rgb.len());
                    // println!("Instructon dist:{} dir:{}", &rgb[2..7], &rgb[7..8]);
                    let distance_part2 = if swap {
                        distance_part1 as i64
                    } else {
                        i64::from_str_radix(&rgb[2..7], 16).expect("Hex to num :(")
                    };
                    let direction_part2 = if swap {
                        direction_part1.clone()
                    } else {
                        Direction::new(&rgb[7..8])
                    };
                    // println!("Instruction {rgb} dist:{distance_part2} dir:{direction_part2:?} ");
                    let start_left_x: i64;
                    let start_top_y: i64;
                    let end_bottom_y: i64;
                    match direction_part2 {
                        Direction::R | Direction::D => {
                            start_left_x = end_x;
                            start_top_y = end_y;
                            if direction_part2 == Direction::R {
                                end_x += distance_part2;
                                end_bottom_y = start_top_y;
                            } else {
                                end_y += distance_part2;
                                end_bottom_y = end_y;
                            }
                        }
                        Direction::L => {
                            end_x -= distance_part2;
                            start_left_x = end_x;
                            start_top_y = end_y;
                            end_bottom_y = start_top_y;
                        }
                        Direction::U => {
                            end_bottom_y = end_y;
                            end_y -= distance_part2;
                            start_top_y = end_y;
                            start_left_x = end_x;
                        }
                        _ => panic!("Invalid direction"),
                    };

                    Instruction {
                        direction_part1,
                        distance_part1,
                        direction_next: Direction::NotSet,
                        direction_previous: Direction::NotSet,
                        // e.g. rgb="(#12345D)"
                        rgb,
                        distance_part2,
                        direction_part2,
                        start_left_x,
                        start_top_y,
                        end_bottom_y,
                    }
                })
                .collect(),
        );
        // loop through instructions and set next and previous directions.
        let mut temp_direction_previous = Direction::NotSet;
        for instruction in &mut instructions.0 {
            if temp_direction_previous != Direction::NotSet {
                // only true from 2nd entry
                instruction.direction_previous = temp_direction_previous;
            };
            temp_direction_previous = instruction.direction_part1.clone();
        }
        instructions.0[0].direction_previous = temp_direction_previous;
        // loop backwards for next directions
        let inst_len = instructions.0.len(); //Instructions is tuple struct
        let mut temp_direction_next = Direction::NotSet;
        for n in (0..inst_len).rev() {
            let instruction = &mut instructions.0[n];
            if temp_direction_next != Direction::NotSet {
                // only true from 2nd entry
                instruction.direction_next = temp_direction_next;
            };
            temp_direction_next = instruction.direction_part1.clone();
        }
        instructions.0[inst_len - 1].direction_next = temp_direction_next;

        // return
        instructions
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {}
}
