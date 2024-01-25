use std::collections::HashMap;

pub fn run() {
    let mut data = read_data("in.txt");
    //let rule_step_min = 1;
    //let rule_step_max = 3;
    let rule_step_min = 4;
    let rule_step_max = 10;
    data.print_map();
    data.find_route(rule_step_min, rule_step_max);
    println!(
        "Result last spot:: {:?}",
        data.map[data.map_max_y - 1][data.map_max_x - 1]
    );
    for p in data.map[data.map_max_y - 1][data.map_max_x - 1]
        .best_paths
        .values()
    {
        println!(
            "[870] hm {:?} {:?} rl:{} \n",
            p.total_cost,
            p.illegal_dir,
            p.route_taken.len(),
            // p.route_taken,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Route {
    pos: Pos,
    total_cost: u64,
    illegal_dir: Direction,
    route_taken: Vec<(Pos, u64, u64)>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Data {
    map: Vec<Vec<Spot>>,
    routes: Vec<Route>,
    map_max_x: usize,
    map_max_y: usize,
}
impl Data {
    fn find_route(&mut self, smin: usize, smax: usize) {
        let mut i = 0;
        while self.routes.len() > 0 {
            i += 1;
            if i % 1 == 0 {
                println!("{i} route cnt {}", self.routes.len());
            };
            // loop through routes from high to low so we can remove them.
            for r_i in (0..self.routes.len()).rev() {
                // 1 check if best here, update map or delete route
                let r = self.routes.remove(r_i);
                if let Some(prev_route) = self.map[r.pos.y][r.pos.x].best_paths.get(&r.illegal_dir)
                {
                    // route has been at this node before
                    if prev_route.total_cost <= r.total_cost {
                        // we can't do better.
                        // println!(
                        //     " kill route x{} y{}  prev:{} our_cost:{}",
                        //     r.pos.x, r.pos.y, prev_route.total_cost,r.total_cost
                        // );
                        continue;
                    }
                };
                // first / best route & dir at this node, add to hashmap
                self.map[r.pos.y][r.pos.x]
                    .best_paths
                    .insert(r.illegal_dir.clone(), r.clone());
                // now create new routes from here
                for offset in smin..=smax {
                    if r.illegal_dir != Direction::Horizontal {
                        if r.pos.x + offset < self.map_max_x {
                            let illegal_dir = Direction::Horizontal;
                            let cooldown_cost: u64 = (r.pos.x + 1..=r.pos.x + offset)
                                .map(|x| self.map[r.pos.y][x].cost)
                                .sum();
                            self.add_route(
                                &r,
                                Pos {
                                    x: r.pos.x + offset,
                                    y: r.pos.y,
                                },
                                cooldown_cost,
                                illegal_dir,
                            );
                        };
                        if offset <= r.pos.x {
                            let illegal_dir = Direction::Horizontal;
                            let cooldown_cost: u64 = (r.pos.x - offset..=r.pos.x - 1)
                                .map(|x| self.map[r.pos.y][x].cost)
                                .sum();
                            self.add_route(
                                &r,
                                Pos {
                                    x: r.pos.x - offset,
                                    y: r.pos.y,
                                },
                                cooldown_cost,
                                illegal_dir,
                            );
                        }
                    };
                    if r.illegal_dir != Direction::Vertical {
                        if r.pos.y + offset < self.map_max_y {
                            let illegal_dir = Direction::Vertical;
                            let cooldown_cost: u64 = (r.pos.y + 1..=r.pos.y + offset)
                                .map(|y| self.map[y][r.pos.x].cost)
                                .sum();
                            self.add_route(
                                &r,
                                Pos {
                                    x: r.pos.x,
                                    y: r.pos.y + offset,
                                },
                                cooldown_cost,
                                illegal_dir,
                            );
                        };
                        if offset <= r.pos.y {
                            let illegal_dir = Direction::Vertical;
                            let cooldown_cost: u64 = (r.pos.y - offset..=r.pos.y - 1)
                                .map(|y| self.map[y][r.pos.x].cost)
                                .sum();
                            self.add_route(
                                &r,
                                Pos {
                                    x: r.pos.x,
                                    y: r.pos.y - offset,
                                },
                                cooldown_cost,
                                illegal_dir,
                            );
                        }
                    };
                }
            }
        }
    }
    fn add_route(
        &mut self,
        parent_route: &Route,
        new_pos: Pos,
        cooldown_cost: u64,
        illegal_dir: Direction,
    ) {
        let total_cost = parent_route.total_cost + cooldown_cost;
        if let Some(prev_route) = self.map[new_pos.y][new_pos.x].best_paths.get(&illegal_dir) {
            // route has been at this node before
            if prev_route.total_cost <= total_cost {
                // we can't do better.
                return;
            }
        };
        let mut route_taken = parent_route.route_taken.clone();
        route_taken.push((new_pos.clone(), total_cost, cooldown_cost));
        self.routes.push(Route {
            pos: new_pos,
            total_cost,
            illegal_dir,
            route_taken,
        });
    }

    fn new() -> Data {
        Data {
            map: vec![],
            routes: vec![Route {
                pos: Pos { x: 0, y: 0 },
                total_cost: 0,
                illegal_dir: Direction::None,
                route_taken: vec![(Pos { x: 0, y: 0 }, 0, 0)],
            }],
            map_max_x: 0,
            map_max_y: 0,
        }
    }
    fn print_map(&self) {
        for row in &self.map {
            for spot in row {
                print!("{}", spot.cost)
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
#[derive(Debug, Clone, PartialEq, Eq)]
struct Spot {
    cost: u64,
    // Direction is the invalid direction
    best_paths: HashMap<Direction, Route>,
}
impl Spot {
    fn new(v: u64) -> Spot {
        Spot {
            cost: v,
            best_paths: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    // N,
    // S,
    // E,
    // W,
    Horizontal,
    Vertical,
    None,
}
fn read_data(file_name: &str) -> Data {
    let input = std::fs::read_to_string(file_name).expect("Missing file ?");
    let mut data = Data::new();
    for line in input.split("\n") {
        let row: Vec<Spot> = line
            .chars()
            .map(|ch| Spot::new(ch.to_string().parse().expect("Not a cost ?")))
            .collect();
        data.map.push(row);
    }
    data.map_max_y = data.map.len();
    data.map_max_x = data.map[0].len();
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {}
}
