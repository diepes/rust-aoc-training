pub fn run() {
    println!("lib.rs");
    // read_input("in.tst.txt");
    let vec_two_blocks = read_input("in.txt", false);
    let answer: usize = vec_two_blocks
        .iter()
        .map(|tb| tb.p1vr.p_centre + tb.p2h.p_centre * 100)
        .sum();
    println!("Final answer part1: {answer} = 34889");
    let vec_two_blocks = read_input("in.txt", true);
    let answer: usize = vec_two_blocks
        .iter()
        .map(|tb| tb.p1vr.p_centre + tb.p2h.p_centre * 100)
        .sum();
    println!("Final answer part2: {answer} = 34224");
}
#[derive(Debug)]
struct TwoBlocks {
    p1vr: Block, // rotated left (anti clockwise)
    p2h: Block,
}
#[derive(Debug)]
struct Block {
    b: Vec<Row>,
    p_centre: usize,
}
impl Block {
    fn new(input: &str, illegal_center: usize) -> Block {
        let mut rows = vec![];
        for (pos, line) in input.lines().enumerate() {
            let row = Row::new(line, pos);
            rows.push(row);
        }
        let p_centre = Block::find_centre(&rows, illegal_center);
        //assert_ne!(p_centre, 0, "no center found for {:?}", input);
        Block { p_centre, b: rows }
    }
    fn find_centre(rows: &Vec<Row>, illegal_center: usize) -> usize {
        let mut last_row_value = i64::MAX; //should never match row value
        let mut p_centre: usize = 0;
        for (pos, row) in rows.iter().enumerate() {
            if row.p_num == last_row_value && pos != illegal_center {
                // found possible centre (1 past) check reflections up and down
                let ldown: usize = pos;
                let lup = ldown - 1; // row[0] couldn't match
                let num_checks = std::cmp::min(lup, rows.len() - 1 - ldown);
                let true_reflection =
                    (0..=num_checks).all(|n| rows[ldown + n].p_num == rows[lup - n].p_num);
                if true_reflection {
                    assert_eq!( rows[ldown+num_checks].p_num, rows[lup-num_checks].p_num, "find_centre sanity check. num_checks:{num_checks} lup:{lup} ldown:{ldown} center:{ldown}\n{:?}\n",rows);
                    // assert_eq!(
                    //     p_centre,
                    //     0,
                    //     "found 2nd true_reflection!! #1@{p_centre} #2@{ldown} len:{len} \n{:#?}\n",
                    //     rows,
                    //     len = rows.len(),
                    // );
                    if p_centre == 0 {
                        p_centre = ldown; // record # rows above centre line
                    } else {
                        // 2nd true_reflection, set to zero
                        p_centre = 0
                    }
                } else {
                    // println!("no reflection at {pos}");
                }
            }
            last_row_value = row.p_num;
        }
        //assert_ne!(p_centre, 0, "found no true_reflection!! \n{:#?}\n", rows);
        p_centre
    }
    fn new_rotate(input: &str, illegal_center: usize ) -> Block {
        let mut rows: Vec<Vec<char>> = vec![];
        // load block int Vec
        let mut row_len: usize = 0;
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            if row_len > 0 {
                assert_eq!(row.len(), row_len, "Char row len should not differ !");
            } else {
                row_len = row.len();
            };
            rows.push(row);
        }
        // add newlines
        rows.push((0..row_len).map(|_n| '\n').collect());

        let rotated_block: String = (0..row_len)
            .map(|n| rows.iter().map(|row| row[n]).collect::<String>())
            .collect();
        //println!("\nrotate\n rows{:?}\n\n input:{:#?}\n\n rotate:{:#?}\n",rows,input,rotated_block);
        Block::new(&rotated_block, illegal_center)
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Row {
    //p: Vec<Vec<char>>,
    p: String,
    p_num: i64,
    pos: usize,
}
impl Row {
    fn new(line: &str, pos: usize) -> Row {
        Row {
            p: line.to_string(),
            pos: pos,
            p_num: line
                .chars()
                .enumerate()
                .map(|(n, ch)| match ch {
                    '#' => i64::pow(2, n as u32),
                    '.' => 0,
                    _ => panic!("Invalid char {ch}"),
                })
                .sum(),
        }
    }
}
fn read_input(file_name: &str, fix_smudge: bool) -> Vec<TwoBlocks> {
    let mut data: Vec<TwoBlocks> = vec![];
    let input = std::fs::read_to_string(file_name).expect(&format!("Cant read file {file_name}"));
    // for chunk in &data.into_iter().chunks(3) {
    //for (p1, p2) in input.split("\n\n").into_iter().windows(2).collect() {
    let mut iter_pattern = input.split("\n\n");
    for (block_num, block_txt) in iter_pattern.enumerate() {
        data.push(TwoBlocks {
            p1vr: Block::new_rotate(block_txt, 0),
            p2h: Block::new(block_txt, 0),
        })
    }
    if fix_smudge {
        // we got base data, now find change that give new reflection
        let mut data_smudge: Vec<TwoBlocks> = vec![];
        let input = std::fs::read_to_string(file_name).unwrap();
        let mut iter_pattern = input.split("\n\n");
        'next_block: for (block_num, block_txt) in iter_pattern.enumerate() {
            let mut data_smudge_options: Vec<TwoBlocks> = vec![];
            'next_ch: for (ch_num, ch) in block_txt.chars().enumerate() {
                let mut unsmudge_txt = block_txt.to_string();
                match ch {
                    '#' => unsmudge_txt.replace_range(ch_num..ch_num + 1, "."),
                    '.' => unsmudge_txt.replace_range(ch_num..ch_num + 1, "#"),
                    _ => {
                        assert_eq!(ch, '\n', "not newline ?");
                        continue 'next_ch;
                    } // new line
                }
                assert_eq!(unsmudge_txt.len(), block_txt.len(), "length changed ??");
                assert_ne!(unsmudge_txt, block_txt, "no char changed ??");
                let new_p1vr = Block::new_rotate(&unsmudge_txt, data[block_num].p1vr.p_centre);
                let new_p2h = Block::new(&unsmudge_txt, data[block_num].p2h.p_centre);
                // if (new_p1vr.p_centre != 0 && new_p1vr.p_centre != data[block_num].p1vr.p_centre)
                //     && (new_p2h.p_centre != 0 && new_p2h.p_centre != data[block_num].p2h.p_centre)
                if (new_p1vr.p_centre != 0 || new_p2h.p_centre != 0)
                    && (new_p1vr.p_centre != data[block_num].p1vr.p_centre
                        || new_p2h.p_centre != data[block_num].p2h.p_centre)
                {
                    data_smudge_options.push(TwoBlocks {
                        p1vr: new_p1vr,
                        p2h: new_p2h,
                    });
                }
            }
            // assert!(
            //     data_smudge_options.len() < 2,
            //     "not just 1 smudge option {} \n{:?}\n",
            //     data_smudge_options.len(),
            //     data_smudge_options
            // );
            if data_smudge_options.len() > 0 {
                data_smudge.push(
                    data_smudge_options
                        .pop()
                        .expect(" found no new reflection :("),
                );
            };
        }

        return data_smudge;
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_center_1horizontal() {
        for (input_b, values, center) in [
            ("..##..\n..##..\n......", [12, 12, 0, 99], 1),
            (".####.\n..##..\n..##..", [30, 12, 12, 99], 2),
            (".#..\n####\n####\n..##", [2, 15, 15, 12], 0),
        ] {
            let block = Block::new(input_b);
            println!("{:?}", block);
            for (n, line) in input_b.split('\n').enumerate() {
                assert_eq!(
                    block.b[n],
                    Row {
                        p: line.to_string(),
                        p_num: values[n],
                        pos: n
                    }
                );
            }
            assert_eq!(
                Block::find_centre(&block.b),
                center,
                "wrong center input:\n{input_b}\n{:?}\n",
                block.b
            );
        }
    }
    #[test]
    fn test_find_center_2vertical() {
        for (input_b, values, center, rotated) in [
            (
                "...\n...\n##.\n##.\n...\n...",
                [12, 12, 0],
                1,
                "..##..\n..##..\n......",
            ),
            (
                "...\n#..\n###\n#..\n...",
                [14, 4, 4],
                2,
                ".###.\n..#..\n..#..",
            ),
        ] {
            let block = Block::new_rotate(input_b, 0);
            println!("{:?}", block);
            for (n, line) in rotated.split('\n').enumerate() {
                assert_eq!(
                    block.b[n],
                    Row {
                        p: line.to_string(),
                        p_num: values[n],
                        pos: n
                    },
                    "block.b[{n}]:{:?}  line:{}",
                    block.b[n],
                    line
                );
            }
            assert_eq!(Block::find_centre(&block.b), center);
        }
    }
}
