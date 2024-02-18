use std::collections::HashMap;

pub fn run() {
    println!("lib.rs");
    let data_empty = Data::parse("in.txt");
    let mut data = data_empty.clone();
    data.update_spot(data.start_y, data.start_x, 0, 0, Plot::Person, 0); // 1 Person at start pos.
                                                                         // data.print_map(&data_empty, 0);
    let num_steps = 26_501_365;
    let remove_inside = false;
    for step in 1..=num_steps {
        data.take_step(step, remove_inside);
        if step < 132 * 4 || (step % 1000 == 0) {
            let map_size = 16;
            let map_start_top_left = Point {
                x: data.start_x - map_size + 5 + (step as i64),
                y: data.start_y - map_size / 2,
            };
            // data.print_map(&data_empty, step);
            data.calc_full_maps(step);
            // data.print_map_spot(&data_empty, step, map_start_top_left, map_size);
        }
    }
    println!(
        "Count {} Steps:{num_steps}  (64 stpes = 3847 part1)",
        data.count_person(num_steps),
    );
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:>2},{:>2}]", self.x, self.y)
    }
}
#[derive(Debug, Clone, PartialEq)]
struct PersonHistory {
    // present: usize,
    // permanent: bool, //remove and count, not changing
    on_even: bool, // person on for even or odd
    first_arrival: u64,
    distance_xy: u64,
}
impl PersonHistory {
    fn present(&self, step: u64) -> bool {
        // check if step off or even and when person is present
        let step_even = if step % 2 == 0 { true } else { false };
        // println!(
        //     "Debug present step:{step}>>step_even:{step_even} on_even:{}  result:{} dist:{}",
        //     self.on_even,
        //     self.on_even == step_even,
        //     self.distance_xy,
        // );
        self.on_even == step_even // on if on_even and step_even both true or both false.
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Plot {
    Rock,
    Garden,
    Person,
}
#[derive(Debug, Clone)]
struct Data {
    map: Vec<Vec<Plot>>,
    people: HashMap<Point, PersonHistory>,
    people_permanent_odd: u64,
    people_permanent_even: u64,
    start_x: i64,
    start_y: i64,
    max: Point, // same as max below but i64 of base map
    max_x: usize,
    max_y: usize,
}
impl Data {
    fn count_person(&self, step: u64) -> u64 {
        let mut cnt = 0;
        for (_point, person) in self.people.iter() {
            if person.present(step) {
                cnt += 1;
            }
        }
        cnt + if step % 2 == 0 {
            self.people_permanent_even
        } else {
            self.people_permanent_odd
        }
    }
    fn calc_steps_to_corner_far(&self, map_loc: &Point) -> i64 {
        self.max.x - 1 + self.max.x * (map_loc.x.abs() + map_loc.y.abs())
    }
    fn calc_map_fully_inclosed(&self, step: u64, map_num: &Point) -> bool {
        let map_x = map_num.x.abs() as u64;
        let map_y = map_num.y.abs() as u64;
        if map_x == 0 || map_y == 0 {
            // + horizontal or vertical
            if step > ((map_x + map_y) * (self.max_x as u64) + self.start_x as u64) {
                return true;
            } else {
                return false;
            };
        }
        if step > self.calc_steps_to_corner_far(map_num) as u64 {
            return true;
        } else {
            return false;
        }
    }

    fn count_person_map(&self, step_odd_even: u64, map_num: &Point) -> u64 {
        let mut cnt = 0;
        let map_start_x = map_num.x * self.max.x;
        let map_start_y = map_num.y * self.max.y;
        let map_max_x = (map_num.x + 1) * self.max.x;
        let map_max_y = (map_num.y + 1) * self.max.y;
        for (point, person) in self.people.iter() {
            if point.x >= map_start_x
                && point.y >= map_start_y
                && point.x < map_max_x
                && point.y < map_max_y
            {
                if person.present(step_odd_even) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn take_step(&mut self, step: u64, remove_inside: bool) {
        let mut update_points: Vec<Point> = vec![];
        for (point, person) in &self.people {
            if person.present(step - 1) {
                update_points.push(point.clone());
            };
        }
        assert!(
            update_points.len() > 0,
            "No update points for step: {step} ?"
        );

        for Point { x: px, y: py } in update_points.iter() {
            // move in 4 dir
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if dx < 0 && px > &self.start_x {
                    continue;
                };
                if dx > 0 && px < &self.start_x {
                    continue;
                };
                if dy < 0 && py > &self.start_y {
                    continue;
                };
                if dy > 0 && py < &self.start_y {
                    continue;
                };
                self.update_spot(*px, *py, dx, dy, Plot::Person, step);
            }
        }

        if remove_inside {
            // cleanup and implement permanent counter to remove people/spots not changing.
            self.people.retain(|point, person| {
                // update just sets counter PersonHistory::present
                if person.distance_xy + 1 < step {
                    //remove person and add to permanent count
                    // println!(
                    //     "take_step: remove {point:?} dist:{distance} on_even:{on_even} present:{present}",
                    //     point = point,
                    //     distance = person.distance_xy,
                    //     on_even = person.on_even,
                    //     present = person.present(step),
                    // );
                    if person.present(step) {
                        self.people_permanent_even += 1;
                    } else {
                        self.people_permanent_odd += 1;
                    }
                    // remove, return false to retain.
                    false
                } else {
                    true
                }
            }); // end retain
        }
    }

    /// update_spot only inc/dec PersonHistory::present +1 Person -1 Garden
    /// cleanup is done in take_step.
    fn translate_to_map(&self, point: Point) -> (usize, usize) {
        let mx_i = point.x % self.max.x;
        let mx = if mx_i < 0 {
            (mx_i + self.max.x) as usize
        } else {
            mx_i as usize
        };
        let my_i = point.y % self.max.y;
        let my = if my_i < 0 {
            (my_i + self.max.y) as usize
        } else {
            my_i as usize
        };
        (mx, my)
    }
    fn update_spot(
        &mut self,
        x: i64,
        y: i64,
        delta_x: i64,
        delta_y: i64,
        update_plot_type: Plot,
        step: u64,
    ) {
        assert_ne!(update_plot_type, Plot::Rock, "Can't move/change a Rock's");
        let nx = x + delta_x;
        let ny = y + delta_y;
        // translate from global nx,ny to map mx,my
        let (mx, my) = self.translate_to_map(Point { x: nx, y: ny });
        // assert mx and my in small map
        if mx >= self.max_x || my >= self.max_y {
            panic!(
                " mx:{mx} my:{my} max.x:{max_x} max.y:{max_y} max:{max:?} out of bounds!",
                max = self.max,
                max_x = self.max.x,
                max_y = self.max.y,
            );
        };
        if self.map[my][mx] == Plot::Rock {
            // skip rock, confirm no person at this coord.
            assert_eq!(
                self.people.get(&Point { x: nx, y: ny }),
                None,
                "Cant have a person at a rock position"
            );
        } else if update_plot_type == Plot::Garden {
            let person: &PersonHistory = self
                .people
                .get(&Point { x: nx, y: ny })
                .expect("Only existing person should change back to garden");
            //     println!("Update to Garden update_spot x:{nx},y:{ny} step:{step} on_even:{} present:{} {} {}",
            //     person.on_even,
            //     person.present(step),
            //     self.start_x,
            //     self.start_y,
            // );
            assert_eq!(
                person.present(step),
                false,
                "update_spot x:{nx},y:{ny} step:{step} on_even:{} present:{} {} {}",
                person.on_even,
                person.present(step),
                self.start_x,
                self.start_y,
            );
        } else {
            // Update to Person at spot
            // get person at this possion, or create a new one.
            let person: &mut PersonHistory = self
                .people
                .entry(Point { x: nx, y: ny })
                .or_insert_with(|| {
                    PersonHistory {
                        // present: 0, move to impl function
                        on_even: if step % 2 == 0 { true } else { false },
                        first_arrival: step,
                        distance_xy: ((nx - self.start_x).abs() + (ny - self.start_y).abs()) as u64,
                    }
                });
            assert_eq!(
                person.present(step),
                true,
                "update_spot step:{step} on_even:{} ",
                person.on_even
            );
        }
    }

    fn calc_full_maps(&self, step: u64) {
        assert_eq!(self.max_x, self.max_y, "Assumed map is square");
        assert_eq!(
            self.start_x, self.start_y,
            "Assumed map S start is same x = y"
        );
        assert_eq!(
            self.start_x * 2 + 1,
            self.max.x,
            "Assumed start possition is in center of map, same distance to each side"
        );
        let steps_to_edge_1 = self.start_x;
        let steps_to_full = self.max.x;
        let steps_to_edge = |n| -> i64 { self.start_x + n * self.max.x };
        assert_eq!(steps_to_edge_1, steps_to_edge(0));
        let steps_to_corner_far = |n| -> i64 { self.max.x - 1 + n * self.max.x };
        let steps_to_corner_first = |n| -> i64 {
            if n == 0 {
                panic! {"not center block"}
            };
            n * self.max.x
        };
        let map_to_count = Point { x: 1, y: 1 };
        let count_person_map = self.count_person_map(step, &map_to_count);
        println!(
            "calc..: steps_to.. edge:{edge} corner_far(0):{corner_far}, corner_first(1):{corner_first}  step:{step} count_map:{count_person_map}",
            edge = steps_to_edge(0),
            corner_far = steps_to_corner_far(0),
            corner_first = steps_to_corner_first(1),
        );
        for x in -1..=1 {
            for y in -1..=1 {
                let map_to_count = Point { x: x, y: y };
                println!(
                    "calc_full_maps map:{map_to_count} count_map:{count_person_map:>4} inclosed:{inclosed:>5}  {step}/{steps_to_corner_far}",
                    map_to_count = &map_to_count,
                    count_person_map = self.count_person_map(step, &map_to_count),
                    inclosed = self.calc_map_fully_inclosed(step, &map_to_count),
                    steps_to_corner_far= self.calc_steps_to_corner_far(&map_to_count),
                )
            }
        }
    }
    fn print_map_spot(&self, empty: &Data, step: u64, start: Point, size: i64) {
        let count_person = self.count_person(step);
        let map_to_count = Point { x: 1, y: 0 };
        let count_person_map = self.count_person_map(step, &map_to_count);
        println!("⬇️ Step: {step} count_pers:{count_person} count_map:{count_person_map}");
        // restrict to base map
        for y in start.y..start.y + size {
            for x in start.x..start.x + size {
                let mut person_present = false;
                if let Some(person) = self.people.get(&Point { x, y }) {
                    person_present = person.present(step);
                };
                let (mx, my) = self.translate_to_map(Point { x, y });
                if x == self.start_x && y == self.start_y {
                    if person_present {
                        print!("S");
                    } else {
                        print!("s");
                    }
                } else {
                    print!(
                        "{}",
                        match (&empty.map[my][mx], person_present) {
                            (_, true) => 'O',
                            (Plot::Rock, false) => '#',
                            (Plot::Garden, _) => '.',
                            _ => panic!(
                                "! invalid combination ?? point:{:?} {person_present}",
                                empty.map[my][mx]
                            ),
                        }
                    );
                }
            }
            println!();
        }
        println!("⬆️ Step: {step} count_pers:{count_person} count_map:{count_person_map} people.len():{num_people}",
        num_people = self.people.len());
        println!()
    }

    fn print_map(&self, empty: &Data, step: u64) {
        let count_person = self.count_person(step);
        let count_person_map = self.count_person_map(step, &Point { x: 0, y: 0 });
        println!("⬇️ Step: {step} count_pers:{count_person} count_map:{count_person_map}");
        // restrict to base map
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let mut person_present = false;
                if let Some(person) = self.people.get(&Point {
                    x: x as i64,
                    y: y as i64,
                }) {
                    //let step_even = if step % 2 == 0 { true } else { false };
                    //person_present = person.on_even == step_even; // on if on_even and step_even both true or both false.
                    person_present = person.present(step);
                };
                //if point.x > 0 && point.y > 0 && point.x < self.max.x && point.y < self.max.y {
                if x == self.start_x as usize && y == self.start_y as usize {
                    if person_present {
                        print!("S");
                    } else {
                        print!("s");
                    }
                } else {
                    print!(
                        "{}",
                        match (&empty.map[y][x], person_present) {
                            (_, true) => 'O',
                            (Plot::Rock, false) => '#',
                            (Plot::Garden, _) => '.',
                            _ => panic!(
                                "! invalid combination ?? point:{:?} {person_present}",
                                empty.map[y][x]
                            ),
                        }
                    );
                }
            }
            println!();
        }
        println!("⬆️ Step: {step} count_pers:{count_person} count_map:{count_person_map} people.len():{num_people} , start:{start:?} size:{size:?}",
        num_people = self.people.len(),
        start=Point{x:self.start_x, y:self.start_y},
        size=self.max,
    );
        println!()
    }

    fn parse(file_name: &str) -> Data {
        let input = std::fs::read_to_string(file_name).expect("Missing file?");
        let map: Vec<Vec<Plot>>;
        let mut start_x = 0;
        let mut start_y = 0;
        map = input
            .split("\n")
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '#' => Plot::Rock,
                        '.' => Plot::Garden,
                        'S' => {
                            start_x = x as i64;
                            start_y = y as i64;
                            Plot::Garden //Empty map
                        }
                        _ => panic!("Illegal plot type '{ch}'"),
                    })
                    .collect()
            })
            .collect();

        Data {
            max_x: map[0].len(),
            max_y: map.len(),
            max: Point {
                x: map[0].len() as i64,
                y: map.len() as i64,
            },
            map,
            start_x,
            start_y,
            people: HashMap::new(),
            people_permanent_odd: 0,
            people_permanent_even: 0,
        }
    }
}
