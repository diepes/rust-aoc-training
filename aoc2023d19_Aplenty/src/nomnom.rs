// use nom::{
//     branch::alt,
//     character::complete::{char, digit1, multispace0},
//     combinator::{map, opt},
//     multi::separated_list1,
//     sequence::{delimited, tuple},
//     IResult,
// };

// pub fn nom_workflow(s: &str) -> IResult<(&str, &str)> {
//     // px{a<2006:qkq,m>2090:A,rfg}
//     let (s, name) = nom::character::complete::alpha1(s)?;
//     let (s, _) = nom::complete::tag("{")(s)?;
//     Ok((s, name))
// }
