fn main() {
    let input = std::fs::read_to_string("in.txt").expect("Err loading file");
    println!("Hello, world! {}", input.len());
    let data = parse(&input);
    println!("Data line1 {:?}", data.data[0]);
    let mut total = 0;
    let mut total2 = 0;
    for (l, d) in data.data.iter().enumerate() {
        let (answer,answer2) = run(&d);
        total += answer;
        total2 += answer2;
        println!("Answer: {l} {answer} {answer2}");
    }
    println!("Total: {total} === 1969958987  total2: {total2}");
    // let num: u64 = 100_000_000;
    // Size of one stack frame for `factorial()` was measured experimentally
    // let _ = std::thread::Builder::new()
    //     .stack_size(num as usize * 0xFF)
    //     .spawn(move || {
    //         run(&data);
    //     })
    //     .unwrap()
    //     .join();
}

fn run(data: &Vec<i64>) -> (i64, i64) {
    // rows of numbers
    let len = data.len();
    let mut new_num: Vec<Vec<i64>> = vec![]; //.iter().map(|v| v.clone()).collect()];
    let mut current_row: Vec<i64> = data.clone(); //.iter().map(|v| v.clone()).collect();
    loop {
        let mut new_row: Vec<i64> = vec![];
        for t in current_row.windows(2) {
            new_row.push(t[1] - t[0]);
        }
        current_row = new_row.clone();
        new_num.push(new_row.clone());
        println!(" rows {:?}", new_row);
        if new_row.iter().all(|v| *v == 0) {
            break;
        }
    }
    // calc new num
    let mut new_value: i64 = data.iter().last().unwrap().clone();
    let mut new2: i64 = data[0];
    let mut sign: i64 = -1;
    for l in new_num.iter() {
        new_value += l.last().unwrap().clone();

        new2 += l[0] * sign;
        sign = sign * -1; //flip sign
    }
    
    (new_value, new2)
}
#[derive(Debug)]
struct Data {
    data: Vec<Vec<i64>>,
}
fn parse(input: &str) -> Data {
    Data {
        data: input
            .lines()
            .map(|l| l.split(" ").map(|v| v.parse().expect("Not num?")).collect())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        let data = vec![0, 3, 6, 9, 12, 15];
        //                    -3  3  3  3  3
        //                         0  0  0
        // lvl0 lookup
        assert_eq!(run(&data), (18, -3));
    }
}
