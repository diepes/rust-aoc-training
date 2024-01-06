use crate::*;
use num::Integer;
use std::time::Instant;

#[derive(Debug, PartialEq, PartialOrd)]
struct Cycle {
    names: Vec<String>,
    jump_from_start_to_cycle: u64,
    cycle_jump: u64,
}
fn calc(a: Cycle, b: Cycle) -> Cycle {
    // Takes two cycles and combines them into one bigger cycle
    //  @ Pos    (Pos - a.start) / a.cycle_jump = Int

    let mut cnt_a = a.jump_from_start_to_cycle;
    let mut cnt_b = b.jump_from_start_to_cycle;
    let (mut cnt_a_jumps, mut cnt_b_jumps) = (0_u64, 0_u64);
    let mut show_progress = 0_u64;
    let mut combine_names: Vec<String> = Vec::new();

    combine_names.extend(a.names);
    combine_names.extend(b.names);

    let t_start = Instant::now();
    if cnt_a.div_mod_floor(&cnt_b).1 == 0 {
        panic!("cnt_a:{cnt_a} is divisable by cnt_b:{cnt_b}");
    };
    while cnt_a != cnt_b {
        while cnt_a < cnt_b {
            cnt_a += a.cycle_jump;
            cnt_a_jumps += 1;
        }
        // as b is normally much smaller speed-up with intiger division.
        if cnt_a > (cnt_b + b.cycle_jump) {
            let div = (cnt_a - cnt_b) / b.cycle_jump; //get lower multiple
            cnt_b += b.cycle_jump * div;
            cnt_b_jumps += 1;
        }
        if cnt_a > cnt_b {
            cnt_b += b.cycle_jump;
            cnt_b_jumps += 1;
        };
        show_progress += 1;
        if show_progress > 1_000_000_000
            || cnt_a == cnt_b
            || num::abs(cnt_a as i64 - cnt_b as i64) < 200
        {
            show_progress = 0;
            let time_total = t_start.elapsed().as_secs() + 1;
            let rate = cnt_a / time_total;
            let diff = num::abs(cnt_a as i64 - cnt_b as i64);
            if rate > 0 {
                let rate_ns = rate / (1000 * 1000 * 1000);
                println!("{combine_names:?} pos {cnt_a} , a x {cnt_a_jumps} , b x {cnt_b_jumps} , a_cycle:{a_cycle} b_cycle:{b_cycle} diff:{diff:>3} , time: {time}s , rate: {rate_ns}/nanosec",
                            time=time_total, a_cycle=a.cycle_jump,b_cycle=b.cycle_jump);
            };
        };
    }
    println!("Found cycle @ step: {cnt_a}  a_loops: {cnt_a_jumps} b_loops: {cnt_b_jumps} {combine_names:?}");

    Cycle {
        names: combine_names,
        jump_from_start_to_cycle: cnt_a,
        cycle_jump: a.cycle_jump.lcm(&b.cycle_jump),
    }
}

fn run_to_cycle(run: &Run) -> Cycle {
    Cycle {
        names: vec![run.run_id.to_string()],
        jump_from_start_to_cycle: run.step_initial,
        cycle_jump: run.step_size,
    }
}
pub fn doit(runs: Vec<Run>) {
    let mut result = run_to_cycle(&runs[0]);
    for r in 1..runs.len() {
        result = calc(result, run_to_cycle(&runs[r]));
        println!("merged cycle r:{r} {result:#?}");
    }
}

fn combine_phased_rotations(
    a_per: u64,
    a_ph: u64,
    b_per: u64,
    b_ph: u64,
) -> Result<(u64, u64, u64), Box<dyn std::error::Error>> {
    /*  Combine two phased rotations into a single phased rotation

    Returns: combined_period, combined_phase

    The combined rotation is at its reference point if and only if both a and b
    are at their reference points.
    (3, 50, 6, 20)
    */
    // if a_per == b_per {

    //     let err_msg:Box<dyn std::error::Error> = format!("Rotation reference points never synchronize. A[{a_per}, {a_ph}]  B[{b_per}, {b_ph}] {pd_remainder}").into();
    //     return Err(err_msg);
    //     return Ok( ( u64::try_from(a_per)?, u64::try_from(a_ph)? ) );
    // }
    let start_ph: u64;
    let a_phase: i64;
    let b_phase: i64;

    if a_ph > b_ph {
        // [9,6 . 15,0]
        // b need to catchup
        let ph_diff = a_ph - b_ph; // 6
        b_phase = 0_i64;
        let (remain_div_b, remain_remainder) = (ph_diff).div_rem(&b_per); // 0,6 (6)div(15)
        a_phase = -i64::try_from(remain_remainder)?; //-6
        start_ph = a_ph - ph_diff + remain_div_b * b_per; // 6 + 0
    } else {
        let ph_diff = b_ph - a_ph; // -6
        a_phase = 0_i64;
        let (remain_div_a, remain_remainder) = (ph_diff).div_rem(&a_per); //
        b_phase = -i64::try_from(remain_remainder)?;
        start_ph = b_ph - ph_diff + remain_div_a * a_per;
    }

    let (a_period, b_period) = (i64::try_from(a_per)?, i64::try_from(b_per)?);
    let (combined_period, mut combined_phase) =
        phase_combine(a_period, a_phase, b_period, b_phase)?;
    while combined_phase < 0 {
        combined_phase += combined_period;
    }
    Ok((
        u64::try_from(combined_period)?,
        u64::try_from(combined_phase)? + start_ph,
        start_ph,
    ))
}

fn phase_combine(
    a_period: i64,
    a_phase: i64,
    b_period: i64,
    b_phase: i64,
) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let e = i64::extended_gcd(&a_period, &b_period);
    let (gcd, s, _t) = (e.gcd, e.x, e.y);
    let phase_difference = a_phase - b_phase;
    let (pd_mult, pd_remainder) = phase_difference.div_rem(&gcd);
    if pd_remainder != 0 {
        let err_msg:Box<dyn std::error::Error> = format!("Rotation reference points never synchronize. A[{a_period}, {a_phase}]  B[{b_period}, {b_phase}] {pd_remainder}").into();
        return Err(err_msg);
    };

    let combined_period = a_period.div_floor(&gcd) * b_period;
    let combined_phase = (a_phase - s * pd_mult * a_period) % combined_period;
    //let combined_phase = a_phase - s * pd_mult * a_period;
    Ok((combined_period, combined_phase))
}

#[cfg(test)]
mod tests {

    use super::*;

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

        assert_eq!(
            calc(a, b),
            Cycle {
                names: vec!["a".to_string(), "b".to_string()],
                jump_from_start_to_cycle: 11,
                cycle_jump: 12
            }
        );
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

        assert_eq!(
            calc(a, b),
            Cycle {
                names: vec!["a".to_string(), "b".to_string()],
                jump_from_start_to_cycle: 51025,
                cycle_jump: 101000
            }
        );
    }
    // #[test]
    // fn test_calc3() {
    //     // Check that same cycle does not change when combined.
    //     let a = Cycle {
    //         names: vec!["a".to_string()],
    //         jump_from_start_to_cycle: 20,
    //         cycle_jump: 105,
    //     };
    //     let b = Cycle {
    //         names: vec!["b".to_string()],
    //         jump_from_start_to_cycle: 20,
    //         cycle_jump: 105,
    //     };

    //     assert_eq!(
    //         calc(a, b),
    //         Cycle {
    //             names: vec!["a".to_string(), "b".to_string()],
    //             jump_from_start_to_cycle: 20,
    //             cycle_jump: 105
    //         }
    //     );
    // }

    #[test]
    fn test_num_extendedgcd() {
        // ExtendedGcd
        // https://reference.wolfram.com/language/ref/ExtendedGCD.html
        // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
        let a: i64 = 10;
        let b: i64 = 16;
        let e = i64::extended_gcd(&a, &b);
        assert_eq!((e.gcd, e.x, e.y), (2, -3, 2));
        assert_eq!(e.gcd, e.x * a + e.y * b);
    }

    #[test]
    #[should_panic]
    fn test_combine_phased_rotations_fail() {
        assert_eq!(combine_phased_rotations(10, 1, 10, 0).unwrap(), (10, 0, 1));
    }

    #[test]
    fn test_div_rem() {
        assert_eq!(10.div_rem(&3), (3, 1));
        assert_eq!(10.div_rem(&10), (1, 0));
        assert_eq!(10.div_rem(&20), (0, 10)); // ??? Why???
    }

    #[test]
    fn test_phase_combine() {
        assert_eq!(phase_combine(9, -3, 15, 0).unwrap(), (45, 15)); // 30 = 9*3+3
        assert_eq!(phase_combine(9, 0, 15, 3).unwrap(), (45, 18)); // 18 = 9*2 = 15+3
        assert_eq!(phase_combine(9, 0, 15, -3).unwrap(), (45, -18)); // 45-18 = 27 = 9*3+3 = 15*2 -3
        assert_eq!(phase_combine(9, 3, 15, 0).unwrap(), (45, -15)); // 45-15 = 30 = 9*3+3 = 15*2
        assert_eq!(phase_combine(9, -6, 15, 0).unwrap(), (45, 30));
        assert_eq!(phase_combine(9, 6, 15, 0).unwrap(), (45, -30));
        assert_eq!(phase_combine(9, 0, 15, 0).unwrap(), (45, 0));
        assert_eq!(phase_combine(15, 0, 9, -6).unwrap(), (45, 30));
        assert_eq!(phase_combine(9, -9, 15, 0).unwrap(), (45, 0)); //
        assert_eq!(phase_combine(9, 0, 15, -6).unwrap(), (45, -36)); // 36 = 9*4 = 15*2 + 6
        assert_eq!(phase_combine(15, 6, 9, 0).unwrap(), (45, 36)); // 36 = 9*4 = 15*2 + 6
    }

    #[test]
    fn test_combine_phased_rotations() {
        assert_eq!(combine_phased_rotations(3, 30, 6, 00).unwrap(), (6, 30, 30));

        //assert_eq!(combine_phased_rotations(3, 33, 6, 00).unwrap(), (6, 33, 33));

        assert_eq!(combine_phased_rotations(3, 50, 6, 20).unwrap(), (6, 50, 50)); // 50 = 6*5+20 = 3*0 + 50

        assert_eq!(combine_phased_rotations(9, 9, 15, 15).unwrap(), (45, 18, 9)); //
        assert_eq!(combine_phased_rotations(9, 0, 15, 6).unwrap(), (45, 9, 0)); //

        assert_eq!(
            combine_phased_rotations(9, 21, 15, 15).unwrap(),
            (45, 45, 15)
        ); //45 = 9*3=27+21=48 27+ 21

        assert_eq!(combine_phased_rotations(9, 6, 15, 0).unwrap(), (45, 15, 6)); //45,36

        assert_eq!(combine_phased_rotations(10, 0, 10, 0).unwrap(), (10, 0, 0)); // no change

        assert_eq!(
            combine_phased_rotations(10, 10, 10, 0).unwrap(),
            (10, 10, 10)
        ); // no change
        assert_eq!(
            combine_phased_rotations(10, 10, 10, 10).unwrap(),
            (10, 10, 10)
        ); // no change

        assert_eq!(combine_phased_rotations(3, 9, 6, 12).unwrap(), (6, 12, 12)); // @12 = AAA + 1xRot ZZZ
        assert_eq!(combine_phased_rotations(3, 3, 6, 12).unwrap(), (6, 12, 12)); // @12 = AAA + 1xRot ZZZ
        assert_eq!(combine_phased_rotations(3, 0, 6, 12).unwrap(), (6, 12, 12)); // @12 = AAA + 1xRot ZZZ
        assert_eq!(combine_phased_rotations(6, 0, 3, 12).unwrap(), (6, 12, 12)); // @12 = AAA + 1xRot ZZZ
        assert_eq!(combine_phased_rotations(15, 1, 9, 1).unwrap(), (45, 1, 1));
        assert_eq!(combine_phased_rotations(9, 9, 15, 0).unwrap(), (45, 54, 9)); // ??
        assert_eq!(combine_phased_rotations(9, 6, 15, 0).unwrap(), (45, 36, 6));
        // print(arrow_alignment(red_len=30, green_len=38, advantage=6))  # 120
        // print(arrow_alignment(red_len=9, green_len=12, advantage=5))  # ValueError
    }
}
