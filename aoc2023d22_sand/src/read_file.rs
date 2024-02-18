use crate::{Data, SandBlock};
use std::collections::HashSet;

pub fn read(file_name: &str) -> Data {
    let mut blocks: Vec<SandBlock> = Vec::new();
    let input = std::fs::read_to_string(file_name).expect("Unknown file !");
    for (num, line) in input.split("\n").enumerate() {
        let (pa, pb) = line.split_once("~").expect("Invalid line");
        let mut val = pa.split(",");
        let x: usize = val
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not numeric value?");
        let y: usize = val
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not numeric value?");
        let z: usize = val
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not numeric value?");
        let mut val = pb.split(",");
        let x2: usize = val
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not numeric value?");
        let y2: usize = val
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not numeric value?");
        let z2: usize = val
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not numeric value?");
        assert!(x2 >= x && y2 >= y && z2 >= z);
        blocks.push(SandBlock {
            id: num,
            step: 0,
            supported_by: HashSet::new(),
            holding_up: HashSet::new(),
            x,
            y,
            z,
            xl: x2 - x,
            yl: y2 - y,
            zl: z2 - z,
        })
    }

    blocks.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
    Data { blocks }
}
