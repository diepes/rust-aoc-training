pub fn run() {
    let mut map = read_data("in.txt");
    //println!("{:?}", map);
    let max_x = map.d[0].len();
    let max_y = map.d.len();
    let mut max_e = 0;
    // entry top bottom
    for x in 0..max_x {
        let e = calc_energize_with_entry(&map, x, 0, Direction::S);
        if e > max_e {
            max_e = e;
        };
        let e = calc_energize_with_entry(&map, x, max_y - 1, Direction::N);
        if e > max_e {
            max_e = e;
        };
    }
    for y in 0..max_y {
        let e = calc_energize_with_entry(&map, 0, y, Direction::E);
        if e > max_e {
            max_e = e;
        };
        let e = calc_energize_with_entry(&map, max_x - 1, y, Direction::W);
        if e > max_e {
            max_e = e;
        };
    }
    println!("Max_e: {max_e}");
}
fn calc_energize_with_entry(
    map_original: &Data,
    entry_x: usize,
    entry_y: usize,
    direction: Direction,
) -> usize {
    let mut light_beams = LightBeams::new(entry_x, entry_y, direction.clone());
    let mut map = map_original.clone();
    let mut cnt_stable_cycles = 0;
    let mut cnt_energized_before = 0;
    let mut i = 0;
    while light_beams.beams.len() > 0 {
        i += 1;
        light_beams.fly(&mut map);
        let (s, st) = map.count_energized();
        // println!(
        //     "{i} >> light_beams start beams:{} ⚡️s:{s} ⚡️st:{st}",
        //     light_beams.beams.len()
        // );
        // if light_beams.beams.len() == 0 {
        //     println!("{i} Stop light_beams cnt: {}", light_beams.beams.len());
        //     break;
        // };
        if cnt_energized_before == s {
            cnt_stable_cycles += 1;
        } else {
            cnt_energized_before = s;
            cnt_stable_cycles = 0;
            //map.print_map();
        };
        if cnt_stable_cycles > 100 {
            println!("Break stable count for 1000");
            break;
        }
    }
    //map.print_map();
    let (s, st) = map.count_energized();
    println!(" Energized map spots spots: {s} spot_total:{st} entry_x:{entry_x} entry_y:{entry_y} dir:{direction:?}");
    s
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    //Map
    d: Vec<Vec<Spot>>,
    mx: usize,
    my: usize,
}
impl Data {
    //Map
    fn new() -> Data {
        Data {
            d: vec![],
            mx: 0,
            my: 0,
        }
    }
    fn print_map(&self) {
        for row in &self.d {
            for spot in row {
                if spot.energized > 0 {
                    print!("E");
                } else {
                    print!("{}", spot.ch)
                }
            }
            println!();
        }
        let (s, st) = self.count_energized();
        println!(" Energized map spots spots: {s} spot_total:{st}");
        println!();
    }
    fn count_energized(&self) -> (usize, usize) {
        let mut cnt = 0;
        let mut cnt_total = 0;
        for y in 0..self.d.len() {
            for x in 0..self.d[0].len() {
                let v = self.d[y][x].energized;
                if v > 0 {
                    cnt += 1;
                    cnt_total += v;
                }
            }
        }
        (cnt, cnt_total)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Spot {
    ch: char,
    t: Types,
    energized: usize,
    seen_light_going: Vec<Direction>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Types {
    Empty,
    SplitV,
    SplitH,
    MirrorB,
    MirrorF,
}
impl Types {
    fn get_char(&self) -> char {
        match &self {
            Types::Empty => '.',
            Types::SplitV => '|',
            Types::SplitH => '-',
            Types::MirrorB => '\\',
            Types::MirrorF => '/',
        }
    }
    fn get_type(ch: char) -> Types {
        match ch {
            '.' => Types::Empty,
            '|' => Types::SplitV,
            '-' => Types::SplitH,
            '\\' => Types::MirrorB,
            '/' => Types::MirrorF,
            _ => panic!("Invalid char {ch} ?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LightBeam {
    pos_x: usize,
    pos_y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LightBeams {
    beams: Vec<LightBeam>,
}
impl LightBeams {
    fn new(x: usize, y: usize, direction: Direction) -> LightBeams {
        // start at top left corner
        LightBeams {
            beams: vec![LightBeam {
                pos_x: x,
                pos_y: y,
                direction: direction,
            }],
        }
    }
    fn update_beam_pos(
        &mut self,
        beam_id: usize,
        xd: isize,
        yd: isize,
        map: &mut Data,
        direction: Direction,
    ) {
        let max_x = map.d[0].len();
        let max_y = map.d.len();
        let b_x = self.beams[beam_id].pos_x;
        let b_y = self.beams[beam_id].pos_y;
        let n_x = (b_x as isize) + xd;
        let n_y = (b_y as isize) + yd;
        // if beam has been through this possion before in same direction, delete it.
        let seen_light = &map.d[b_y][b_x].seen_light_going;
        if seen_light.iter().any(|dir| dir == &direction) {
            // seen beam at this spot going this direction
            self.beams.remove(beam_id);
            return;
        } else {
            map.d[b_y][b_x].seen_light_going.push(direction.clone());
        };

        if n_x < 0 || n_y < 0 || n_x >= (max_x as isize) || n_y >= (max_y as isize) {
            // //delete beam out of bounds
            self.beams.remove(beam_id);
        } else {
            self.beams[beam_id].pos_x = n_x as usize;
            self.beams[beam_id].pos_y = n_y as usize;
            self.beams[beam_id].direction = direction;
        }
    }
    fn fly(&mut self, map: &mut Data) {
        // shoot beams through map 1 step
        let max_x = map.d[0].len();
        let max_y = map.d.len();
        for b_i in (0..self.beams.len()).rev() {
            //high to low so we can delete them
            let b_x = self.beams[b_i].pos_x;
            let b_y = self.beams[b_i].pos_y;
            let m = &mut map.d[b_y][b_x];
            // energize current map pos
            m.energized += 1;
            //let mut x = b_x + 1;
            //let mut y = b_y + 1;
            match (self.beams[b_i].direction.clone(), m.t.clone()) {
                //right
                (Direction::E, Types::SplitH | Types::Empty)
                | (Direction::N, Types::MirrorF)
                | (Direction::S, Types::MirrorB) => {
                    self.update_beam_pos(b_i, 1, 0, map, Direction::E);
                    //b_x += 1;
                }
                //left
                (Direction::W, Types::SplitH | Types::Empty)
                | (Direction::N, Types::MirrorB)
                | (Direction::S, Types::MirrorF) => {
                    self.update_beam_pos(b_i, -1, 0, map, Direction::W);
                }
                //up
                (Direction::N, Types::SplitV | Types::Empty)
                | (Direction::E, Types::MirrorF)
                | (Direction::W, Types::MirrorB) => {
                    self.update_beam_pos(b_i, 0, -1, map, Direction::N);
                }
                //down
                (Direction::S, Types::SplitV | Types::Empty)
                | (Direction::E, Types::MirrorB)
                | (Direction::W, Types::MirrorF) => {
                    self.update_beam_pos(b_i, 0, 1, map, Direction::S);
                }
                // Split Vertical
                (Direction::E | Direction::W, Types::SplitV) => {
                    // beam split into two.
                    if b_y > 0 {
                        // add up
                        self.beams.push(LightBeam {
                            pos_x: b_x,
                            pos_y: b_y.clone() - 1,
                            direction: Direction::N,
                        })
                    };
                    // set direction first as update might delete beam if out of bounds.
                    self.update_beam_pos(b_i, 0, 1, map, Direction::S);
                }
                // Split Horizontal
                (Direction::N | Direction::S, Types::SplitH) => {
                    // beam split into two.
                    if b_x > 0 {
                        // add up
                        self.beams.push(LightBeam {
                            pos_x: b_x.clone() - 1,
                            pos_y: b_y,
                            direction: Direction::W,
                        })
                    };
                    self.update_beam_pos(b_i, 1, 0, map, Direction::E);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}
fn read_data(file_name: &str) -> Data {
    let input = std::fs::read_to_string(file_name).expect("Missing file ?");
    let mut data: Data = Data::new();
    // let mut total = 0;
    for d in input.split("\n") {
        let row: Vec<Spot> = d
            .chars()
            .map(|ch| Spot {
                ch: ch,
                t: Types::get_type(ch),
                energized: 0,
                seen_light_going: vec![],
            })
            .collect();
        data.d.push(row);
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {}
}
