use nom;

pub fn run(input: &str) {
    println!("input: {:?}", input);
    let races: Vec<Race> = parse_races(&input);
    println!("parse {:#?}", races);
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
            races},
        Err(e) => panic!("Error reading input file {:#?}",e),
    }
}
fn nom_parse(input: &str) -> nom::IResult<&str, Vec<Race>> {
    let (input, tag) = nom::bytes::complete::tag("Time:")(input)?; //.expect("No Time: heading");
    let (input, space) = nom::character::complete::space1(input)?;
    let (input, times) = nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u64,
    )(input)?;
    let (input, _ ) = nom::character::complete::line_ending(input)?;
    //#
    let (input, _ ) = nom::bytes::complete::tag("Distance:")(input)?;
    let (input, space) = nom::character::complete::space1(input)?;
    let (input, distances) = nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::u64,
    )(input)?;
    assert_eq!(times.len(), distances.len(), "Each race should have a time and distance.");
    
    let races: Vec<Race> = times.iter().zip(distances.iter()).map(|(t,d)| Race { time_ms: t.clone(), distance_mm: d.clone()}).collect();
    Ok((
        input,
        races,
    ))
}
