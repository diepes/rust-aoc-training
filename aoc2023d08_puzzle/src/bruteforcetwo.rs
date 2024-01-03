use num::Integer;

#[derive(Debug,PartialEq, PartialOrd)]
struct Cycle {
    names: Vec<String>,
    jump_from_start_to_cycle: u64,
    cycle_jump: u64,
}
fn calc(a: Cycle , b: Cycle) -> Cycle {
    // Takes two cycles and combines them into one bigger cycle
    //  @ Pos    (Pos - a.start) / a.cycle_jump = Int

    let mut cnt_a = a.jump_from_start_to_cycle;
    let mut cnt_b = b.jump_from_start_to_cycle;
    let (mut cnt_a_jumps, mut cnt_b_jumps) = (0_u64, 0_u64);

    while cnt_a != cnt_b {
        if cnt_a < cnt_b {
            cnt_a += a.cycle_jump;
            cnt_a_jumps += 1;

        } else {
            cnt_b += b.cycle_jump;
            cnt_b_jumps += 1;
        }
    println!("a {cnt_a} , b {cnt_b}");
    };
    println!("Found cycle @ step: {cnt_a}  a_loops: {cnt_a_jumps} b_loops: {cnt_b_jumps} ");
    let mut vec3: Vec<String> = Vec::new();
    
    vec3.extend(a.names);
    vec3.extend(b.names);
    Cycle {
        names: vec3,
        jump_from_start_to_cycle: cnt_a,
        cycle_jump: a.cycle_jump.lcm(&b.cycle_jump),

    }
}

fn find_same_step(a_start: usize, a_inc: usize, b_start: usize, b_inc: usize) -> usize {
    let lcm = a_inc.lcm(&b_inc);
    let diff = b_start - a_start;

    diff / lcm
}
fn find_same_step2(a_start: usize, a_inc: usize, b_start: usize, b_inc: usize) -> Option<usize> {
    let gcd = a_inc.gcd(&b_inc);
    let diff = b_start as isize - a_start as isize;

    if diff % (gcd as isize) == 0 {
        Some((diff.abs() / gcd as isize) as usize)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_same_step() {
        let a_start = 3;
        let a_inc = 4;
        let b_start = 5;
        let b_inc = 6;
    
        let same_step = find_same_step(a_start, a_inc, b_start, b_inc);
        println!("Both cycles reach the same desired end value at step: {}", same_step);
        assert_eq!(same_step,0);
    }
    #[test]
    fn test_calc1() {
        let a = Cycle {
            names: vec!["a".to_string()],
            jump_from_start_to_cycle: 3,
            cycle_jump: 4,
        };
        let b = Cycle {
            names: vec!["b".to_string()],
            jump_from_start_to_cycle: 5,
            cycle_jump: 6,
        };

        assert_eq!(calc(a,b), Cycle { names: vec!["a".to_string(), "b".to_string()], 
jump_from_start_to_cycle:11, cycle_jump:12}) ;

        // match find_same_step(a_start, a_inc, b_start, b_inc) {
        //     Some(same_step) => println!("Both cycles reach the same desired end value at step: {}", same_step),
        //     None => println!("Cycles do not align within the provided range of steps."),
        // }
    }
    #[test]
    fn test_calc2() {
        let a = Cycle {
            names: vec!["a".to_string()],
            jump_from_start_to_cycle: 20,
            cycle_jump: 101,
        };
        let b = Cycle {
            names: vec!["b".to_string()],
            jump_from_start_to_cycle: 25,
            cycle_jump: 1000,
        };

        assert_eq!(calc(a,b), Cycle { names: vec!["a".to_string(), "b".to_string()], 
jump_from_start_to_cycle:51025, cycle_jump:101000}) ;

        // match find_same_step(a_start, a_inc, b_start, b_inc) {
        //     Some(same_step) => println!("Both cycles reach the same desired end value at step: {}", same_step),
        //     None => println!("Cycles do not align within the provided range of steps."),
        // }
    }
}