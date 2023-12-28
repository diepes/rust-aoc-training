fn main() {
    println!("Hello, world!");
    let engine = std::fs::read_to_string("in.txt").expect("Err reading file");
    let engine_map: Vec<&str> = engine.lines().collect();
    println!("{:#?}", engine_map);
    let mut parts: Vec<Part>;
    let mut symbols: Vec<Symbol>;
    parts = get_parts_from_map(&engine_map);
    symbols = get_symbols_from_map(&engine_map);
    println!("{:#?}", symbols);
    update_part_symbol_touch(&engine_map, &mut parts, &mut symbols);
    let mut part_real = 0;
    let mut part_total = 0;
    for p in &parts {
        if p.touch_symbol == false {
            println!("part{:#?}", p);
        }
        if p.touch_symbol == true {
            part_real += 1;
            part_total += p.number;
        }
    }
    let mut total_gear_ratio = 0;
    let mut count_star_symbols = 0;
    for mut s in &mut symbols {
        if s.symbol == '*' {
            println!("* {:#?}", s);
            count_star_symbols += 1;
            if s.part_numbers.len() > 1 {
                let mut gear_ratio = 1;
                for p in &s.part_numbers {
                    gear_ratio = gear_ratio * p
                }
                s.gear_ratio = gear_ratio;
                total_gear_ratio += gear_ratio;
            }
        }
    }
    println!(
        "Found {} parts and {} symbols *{count_star_symbols} ⚙️⚙️{total_gear_ratio}, real:{part_real} total:{part_total}",
        parts.len(),
        symbols.len(),
    );
}

fn update_part_symbol_touch(e: &Vec<&str>, parts: &mut Vec<Part>, symbols: &mut Vec<Symbol>) {
    // for each part scan around it for symbols and update.
    // when touching symbol found update symbol vec with part number.
    let max_y = e.len();
    let max_x = e[0].len();
    for mut p in parts {
        //check top
        if p.y > 1 {
            let y = p.y - 2;
            let mut x = if p.x > 1 { p.x - 2 } else { 0 };
            while x < max_x && x < (p.x + p.len) {
                if let XyType::Symbol(c) = get_type(e[y].chars().nth(x).unwrap()) {
                    p.touch_symbol = true;
                    p.symbol_count += 1;
                    update_symbol_with_part_num(symbols, p.number, c, x + 1, y + 1);
                }
                x += 1;
            }
        }
        //check botom
        if p.y < max_y {
            let y = p.y;
            let mut x = if p.x > 1 { p.x - 2 } else { 0 };
            while x < max_x && x < (p.x + p.len) {
                if let XyType::Symbol(c) = get_type(e[y].chars().nth(x).unwrap()) {
                    p.touch_symbol = true;
                    p.symbol_count += 1;
                    update_symbol_with_part_num(symbols, p.number, c, x + 1, y + 1);
                }
                x += 1;
            }
        }

        //check left
        if p.x > 1 {
            let x = p.x - 1 - 1;
            let y = p.y - 1;
            if let XyType::Symbol(c) = get_type(e[y].chars().nth(x).unwrap()) {
                p.touch_symbol = true;
                p.symbol_count += 1;
                update_symbol_with_part_num(symbols, p.number, c, x + 1, y + 1);
            }
        }

        //check right
        if (p.x + p.len) < max_x {
            let x = p.x - 1 + p.len;
            let y = p.y - 1;
            let check_type = get_type(e[y].chars().nth(x).expect(&format!(
                "ERROR getting char x={x} y={y} max_x={max_x} {:#?}",
                p
            )));
            if let XyType::Symbol(c) = check_type {
                p.touch_symbol = true;
                p.symbol_count += 1;
                update_symbol_with_part_num(symbols, p.number, c, x + 1, y + 1);
            }
        }
    }
}

fn update_symbol_with_part_num(
    symbols: &mut Vec<Symbol>,
    p_number: usize,
    c: char,
    x: usize,
    y: usize,
) {
    // find symbol
    let mut found = 0;
    for s in symbols {
        if s.x == x && s.y == y {
            s.part_numbers.push(p_number);
            found += 1;
        }
    }
    assert_eq!(found, 1, "Only one symbol at single x{x},y{y},c{c}")
}

fn get_symbols_from_map(map: &Vec<&str>) -> Vec<Symbol> {
    let mut symbols = vec![];
    for (y, line) in map.iter().enumerate() {
        get_symbols_from_str(&mut symbols, y, line);
    }
    symbols
}
fn get_symbols_from_str(p: &mut Vec<Symbol>, y: usize, line: &str) {
    for (x, c) in line.chars().enumerate() {
        if let XyType::Symbol(c) = get_type(c) {
            p.push(Symbol {
                symbol: c,
                y: y + 1,
                x: x + 1,
                is_star: if c == '*' { true } else { false }, // This is a symbol, * ?
                part_numbers: vec![],
                gear_ratio: 0,
            });
        }
    }
}

fn get_parts_from_map(map: &Vec<&str>) -> Vec<Part> {
    let mut parts = vec![];
    for (y, line) in map.iter().enumerate() {
        get_parts_from_str(&mut parts, y, line);
    }
    parts
}

fn get_parts_from_str(p: &mut Vec<Part>, y: usize, line: &str) {
    let mut num_str = "".to_string();
    for (x, c) in line.chars().enumerate() {
        if get_type(c) == XyType::Number {
            num_str.push(c);
        } else if num_str.len() > 0 {
            // found a number
            p.push(Part {
                number: num_str.parse().unwrap(),
                y: y + 1,
                x: x + 1 - num_str.len(),
                len: num_str.len(),
                touch_symbol: false, // Unknown
                symbol_count: 0,
            });
            num_str = "".to_string();
        } else {
            assert_ne!(get_type(c), XyType::Number);
        }
    }
    // catch case where last char was against side.
    if num_str.len() > 0 {
        // found a number
        let x = line.len();
        p.push(Part {
            number: num_str.parse().unwrap(),
            y: y + 1,
            x: x + 1 - num_str.len(),
            len: num_str.len(),
            touch_symbol: false, // Unknown
            symbol_count: 0,
        });
    }
}

fn get_type(c: char) -> XyType {
    if c == '.' {
        return XyType::Dot;
    }
    if c.is_digit(10) {
        return XyType::Number;
    }
    XyType::Symbol(c)
}

#[derive(Debug, PartialEq)]
enum XyType {
    Dot,
    Number,
    Symbol(char),
}

#[derive(Debug, PartialEq)]
struct Part {
    number: usize,
    x: usize,
    y: usize,
    len: usize,
    touch_symbol: bool,
    symbol_count: usize,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
    is_star: bool,
    part_numbers: Vec<usize>,
    gear_ratio: usize,
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_right() {
        let engine_map = vec!["467..114*."];
        let mut parts: Vec<Part>;
        let mut symbols: Vec<Symbol> = vec![];
        parts = get_parts_from_map(&engine_map);
        symbols = get_symbols_from_map(&engine_map);
        update_part_symbol_touch(&engine_map, &mut parts, &mut symbols);
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            Part {
                number: 467,
                x: 1,
                y: 1,
                len: 3,
                touch_symbol: false,
                symbol_count: 0,
            }
        );
        assert_eq!(
            parts[1],
            Part {
                number: 114,
                x: 6,
                y: 1,
                len: 3,
                touch_symbol: true,
                symbol_count: 1,
            }
        );
    }
    #[test]
    fn test_down_right() {
        let engine_map = vec!["467...114.", "...*......"];
        let mut parts: Vec<Part>;
        let mut symbols: Vec<Symbol> = vec![];
        parts = get_parts_from_map(&engine_map);
        symbols = get_symbols_from_map(&engine_map);
        update_part_symbol_touch(&engine_map, &mut parts, &mut symbols);
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            Part {
                number: 467,
                x: 1,
                y: 1,
                len: 3,
                touch_symbol: true,
                symbol_count: 1,
            }
        );
        assert_eq!(
            parts[1],
            Part {
                number: 114,
                x: 7,
                y: 1,
                len: 3,
                touch_symbol: false,
                symbol_count: 0,
            }
        );
    }
    #[test]
    fn test_corners() {
        let engine_map = vec![
            "*...*.222.",
            ".467...114",
            "$...#.....",
            "..........",
            "..........",
        ];
        let mut parts: Vec<Part>;
        let mut symbols: Vec<Symbol> = vec![];
        parts = get_parts_from_map(&engine_map);
        symbols = get_symbols_from_map(&engine_map);
        update_part_symbol_touch(&engine_map, &mut parts, &mut symbols);
        assert_eq!(parts.len(), 3);
        assert_eq!(
            parts[1],
            Part {
                number: 467,
                x: 2,
                y: 2,
                len: 3,
                touch_symbol: true,
                symbol_count: 4,
            }
        );
        assert_eq!(
            parts[0],
            Part {
                number: 222,
                x: 7,
                y: 1,
                len: 3,
                touch_symbol: false,
                symbol_count: 0,
            }
        );
        assert_eq!(
            parts[2],
            Part {
                number: 114,
                x: 8,
                y: 2,
                len: 3,
                touch_symbol: false,
                symbol_count: 0,
            }
        );
    }
}
