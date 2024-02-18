mod read_file;
use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;

pub fn run() {
    println!("Hello lib.rs");
    let mut data = read_file::read("in.txt");
    // data.blocks sorted lowest z=1 first.
    //println!("Data {:?}", data);
    let mut grid = Grid::new(&data);
    //println!("Grid {:?}", grid.g);
    let mut cnt_drops = 0;
    loop {
        let blocks_dropped = grid.move_down(&mut data);
        // println!("Moving down blocks: {blocks_dropped}");
        if blocks_dropped == 0 {
            break;
        }
        cnt_drops += 1;
    }
    println!("Moving down steps:{cnt_drops}");
    let free = grid.cnt_none_above(&mut data);
    println!("Free {free} / {} p1=418", data.blocks.len());
    println!("start p2 ...");
    let drop = cnt_total_drop(&data);
    println!("Drop {drop} / {} p2=70702", data.blocks.len());
}
fn cnt_total_drop(data: &Data) -> usize {
    let mut cnt_drops = 0;
    let num_blocks = data.blocks.len();
    for block_num in 0..data.blocks.len() {
        // remove / explode block out of Grid and see how many drop
        //let temp_grid = self.clone();
        let mut temp_data: Data = data.clone();
        // println!("  cnt_total_drop: remove data.blocks #{}",block_num);
        temp_data.blocks.remove(block_num);
        assert_eq!(temp_data.blocks.len(), num_blocks - 1, "Dropped one block.");
        let mut temp_grid = Grid::new(&temp_data);
        let blocks_dropped = temp_grid.move_down(&mut temp_data);
        cnt_drops += blocks_dropped;
        // println!("p2 bd:{blocks_dropped} total:{cnt_drops}");

        // cnt_drops += self.cnt_drops_recursive(data, block.id);
    }
    cnt_drops
}

#[derive(Debug, Clone)]
pub struct Data {
    blocks: Vec<SandBlock>,
}
#[derive(Debug, Clone)]
pub struct SandBlock {
    //bottom
    x: usize,
    y: usize,
    z: usize,
    xl: usize,
    yl: usize,
    zl: usize,
    id: usize,
    step: usize,
    supported_by: HashSet<usize>, // Vec of block id's
    holding_up: HashSet<usize>,   // Vec of block id's
}
#[derive(Debug)]
pub struct Grid {
    g: HashMap<(usize, usize, usize), usize>,
}
impl Grid {
    fn new(data: &Data) -> Grid {
        let mut g: HashMap<(usize, usize, usize), usize> = HashMap::new();
        for block in &data.blocks {
            for x in block.x..=block.x + block.xl {
                for y in block.y..=block.y + block.yl {
                    for z in block.z - block.step..=block.z + block.zl - block.step {
                        assert_eq!(g.get(&(x, y, z)), None);
                        g.insert((x, y, z), block.id.clone());
                    }
                }
            }
        }
        Grid { g }
    }

    fn cnt_none_above(&self, data: &mut Data) -> usize {
        let mut cnt_none_above = 0;
        for block in &mut data.blocks {
            let mut holding_up: HashSet<usize> = HashSet::new(); //blocks above this one
            let z = block.z + block.zl - block.step; //top
            for x in block.x..=block.x + block.xl {
                for y in block.y..=block.y + block.yl {
                    // assert existing blocks in grid
                    assert_eq!(
                        self.g.get(&(x, y, z)),
                        Some(block.id).as_ref(),
                        "Existing block not there."
                    );
                    // check if block above
                    if let Some(id) = self.g.get(&(x, y, z + 1)) {
                        holding_up.insert(id.clone());
                    }
                }
            }
            block.holding_up = holding_up;
        }
        for bn in 0..data.blocks.len() {
            let mut this_block_critical_support = false;
            if data.blocks[bn].holding_up.len() == 0 {
                this_block_critical_support = false;
            } else {
                for id in &data.blocks[bn].holding_up {
                    // find the id in data.blocks and get supported_by
                    let block_above = &data.blocks.iter().find(|bl| bl.id == *id).unwrap();
                    assert!(
                        block_above.supported_by.len() > 0,
                        "We are holding up a block that thinks its supported by nothing ?"
                    );
                    if block_above.supported_by.len() == 1 {
                        this_block_critical_support = true;
                    }
                }
            }
            if this_block_critical_support == false {
                cnt_none_above += 1;
            }
        }

        cnt_none_above
    }

    fn move_down(&mut self, data: &mut Data) -> usize {
        let debug_id = 1_000_000;
        let mut movement_cnt = 0;
        // println!("  Start move_down ...");
        for (block_index, block) in &mut data.blocks.iter_mut().enumerate() {
            // println!("     ... step:{step} id:{id} z:{z}",step=block.step,id=block.id,z=block.z);
            block.holding_up = HashSet::new(); //Empty, we start from bottom to top
            if block.z - block.step > 1 {
                // can still move down
                let z = block.z - block.step; //bottom of block
                let mut supported_by: HashSet<usize> = HashSet::new();
                // check with z at bottom of this block for something below this block using x,y
                for x in block.x..=block.x + block.xl {
                    for y in block.y..=block.y + block.yl {
                        assert_eq!(
                            self.g.get(&(x, y, z)),
                            Some(block.id).as_ref(),
                            "Existing block not there. id:{id} block.step:{step} z:{z}",
                            id = block.id,
                            step = block.step
                        );
                        // check if block below empty or not.
                        if let Some(id) = self.g.get(&(x, y, z - 1)) {
                            supported_by.insert(id.clone());
                        };
                    }
                }
                block.supported_by = supported_by;
                //
                // checked state of supported_by
                if block.supported_by.len() == 0 {
                    // Now we are still falling, and nothing below this block
                    // move block down in Grid
                    movement_cnt += 1;
                    for z in block.z - block.step..=block.z + block.zl - block.step {
                        for x in block.x..=block.x + block.xl {
                            for y in block.y..=block.y + block.yl {
                                if block.id == debug_id {
                                    //debug
                                    println!(
                                        "DebugB drop {debug_id}:{block_index} x:{x}, y:{y}, z:{z} -> zn:{zn} step:{step} mv down bz:{bz} zrange:{zr:?} => {zrr:?}",
                                        bz=block.z,
                                        zn = z - 1,
                                        step=block.step,
                                        zr=(block.z - block.step..=block.z + block.zl - block.step),
                                        zrr=(block.z - block.step..=block.z + block.zl - block.step).rev(),
                                    );
                                }
                                // assert existing blocks in grid
                                assert_eq!(
                                    self.g.get(&(x, y, z)),
                                    Some(block.id).as_ref(),
                                    "Existing block not there."
                                );
                                // assert empty blocks in grid
                                assert_eq!(self.g.get(&(x, y, z - 1)), None);
                                // move pieces down one
                                self.g.remove(&(x, y, z));
                                self.g.insert((x, y, z - 1), block.id.clone());
                            }
                        }
                    }
                    block.step += 1;
                    if block.id == debug_id {
                        //debug
                        println!("Debug {debug_id}: inc bloc.step to {:?}", block.step,);
                    }
                } else {
                    // we were blocked by something.
                    if block.id == debug_id {
                        println!(
                            "DebugC {debug_id}:{block_index} blocked, not moving zl:{zl} supported_by:{sup:?}",
                            zl= block.z - block.step,
                            sup=block.supported_by,
                        );
                    };
                };
            } else {
                if block.id == debug_id {
                    println!("DebugD {debug_id}:{block_index} at bottom already");
                };
            };
        }
        movement_cnt
    }
}
