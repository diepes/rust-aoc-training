use colored::Colorize;
use colour;

fn main() {
    println!("Hello, world!");
    let mut tiles = read_to_string("in.txt");
    println!(
        "Loaded tiles {} x {}",
        tiles.grid.len(),
        tiles.grid[0].len()
    );
    // find s start tile (x,y, tile)
    let (x, y): (usize, usize) = tiles
        .grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            if let Some(x) = line.iter().enumerate().find_map(|(x, tile)| {
                if tile.pipe == TileType::s {
                    Some(x)
                } else {
                    None
                }
            }) {
                Some((x, y))
            } else {
                None
            }
        })
        .expect("Missing s start tile ?");
    println!("Found start @ {x},{y} ",);
    let start_tile = Pos { x, y };
    // println!();
    // print_tiles(&tiles);
    // println!();
    let mut tiles_connected: Vec<Pos> = vec![];
    let mut tiles_next = std::collections::VecDeque::new();
    tiles.update_colour(x, y, Colours::Start);
    tiles.update_steps_from_start(x, y, 0);
    if let Some(tiles_connected) = find_connecting_pipes(&start_tile, &mut tiles) {
        tiles_next = tiles_connected.into();
        // update start tile with directions for counting later.
        // relative pos to start
        let pos23 = (
            &tiles_next[0].x + 1 - &start_tile.x,
            &tiles_next[0].y + 1 - &start_tile.y,
            &tiles_next[1].x + 1 - &start_tile.x,
            &tiles_next[1].y + 1 - &start_tile.y,
        );
        let start_type = match pos23 {
            (0, 1, 1, 0) | (1, 0, 0, 1) => TileType::nw,
            (1, 0, 2, 1) | (2, 1, 1, 0) => TileType::ne,
            (2, 1, 1, 2) | (1, 2, 2, 1) => TileType::se,
            (1, 2, 0, 1) | (0, 1, 1, 2) => TileType::sw,
            (0, 1, 2, 1) | (2, 1, 0, 1) => TileType::h,
            (1, 0, 1, 2) | (1, 2, 1, 0) => TileType::v,
            _ => panic!("Illegal tiles around start tile {pos23:?}"),
        };
        // Save calculated start tile type
        tiles.grid[start_tile.y][start_tile.x].pipe = start_type;
    } else {
        panic!("No tiles found connecting to start tile ?");
    };
    let mut last_pos: Pos = start_tile; // dumy value to find end
    while tiles_next.len() > 0 {
        if let Some(next_pos) = tiles_next.pop_back() {
            last_pos = next_pos;
            if let Some(tiles_temp) = find_connecting_pipes(&next_pos, &mut tiles) {
                for tt in tiles_temp {
                    tiles_next.push_front(tt);
                    tiles_connected.push(tt);
                }
            };
        };
    }
    tiles.update_colour(last_pos.x, last_pos.y, Colours::End);

    //println!("Found connected tiles {:#?}", tiles_connected);
    println!();
    print_tiles(&tiles);
    println!();

    tiles.update_in_out();

    println!();
    print_tiles(&tiles);
    println!();
}

fn find_connecting_pipes<'a>(current_pos: &Pos, data: &'a mut Tiles) -> Option<Vec<Pos>> {
    let max_y = data.grid.len() - 1;
    let max_x = data.grid[0].len() - 1;
    let x = current_pos.x;
    let y = current_pos.y;
    //let tile = &tiles[y][x];
    let mut connected_tiles: Vec<Pos> = vec![];
    let (_p_ch, p_directions, _p_description) = data.get_pipe_details(x, y);
    for d in p_directions {
        // println!("check direction match {d} ...");

        match d {
            'n' => {
                if y > 0 {
                    if data.get_pipe_details(x, y - 1).1.contains(&'s') {
                        connected_tiles.push(Pos { x: x, y: y - 1 });
                    }
                }
            }
            's' => {
                //println!("match s");
                if y < max_y {
                    if data.get_pipe_details(x, y + 1).1.contains(&'n') {
                        connected_tiles.push(Pos { x: x, y: y + 1 });
                    }
                }
            }
            'e' => {
                //right
                if x < max_x {
                    if data.get_pipe_details(x + 1, y).1.contains(&'w') {
                        connected_tiles.push(Pos { x: x + 1, y: y });
                    }
                }
            }
            'w' => {
                if x > 0 {
                    if data.get_pipe_details(x - 1, y).1.contains(&'e') {
                        connected_tiles.push(Pos { x: x - 1, y: y });
                    }
                }
            }
            'g' => {
                // println!("match g");
            }
            _ => panic!(" Unknown pipe direction '{d}' ?"),
        }
    }
    assert!(
        connected_tiles.len() <= 2,
        "Found more than two connecting pipes"
    );
    for (n, tile) in connected_tiles.clone().iter().enumerate() {
        // if tile already has steps remove it from our findings
        if let Some(st) = data.grid[tile.y][tile.x].steps_from_start {
            let pos_del = connected_tiles.remove(if connected_tiles.len() > n { n } else { 0 });
            //println!(" remove connected {:?} had step {st}", pos_del);
        } else {
            // set to previos tile color
            let mut prev_color = data.grid[y][x].colour.clone();
            // println!(
            //     " for search [{x},{y}] set colour to {prev_color:?} for tile {:?}",
            //     tile
            // );
            if prev_color == Colours::Start {
                prev_color = if n == 0 { Colours::Red } else { Colours::Green };
                println!("   ... prev_color Start use {:?}", prev_color);
            }
            data.update_colour(tile.x, tile.y, prev_color);
        }
    }
    for (_j, pos) in connected_tiles.iter().enumerate() {
        if let Some(step) = data.grid[current_pos.y][current_pos.x].steps_from_start {
            data.update_steps_from_start(pos.x, pos.y, step + 1);
        } else {
            panic!(" search tile had no steps_from_start ??? {x},{y}");
        }
    }
    // println!("    ... done. found:{}", connected_tiles.len());
    if connected_tiles.len() > 0 {
        Some(connected_tiles)
    } else {
        None
    }
}

fn print_tiles(t: &Tiles) {
    let print_steps = false;
    let print_start_end = false;
    let mut end_steps: usize = 0;
    let mut cnt_in = 0;
    let mut cnt_out = 0;
    let mut cnt_pipe = 0;
    let mut cnt_total = 0;
    for (y, row) in t.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            cnt_total +=1;
            let mut ch = t.get_pipe_details(x, y).0;
            if print_steps {
                if let Some(steps) = tile.steps_from_start {
                    ch = steps.to_string().chars().nth(0).unwrap();
                };
            };
            if print_start_end {
                if [Colours::Start, Colours::End].contains(&tile.colour) {
                    // ch = steps.to_string().chars().nth(0).unwrap();
                    if let Some(steps) = tile.steps_from_start {
                        if tile.colour == Colours::End {
                            end_steps = steps
                        };
                        ch = steps.to_string().chars().nth(0).unwrap();
                    };
                };
            }
            match &tile.colour {
                Colours::Start => { cnt_pipe +=1; colour::yellow!("{}", ch)},
                Colours::End => { cnt_pipe +=1; colour::blue!("{}", ch)},
                Colours::Red => { cnt_pipe +=1; colour::red!("{}", ch)},
                Colours::Green => { cnt_pipe +=1; colour::green!("{}", ch)},
                Colours::In => {
                    cnt_in += 1;
                    colour::e_cyan!("{}", ch)
                }
                Colours::Out => {
                    cnt_out += 1;
                    colour::e_dark_magenta!("{}", ch)
                }
                _ => colour::prnt!("{}", ch),
            }
        }
        println!();
    }
    colour::prnt_ln!(); //reset colour
    println!("End steps = {end_steps} total={cnt_total} in={cnt_in}  out={cnt_out} pipe={cnt_pipe} err={}",cnt_total-cnt_in-cnt_out-cnt_pipe);
}

fn read_to_string(f: &str) -> Tiles {
    let input = std::fs::read_to_string(f).expect("Err reading file");
    let mut data = Tiles::new();
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Tile> = vec![];
        for (x, c) in line.chars().enumerate() {
            let t = TileType::get_type(c);
            // println!("{x} {c} => {t:?}");
            row.push(Tile {
                pipe: t,
                steps_from_start: None,
                colour: Colours::None,
            });
        }
        data.grid.push(row);
    }
    data
}

#[derive(Debug)]
struct Tiles {
    grid: Vec<Vec<Tile>>,
}
impl Tiles {
    fn new() -> Self {
        Tiles { grid: vec![] }
    }
    fn update_colour(&mut self, x: usize, y: usize, colour: Colours) {
        self.grid[y][x].colour = colour;
    }
    fn update_steps_from_start(&mut self, x: usize, y: usize, steps: usize) {
        self.grid[y][x].steps_from_start = Some(steps);
    }
    fn get_pipe_details(&self, x: usize, y: usize) -> (char, Vec<char>, &str) {
        match self.grid[y][x].pipe {
            TileType::v => (
                '|',
                vec!['n', 's'],
                "is a vertical pipe connecting north and south.",
            ),
            TileType::h => (
                '-',
                vec!['e', 'w'],
                "is a horizontal pipe connecting east and west.",
            ),
            TileType::ne => (
                'L',
                vec!['n', 'e'],
                "is a 90-degree bend connecting north and east.",
            ),
            TileType::nw => (
                'J',
                vec!['n', 'w'],
                "is a 90-degree bend connecting north and west.",
            ),
            TileType::sw => (
                '7',
                vec!['s', 'w'],
                "is a 90-degree bend connecting south and west.",
            ),
            TileType::se => (
                'F',
                vec!['s', 'e'],
                "is a 90-degree bend connecting south and east.",
            ),
            TileType::g => ('.', vec!['g'], "is ground; there is no pipe in this tile."),
            TileType::s => (
                'S',
                vec!['n', 's', 'e', 'w'],
                "is the starting position of the animal;)",
            ),
        }
    }
    fn update_in_out(&mut self) {
        let mut ground_out = 0;
        let mut ground_in = 0;
        for (_y, line) in self.grid.iter_mut().enumerate() {
            let mut pipe_cnt = 0;
            let mut prev_pipe = &TileType::h; //horizontal no count
            for (_x, tile) in line.iter_mut().enumerate() {
                match &tile.colour {
                    // red/green start end set.
                    Colours::Start | Colours::End | Colours::Green | Colours::Red => {
                        match (prev_pipe.clone(), &tile.pipe) {
                            (_, TileType::h) => {} // horizontal dont count.
                            (TileType::ne, TileType::sw) | (TileType::se, TileType::nw) => {
                                // zig zag line up down only count 1
                                prev_pipe = &tile.pipe;
                            }
                            (_, TileType::g | TileType::s) | (TileType::s | TileType::g, _) => {
                                panic!(" Pipe Colour, but type {:?} !!", tile.pipe);
                            }
                            (_, TileType::v | TileType::ne | TileType::se)
                            | (TileType::se, _)
                            | (TileType::h | TileType::v, TileType::nw | TileType::sw)
                            | (TileType::ne, TileType::nw)
                            | (TileType::nw, _)
                            | (TileType::sw, _) => {
                                pipe_cnt += 1;
                                prev_pipe = &tile.pipe;
                            }
                        };
                    }
                    _ => {
                        // set ground tile colour
                        if pipe_cnt % 2 == 0 {
                            //outside pipe loop
                            tile.colour = Colours::Out;
                            ground_out += 1;
                        } else {
                            //inside pipe loop
                            tile.colour = Colours::In;
                            ground_in += 1;
                        };
                    }
                };
            }
        }
    }
}

#[derive(Debug)]
struct Tile {
    pipe: TileType,
    //pos: Pos,
    steps_from_start: Option<usize>,
    colour: Colours,
    //connect1: Option<Pos>,
    //connect2: Option<Pos>,
}
impl Tile {}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Colours {
    None,
    Red,
    Green,
    Start,
    End,
    In,
    Out,
}
#[derive(Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum TileType {
    s, //start
    v, //vertical
    h, //horizontal
    ne,
    nw,
    sw,
    se,
    g, //ground
}

impl TileType {
    fn get_type(c: char) -> TileType {
        match c {
            '|' => TileType::v,
            '-' => TileType::h,
            'L' => TileType::ne,
            'J' => TileType::nw,
            '7' => TileType::sw,
            'F' => TileType::se,
            '.' => TileType::g,
            'S' => TileType::s,
            _ => panic!("get_type invalid char {c}"),
        }
    }
}
