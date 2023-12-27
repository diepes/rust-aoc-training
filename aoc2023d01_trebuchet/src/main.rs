// use std::fs;
fn main() {
    let file_in = "in_01b.txt";
    let file_out = "out_02.txt";
    println!("Hello, world! reading {file_in}");
    let l_in = read_lines(file_in);
    // let l_out = read_lines(file_out);
    let mut total: usize = 0;
    let mut total_val: u32 = 0;
    for (i, l) in l_in.iter().enumerate() {
        let get_num = get_num_from_str(l);
        total += get_num;
        let get_nums = get_nums_from_str(l);
        let get_num1 = get_nums.chars().next().unwrap();
        let get_num2 = get_nums.chars().last().unwrap();
        let get_val = get_num1.to_digit(10).unwrap()*10 + get_num2.to_digit(10).unwrap();
        total_val += get_val;
        // let out = if i < l_out.len() { &l_out[i] } else { "0" };
        println!("in:{} get:{}={} >> {}={}  num_from:{}", l_in[i], get_num, get_val, total,total_val, get_nums);
    }
    println!("Total: {} = {}", total, total_val);
}
fn read_lines(file_name: &str) -> Vec<String> {
    let lines = std::fs::read_to_string(file_name).expect("file_in not found!");
    lines.lines().map(|l| l.to_string()).collect()
}
fn get_nums_from_str(in_str: &str) -> String {
    // reads through string and convert to just numbers
    let mut return_str = "".to_string();
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, c) in in_str.chars().enumerate() {
        if let Some(_) = c.to_digit(10) {
            return_str.push(c);
        } else {
            // check for text num
            for (i_num, num_str) in numbers.iter().enumerate() {
                // match from position in_str[i:]
                if in_str[i..].starts_with(num_str) {
                    return_str.push(std::char::from_digit((i_num) as u32, 10).unwrap());
                }
            }
        }
    }
    return_str
}
fn get_num_from_str(s: &str) -> usize {
    let mut a: Option<u32> = None;
    let mut b: Option<u32> = None;
    let mut substring: String = "".to_string();
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for c in s.chars() {
        match c.to_digit(10) {
            None => {
                // not a digit, add to substring and check for word value
                substring.push(c);
                // check for string match
                for (i, num) in numbers.iter().enumerate() {
                    if substring.ends_with(num) {
                        b = Some(i as u32);
                        if a == None {
                            a = b;
                        }
                        // Found number reset sub string
                        substring = "".to_string();
                        break;
                    }
                }
            }
            n => {
                // Found number reset sub string
                substring = "".to_string();
                b = n;
                if a == None {
                    a = n;
                }
            }
        }
    }
    assert_ne!(a, None);
    assert_ne!(b, None);
    (a.unwrap() * 10 + b.unwrap()) as usize
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_get_num_a() {
        assert_eq!(get_num_from_str("1ab2"), 12);
        let file_in = "in_01.txt";
        let file_out = "out_01.txt";
        let l_in = read_lines(file_in);
        let l_out = read_lines(file_out);
        let mut total: usize = 0;
        for (i, l) in l_in.iter().enumerate() {
            let get_num_in = get_num_from_str(l);
            let get_num_out = l_out[i].parse().expect("Err parsing num_out");
            total += get_num_in;
            assert_eq!(get_num_in, get_num_out)
        }
        assert_eq!(total, 142);
    }
    #[test]
    fn test_get_num_b() {
        assert_eq!(get_num_from_str("1ab2two"), 12);
        let file_in = "in_02.txt";
        let file_out = "out_02.txt";
        let l_in = read_lines(file_in);
        let l_out = read_lines(file_out);
        let mut total: usize = 0;
        for (i, l) in l_in.iter().enumerate() {
            let get_num_in = get_num_from_str(l);
            let get_num_out = l_out[i].parse().expect("Err parsing num_out");
            total += get_num_in;
            assert_eq!(get_num_in, get_num_out)
        }
        assert_eq!(total, 281);
    }
    #[test]
    fn test_read_lines() {
        let lines = read_lines("in_01.txt");
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "1abc2")
    }
}
