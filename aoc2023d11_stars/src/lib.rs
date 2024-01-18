use std::collections::HashMap;

pub fn run(input: &str, expansion_ratio: usize) {
    println!("{input}");
    let mut space = Galaxys::new(&input);
    println!(
        "\n x:{:?} \n\n y:{:?} \n\n coords:{:?}",
        space.empty_x, space.empty_y, space.coords,
    );
    // expand
    space.expand_empty_planes(expansion_ratio);
    println!(
        "\n x:{:?} \n\n y:{:?} \n\n coords:{:?}",
        space.empty_x, space.empty_y, space.coords,
    );

    println!("distance {}", space.get_distance_between_all());
}

#[derive(Debug, Clone)]
struct Galaxys {
    coords: Vec<Coord>,
    max: usize,
    empty_x: HashMap<usize, usize>,
    empty_y: HashMap<usize, usize>,
}
impl Galaxys {
    fn new(input: &str) -> Galaxys {
        let mut coords: Vec<Coord> = vec![];
        let mut empty_x: HashMap<usize, usize> = HashMap::default();
        let mut empty_y: HashMap<usize, usize> = HashMap::default();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in input.lines().enumerate() {
            max_y = y;
            for (x, ch) in line.chars().enumerate() {
                max_x = x;
                match ch {
                    '#' => {
                        *empty_y.entry(y).or_default() += 1;
                        *empty_x.entry(x).or_default() += 1;
                        coords.push(Coord {
                            x,
                            y,
                            expanded_x: false,
                            expanded_y: false,
                        });
                    }
                    '.' => {}
                    _ => panic!("input had invalid char '{ch} at {x},{y} !"),
                }
            }
        }
        let max = if max_x > max_y { max_x } else { max_y };
        Galaxys {
            coords,
            max,
            empty_x,
            empty_y,
        }
    }

    fn expand_empty_planes(&mut self, expansion_ratio: usize) {
        //expand galaxys
        let mut x_expand = 0; //cnt # of empty expansions crossed
        let mut y_expand = 0;
        assert!(
            expansion_ratio > 1,
            "Universe can't expand with ration < 2 !"
        );
        for pos in 0..=self.max {
            //walk diagonaly through map and expand
            assert!(
                (x_expand <= pos) & (y_expand <= pos),
                "Impossible expansion ? x_expand={x_expand},{y_expand} pos={pos}"
            );
            //expand x
            let total_galaxys_in_this_xplane = self.empty_x.get(&pos).unwrap_or(&0).clone();
            x_expand += if total_galaxys_in_this_xplane > 0 {
                0
            } else {
                1
            };
            //expand y
            let total_galaxys_in_this_yplane = self.empty_y.get(&pos).unwrap_or(&0).clone();
            y_expand += if total_galaxys_in_this_yplane > 0 {
                0
            } else {
                1
            };
            // got expansion cnt up to this pos
            // loop through coords and adjust any at current x,y
            for coord in &mut self.coords {
                if coord.expanded_x == false && coord.x == pos && x_expand > 0 {
                    assert_ne!(
                        total_galaxys_in_this_xplane ,0,
                        "ERR: Cant have empty plane x:{pos} and galaxy:#:{coord:?} at same plane ? ex:{total_galaxys_in_this_xplane} {} {:?}",total_galaxys_in_this_xplane>0,self.empty_x
                    );
                    coord.x = (coord.x - x_expand) + x_expand * expansion_ratio;
                    coord.expanded_x = true; //only one expansion.
                };
                //
                if coord.expanded_y == false && coord.y == pos && y_expand > 0 {
                    assert_ne!(
                        total_galaxys_in_this_yplane, 0,
                        "ERR: Cant have empty plane y:{pos} and galaxy:#:{coord:?} at same plane ? ey:{total_galaxys_in_this_yplane} {:?}",self.empty_y
                    );
                    coord.y = (coord.y - y_expand) + y_expand * expansion_ratio;
                    coord.expanded_y = true;
                };
            }
        }
    }

    fn get_distance_between_all(&self) -> usize {
        let len = self.coords.len();
        assert!(len > 1, "Need atleast two coords to calculate steps. ");
        let mut sum = 0;
        let mut num_of_comparisons = 0;
        for coord1 in 0..len - 1 {
            for coord2 in coord1 + 1..len {
                let xd = abs_difference(self.coords[coord1].x, self.coords[coord2].x);
                let yd = abs_difference(self.coords[coord1].y, self.coords[coord2].y);
                let steps = steps_between(xd, yd);
                num_of_comparisons += 1;
                println!(
                    "{len} {max} {steps:>2} coords:{coord1}:{coord2} x:{},y:{}  x:{},y:{}",
                    self.coords[coord1].x,
                    self.coords[coord1].y,
                    self.coords[coord2].x,
                    self.coords[coord2].y,
                    max = self.max
                );
                sum += steps;
            }
        }
        println!(" num_of_distances: {num_of_comparisons} sum: {sum}");
        sum
    }
}
fn steps_between(xd: usize, yd: usize) -> usize {
    // 2 x shortest of x or y, + difference
    if xd > yd {
        (xd - yd) + yd * 2
    } else {
        (yd - xd) + xd * 2
    }
}
fn abs_difference(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    expanded_x: bool,
    expanded_y: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_between() {
        assert_eq!(steps_between(0, 0), 0);
        assert_eq!(steps_between(1, 0), 1);
        assert_eq!(steps_between(0, 1), 1);
        assert_eq!(steps_between(1, 1), 2);
        assert_eq!(steps_between(1, 3), 4);
        assert_eq!(steps_between(5, 10), 15);
        assert_eq!(
            steps_between(abs_difference(5, 0), abs_difference(11, 1)),
            15
        );
    }
    #[test]
    fn test_get_distance_between_all() {
        let mut space = Galaxys {
            coords: vec![
                Coord {
                    x: 2,
                    y: 2,
                    expanded_x: false,
                    expanded_y: false,
                },
                Coord {
                    x: 5,
                    y: 6,
                    expanded_x: false,
                    expanded_y: false,
                },
            ],
            max: 10,
            empty_x: HashMap::from([(2, 1), (5, 1)]), //x (coord, count)
            empty_y: HashMap::from([(2, 1), (6, 1)]), //y
        };
        assert_eq!(space.get_distance_between_all(), 7);
        let mut space2 = space.clone();
        space2.expand_empty_planes(2);
        assert_eq!(space2.get_distance_between_all(), 12);
        space.expand_empty_planes(1_000_000);
        assert_eq!(space.get_distance_between_all(), 5_000_002); // dx=5-2-3 dy=6-2=4 ex=2exp(between 2 and 5)
    }
    #[test]
    fn test_space_expand() {
        let mut space = Galaxys {
            coords: vec![
                Coord {
                    x: 2,
                    y: 2,
                    expanded_x: false,
                    expanded_y: false,
                },
                Coord {
                    x: 10,
                    y: 9,
                    expanded_x: false,
                    expanded_y: false,
                },
            ],
            max: 11,
            empty_x: HashMap::from([(2, 99), (3, 99), (10, 1)]), //x (coord, count)
            empty_y: HashMap::from([(0, 2), (2, 99), (9, 1)]),   //y
        };
        let mut space2 = space.clone();
        //   ##
        // .........#
        // ..........
        // ..#......#
        // ..........
        // check before expansion.
        assert_eq!(
            space.coords[0],
            Coord {
                x: 2,
                y: 2,
                expanded_x: false,
                expanded_y: false
            }
        );
        //check after expansion
        space.expand_empty_planes(2);
        assert_eq!(
            space.coords[0],
            Coord {
                x: 4, // (2 -2 exp) + 2exp * 2 = 4
                y: 3, // (2 -1exp) + 1exp * 2 = 3
                expanded_x: true,
                expanded_y: true,
            }
        );
        space2.expand_empty_planes(1_000_000);
        assert_eq!(
            space2.coords[0],
            Coord {
                x: 2_000_000, // (2 -2exp) + 2exp * 1_000_000
                y: 1_000_001,
                expanded_x: true,
                expanded_y: true,
            }
        );
        //check 2nd expansion should cause no change.
        space.expand_empty_planes(2);
        assert_eq!(
            space.coords[0],
            Coord {
                x: 4,
                y: 3,
                expanded_x: true,
                expanded_y: true,
            }
        );
        // test galaxy at edge one coord = max
        // [1] (10,9)
        assert_eq!(
            space.coords[1],
            Coord {
                x: 18, // 10 + 8 exp = (10 - 8 ) + 8 * 2
                y: 16, // 9 + 7 exp = ( 9 -7 ) + 7 * 2
                expanded_x: true,
                expanded_y: true,
            }
        );
        assert_eq!(
            space2.coords[1],
            Coord {
                x: 8_000_002, //(10 - 8) + 8 * 1_000_000 empty
                y: 7_000_002, //(9-7) + 7 * 1_000_000
                expanded_x: true,
                expanded_y: true,
            }
        );
    }
}
