use nom::{
    branch::alt,
    // sequence::{delimited, tuple},
    bytes::complete::tag,
    character::complete::{alpha1, digit1}, //, char, multispace0},
    // combinator::{map, opt},
    multi::separated_list1, // many1},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub steps: Vec<Step>,
}
impl Workflow {
    pub fn parse(input: &str) -> (&str, HashMap<String, Vec<Step>>) {
        // e.g. px{a<2006:qkq,m>2090:A,rfg}
        //      pv{a>1716:R,A}
        let (input, workflows) =
            separated_list1(tag("\n"), nom_workflow)(input).expect("Err parsing Workflow's");
        assert_eq!(input, "", "input should be empty after Workflow parse.");
        // let unique: HashSet<char> = message.chars().collect();
        // unique
        //     .iter()
        //     .map(|&c| (c, message.matches(c).count()))
        //     .collect()
        let workflow_hash: HashMap<String, Vec<Step>> = workflows
            .iter()
            .map(|w| (w.name.clone(), w.steps.clone()))
            .collect();
        (input, workflow_hash)
    }
}
fn nom_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = nom_name_rule(input).expect("Rule name.");
    let (input, steps) = nom_steps(input).expect("Steps.");
    Ok((
        input,
        Workflow {
            name: name.to_string(),
            steps,
        },
    ))
}
fn nom_steps(input: &str) -> IResult<&str, Vec<Step>> {
    // let (input, steps) = separated_list1(tag(","), alt((nom_steps_rule, nom_steps_final)))(input)?;
    let (input, mut steps) = separated_list1(tag(","), nom_steps_rule)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, final_step) = nom_steps_final(input)?;
    steps.push(final_step);
    Ok((input, steps))
}
fn nom_steps_final(input: &str) -> IResult<&str, Step> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, Step::Final(name.to_string())))
}
fn nom_steps_rule(input: &str) -> IResult<&str, Step> {
    let (input, var) = alpha1(input)?;
    let (input, comparison) = alt((tag(">"), tag("<")))(input)?;
    let (input, value) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, next) = alpha1(input)?;
    Ok((
        input,
        Step::Condition(StepRule {
            var: var.to_string(),
            comparison: comparison.to_string(),
            value: value.parse().expect("Invalid number ?"),
            next: next.to_string(),
        }),
    ))
}

fn nom_name_rule(input: &str) -> IResult<&str, &str> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag("{")(input)?;
    Ok((input, name))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Step {
    Condition(StepRule),
    Final(String), // Next name e.g. rfg or A_ccept or R_eject
}
#[derive(Debug, PartialEq, Clone)]
pub struct StepRule {
    pub var: String,
    pub comparison: String,
    pub value: u64,
    pub next: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nom_steps_final() {
        let input = "rfg{s<537:gd,x>2440:R,A}";
        assert_eq!(nom_name_rule(input), Ok(("s<537:gd,x>2440:R,A}", "rfg")));
        let input = "A}";
        assert_eq!(
            nom_steps_final(input),
            Ok(("", Step::Final("A".to_string())))
        );
        let input = "s<537:gd,x>2440:R,A}";
        assert!(nom_steps_final(input).is_err());
        assert_eq!(
            nom_steps_rule(input),
            Ok((
                ",x>2440:R,A}",
                Step::Condition(StepRule {
                    var: "s".to_string(),
                    comparison: "<".to_string(),
                    value: 537,
                    next: "gd".to_string(),
                })
            ))
        );
    }
}
