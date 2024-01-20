pub fn run() {
    let data = read_data("in.txt");
    let total: u32 = data.iter().map(|instruction| instruction.label_hash).sum();
    const ARRAY_REPEAT_VALUE: Vec<Instruction> = Vec::new();
    let mut boxs: [Vec<Instruction>; 256] = [ARRAY_REPEAT_VALUE; 256];
    apply_data_to_boxs(&data, &mut boxs);
    println!("HashTotal: {} = 1320(tst)", total);
    boxs[3].push(data[1].clone());
    boxs[3].push(data[4].clone());
    // println!("{:?}", boxs);
    println!("Total focal: {}", calc_total_focal(&boxs));
    // calc_hash("HASH");
}
fn calc_total_focal(boxs:& [Vec<Instruction>; 256]) -> usize { 
    let mut total = 0;
    for (box_num, b) in boxs.iter().enumerate() {
        for (lens_num, lens) in b.iter().enumerate() {
            // One plus the box number of the lens in question.
            // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
            // The focal length of the lens.
            total += (1+box_num) * (lens_num + 1) * (lens.focal as usize);
        }
    }
    total
}
fn apply_data_to_boxs(data: &Vec<Instruction>, boxs: &mut [Vec<Instruction>; 256]) {
    for d in data {
        let box_num = d.label_hash as usize;
        if d.focal == 0 {
            // remove '-' lense == d.label
            boxs[box_num].retain(|lens| *lens.label != d.label);
        } else {
            // add  '=' lense
            if let Some(index) = boxs[box_num].iter().position(|lens| *lens.label == d.label) {
                // boxs[box_num].swap(index,d.clone());
                let _got = std::mem::replace(&mut boxs[box_num][index], d.clone());
            } else {
                boxs[box_num].push(d.clone());
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    full: String,
    label: String,
    label_hash: u32,
    focal: u32,
}

fn read_data(file_name: &str) -> Vec<Instruction> {
    let input = std::fs::read_to_string(file_name).expect("Missing file ?");
    let mut data: Vec<Instruction> = vec![];
    // let mut total = 0;
    for d in input.split(",") {
        let label;
        let focal;
        if d.contains("-") {
            label = d.split("-").next().unwrap();
            focal = 0;
        } else {
            let mut split = d.split("=");
            label = split.next().unwrap();
            focal = split.next().unwrap().parse().unwrap();
        }
        data.push(Instruction {
            full: d.to_string(),
            label: label.to_string(),
            label_hash: calc_hash(label),
            focal,
        });
        // println!(
        //     " d: {d:<8} h: {:>3} label: {label} focal: {focal}",
        //     calc_hash(label)
        // );
        // total += calc_hash(d);
    }
    data
}
fn calc_hash(input: &str) -> u32 {
    let mut hash = 0;
    for ch in input.chars() {
        let ascii = ch as u32;
        hash = ((hash + ascii) * 17) % 256;
        // println!("ch: {ch} ascii: {ascii} hash: {hash}")
    }
    hash
}
