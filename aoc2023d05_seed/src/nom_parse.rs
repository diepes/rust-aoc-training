//use crate::Card;
//use nom;
use nom::{
    bytes::complete::tag, character, multi::separated_list1, sequence::separated_pair, IResult,
};
#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    pub maps: Vec<Map>,
}
#[derive(Debug, PartialEq)]
pub struct Map {
    pub name: String,
    pub from: String,
    pub to: String,
    pub entries: Vec<MapEntry>,
}
impl Map {
    pub fn new(name: &str, from: &str, to: &str, entries: Vec<MapEntry>) -> Map {
        Map {
            name: name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            entries,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MapEntry {
    pub src: u64,
    pub dst: u64,
    pub range: u64,
    src_below: u64,
    src_above: u64,
}
impl MapEntry {
    pub fn new(src: u64, dst: u64, range: u64) -> MapEntry {
        MapEntry {
            src: src,
            dst: dst,
            range: range,
            src_below: src - 1,     // 1 less than start of range
            src_above: src + range, // 1 more than largest value
        }
    }
}

pub fn parse_map(input_lines: &str) -> Almanac {
    let mut almanac = Almanac {
        seeds: vec![],
        maps: vec![],
    };
    for line in input_lines.lines() {
        let mut input; //define so next step no let, add val to exising var.
        (input, almanac.seeds) = parse_seeds(&line).expect("ERROR Parsing seeds");
        println!("# parse_map seeds {:#?}", almanac.seeds);
        (input, almanac.maps) = parse_maps_full(&input).expect("ERROR Parsing seed maps");
        println!("# parse_map maps {:#?}", almanac.maps);
    }
    almanac
}

fn parse_maps_full(input: &str) -> IResult<&str, Vec<Map>> {
    // map blocks
    let mut maps: Vec<Map> = vec![];
    let (input, m_from) = character::complete::multispace0(input)?;
    let (input, map_entry) = separated_list1(tag("-"), character::complete::alpha1)(input)?;
    assert_eq!(map_entry.len(), 2, "Error map name not from-to-to ");
    let (input, _) = tag(" map:")(input)?;
    //let (input, _) = character::complete::multispace1(input)?;
    //let (input, id) = character::complete::u32(input)?;
    let (input, map_entry) = separated_list1(tag(" "), character::complete::u32)(input)?;
    assert_eq!(map_entry.len(), 3, "Error map len != 3");
    Ok((input, maps))
}
fn parse_map_single(input: &str) -> IResult<&str, Map> {
    let (input, (from, to)) = parse_map_block_header(&input)?;
    let (input, entries) = separated_list1(character::complete::newline, parse_map_entry)(input)?;
    Ok((
        input,
        Map::new(&format!("{from}-to-{to}"), from, to, entries),
    ))
}

fn parse_map_block_header(input: &str) -> IResult<&str, (&str, &str)> {
    // parse "seed-to-soil map: -> ("seed","soil")
    let (input, map_header) = separated_list1(tag("-"), character::complete::alpha1)(input)?;
    assert_eq!(map_header.len(), 3, "Error map name not from-to-to ");
    assert_eq!(map_header[1], "to", "Error map name no -to- ");
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = nom::character::complete::line_ending(input)?;
    Ok((input, (map_header[0], map_header[2])))
}

fn parse_map_entry(input: &str) -> IResult<&str, MapEntry> {
    // parse "37 52 2" -> (37, 52, 2)
    let (input, out) = separated_list1(tag(" "), character::complete::u64)(input)?;
    assert_eq!(out.len(), 3, "[[3 values for MapEntry]]");
    let entry = MapEntry::new(out[0], out[1], out[2]);
    let (input, _) = nom::character::complete::line_ending(input)?;
    Ok((input, entry))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = character::complete::multispace0(input)?;
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = character::complete::multispace1(input)?;
    //let (input, id) = character::complete::u32(input)?;
    let (input, seed_list) = separated_list1(tag(" "), character::complete::u32)(input)?;
    Ok((input, seed_list))
}
fn nom_test_1(input: &str) -> IResult<&str, &str> {
    let (input, out) = nom::sequence::preceded(tag("abc"), tag("efg"))(input)?;
    Ok((input, out))
}

#[cfg(test)]
mod tests {
    //use crate::*;
    use super::*;
    static input1: &str = "seed-to-soil map:
50 98 2
52 50 48

fertilizer-to-water map:
49 53 8
";
    #[test]
    fn test_nom() {
        assert_eq!(nom_test_1("abcefg"), Ok(("", "efg")));
    }

    #[test]
    fn test_parse_map_single() {
        let (input, out) = parse_map_single(&input1).expect("TEST parse_map-single");
        let m1 = MapEntry::new(50, 98, 2);
        let v1 = vec![m1];
        assert_eq!(out, Map::new("seed-to-soil","seed", "soil", v1) );
        assert_eq!(input, "52 50 48\n\nfertilizer-to-water map:\n49 53 8\n");
    }

    #[test]
    fn test_parse_map_block_header() {
        let (input, out) = parse_map_block_header(&input1).expect("TEST_ERR map header");
        assert!(input.starts_with("50 98 2\n"));
        assert_eq!(out, ("seed", "soil"), "[[map header]]");
        //
        let (input, out) = parse_map_entry(&input).expect("TEST_ERR map values #1");
        assert!(input.starts_with("52 50 48\n"));
        assert_eq!(
            out,
            MapEntry::new(50_u64, 98_u64, 2_u64),
            "[[single map entry 3 values]]"
        );
        let (input, out) = parse_map_entry(&input).expect("TEST_ERR map values #2");
        assert!(input.starts_with("\nfertilizer-to-"));
        assert_eq!(
            out,
            MapEntry::new(52_u64, 50_u64, 48_u64),
            "[[single map entry 3 values]]"
        );
    }
}
