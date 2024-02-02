pub fn run() {
    println!("lib.rs");
    let data_empty = Data::parse("in.txt");
    let mut data = data_empty.clone();
    data.update_spot(data.start_y, data.start_x, 0, 0, Plot::Person); // 1 Person at start pos.
    data.print_map();
    for steps in 1..=64 {
        data = data.take_step(&data_empty);
        println!("Steps: {steps}");
        data.print_map();
    }
    println!("Count {}",data.count_person());
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
    start_x: usize,
    start_y: usize,
    max_x: usize,
    max_y: usize,
}
impl Data {
    fn count_person(&self) -> usize {
        let mut cnt = 0;
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                match self.map[y][x] {
                    Plot::Rock => (),
                    Plot::Garden => (),
                    Plot::Person => {
                        cnt += 1;
                    }
                }
            }
        }

        cnt
    }

    fn take_step(&self, empty: &Data) -> Data {
        let mut data = empty.clone();
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                match self.map[y][x] {
                    Plot::Rock => (),
                    Plot::Garden => (),
                    Plot::Person => {
                        // move in 4 dir
                        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            data.update_spot(x, y, dx, dy, Plot::Person);
                        }
                    }
                }
            }
        }

        data
    }
    fn update_spot(&mut self, x: usize, y: usize, delta_x: isize, delta_y: isize, plot_type: Plot) {
        let nx = x as isize + delta_x;
        let ny = y as isize + delta_y;
        if nx >= 0 && nx < self.max_x as isize && ny > 0 && ny < self.max_y as isize {
            // in bounds
            if self.map[ny as usize][nx as usize] == Plot::Rock {
                // skip rock
            } else {
                self.map[ny as usize][nx as usize] = plot_type;
            };
        };
    }

    fn print_map(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if x == self.start_x && y == self.start_y {
                    print!("S");
                } else {
                    print!(
                        "{}",
                        match self.map[y][x] {
                            Plot::Rock => '#',
                            Plot::Garden => '.',
                            Plot::Person => 'O',
                        }
                    );
                }
            }
            println!();
        }
        println!()
    }
    fn parse(file_name: &str) -> Data {
        let input = std::fs::read_to_string(file_name).expect("Missing file?");
        let mut map: Vec<Vec<Plot>>;
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
                            start_x = x;
                            start_y = y;
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
            map,
            start_x,
            start_y,
        }
    }
}
