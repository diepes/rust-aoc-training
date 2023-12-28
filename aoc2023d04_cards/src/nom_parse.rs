//use crate::Card;
//use nom;
use nom::{
    bytes::complete::tag, character, multi::separated_list1, sequence::separated_pair, IResult,
};
const WIN_LEN: usize = 10;
const NUM_LEN: usize = 25;

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning: [u32; WIN_LEN],
    pub numbers: [u32; NUM_LEN],
}

pub fn parse_cards(input_lines: &str) -> Vec<Card> {
    let mut cards = vec![];
    for line in input_lines.lines() {
        let (input, id) = parse_card_id(&line).expect("ERROR Parsing card id");
        let (input, winning) = parse_card_winning(&input).expect("ERROR Parsing winnning numbers");
        let (input, numbers) = parse_card_numbers(&input).expect("ERROR Parsing numbers");
        // println!("Debug id={id} winning={:#?}",winning);
        cards.push(Card {
            id: id,
            winning: winning,
            numbers: numbers,
        })
    }
    cards
}

fn parse_card_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = character::complete::multispace0(input)?;
    let (input, _) = tag("Card")(input)?;
    let (input, _) = character::complete::multispace1(input)?;
    let (input, id) = character::complete::u32(input)?;
    Ok((input, id))
}

fn parse_card_winning(input: &str) -> IResult<&str, [u32; WIN_LEN]> {
    let (mut input, _) = tag(":")(input)?;
    let mut result = [0; WIN_LEN];
    let mut v;
    for val in result.iter_mut() {
        (input, _) = character::complete::multispace1(input)?;
        (input, v) = character::complete::u32(input)?;
        *val = v;
        //println!("input: {}",input)
    }
    Ok((input, result))
}
fn parse_card_numbers(input: &str) -> IResult<&str, [u32; NUM_LEN]> {
    let (mut input, _) = character::complete::multispace1(input)?;
    (input, _) = tag("|")(input)?;
    let mut result = [0; NUM_LEN];
    let mut v;
    for val in result.iter_mut() {
        (input, _) = character::complete::multispace1(input)?;
        (input, v) = character::complete::u32(input)?;
        *val = v;
        // println!("input: {}",input)
    }
    Ok((input, result))
}
// fn parse_section(input: &str) -> IResult<&str, RangeInclusive<u32>> {
//     let (input, start) = character::complete::u32(input)?;
//     let (input, _) = tag("-")(input)?;
//     let (input, end) = character::complete::u32(input)?;
//     Ok((input, start..=end))
// }
