use nom;

pub fn run(input: &str) {
    println!("input: {:?}", input);
    let races: Vec<Race> = parse_races(&input);
    println!("parse {:#?}", races);
    let mut margin = 1;
    for (i, r) in races.iter().enumerate() {
        let (n_slower, n_eq, n_faster, max_distance, best_hold_time) = calc_race_distance(&r);
        println!(
            "Race {i} t={t} d_rec={d:>4} s:{n_slower}, e:{n_eq}, f:{n_faster}  result_total {r_total}  max_d {max_distance:>4} best_hold_t {best_hold_time}",
            i = i + 1,
            t = r.time_ms,
            d = r.distance_mm,
            r_total = n_slower + n_eq + n_faster,
        );
        margin = margin * n_faster;
    }
    println!(" margin: {margin}");
}

fn calc_race_distance(r: &Race) -> (u64, u64, u64, u64, u64) {
    let (mut n_slower, mut n_eq, mut n_faster) = (0, 0, 0);
    let mut max_distance = 0;
    let mut best_hold_time = 0;
    for hold_t in 1..r.time_ms {
        let my_distance = hold_t * (r.time_ms - hold_t);
        assert!(my_distance > 0, "0 distance ?");
        if max_distance < my_distance {
            max_distance = my_distance;
            best_hold_time = hold_t;
        };
        if r.distance_mm < my_distance {
            n_faster += 1;
        } else if r.distance_mm > my_distance {
            n_slower += 1;
        } else {
            n_eq += 1;
        }
    }
    (n_slower, n_eq, n_faster, max_distance, best_hold_time)
}

#[derive(Debug)]
struct Race {
    time_ms: u64,
    distance_mm: u64,
}

fn parse_races(input: &str) -> Vec<Race> {
    let parse_result = nom_parse(&input);
    match parse_result {
        Ok((input, races)) => {
            assert_eq!(input, "", "After parsing input should be empty.");
            races
        }
        Err(e) => panic!("Error reading input file {:#?}", e),
    }
}
fn nom_parse(input: &str) -> nom::IResult<&str, Vec<Race>> {
    let (input, tag) = nom::bytes::complete::tag("Time:")(input)?; //.expect("No Time: heading");
    let (input, space) = nom::character::complete::space1(input)?;
    let (input, times) = nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::digit1,
    )(input)?;
    let num_str: String = times.iter().map(|s| s.to_string()).collect();
    let times: Vec<u64> = vec![ num_str.parse().unwrap(), ];

    let (input, _) = nom::character::complete::line_ending(input)?;
    //#
    let (input, _) = nom::bytes::complete::tag("Distance:")(input)?;
    let (input, space) = nom::character::complete::space1(input)?;
    let (input, distances) = nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::digit1,
    )(input)?;
    let num_str: String = distances.iter().map(|s| s.to_string()).collect();
    let distances: Vec<u64> = vec![ num_str.parse().unwrap(), ];
    assert_eq!(
        times.len(),
        distances.len(),
        "Each race should have a time and distance."
    );

    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time_ms: t.clone(),
            distance_mm: d.clone(),
        })
        .collect();
    Ok((input, races))
}
