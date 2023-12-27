use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    //multi::fold_many1,
    //sequence::Tuple,
    IResult,
    Parser,
};
fn main() {
    println!("Hello, world!");
    let mut id_sum = 0;
    let mut power = 0;
    for l in read_lines("in.txt").iter() {
        // println!("{:#?}", l);
        let (_, game) = parse_game(l).expect("Error parsing game :(");
        if check_game_possible(&game) {
            id_sum += game.number;
        }
        power += calc_game_power(&game);
        println!(
            "parse: {} sum:{} {:?}",
            check_game_possible(&game),
            id_sum,
            game
        );
    }
    println!("Total game power {}", power);
}
fn calc_game_power(g: &Game) -> usize {
    let (mut mr, mut mg, mut mb) = (0, 0, 0);
    for round in &g.round_scores {
        if round.red > mr {
            mr = round.red
        }
        if round.green > mg {
            mg = round.green
        }
        if round.blue > mb {
            mb = round.blue
        }
    }
    mr * mg * mb
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_number) = (get_number).parse(input)?;
    let (input, _) = tag(":")(input)?;
    let mut game = Game {
        number: game_number,
        round_scores: vec![],
    };
    let scores_strs: Vec<&str> = input.split(";").collect();
    // println!("dd0 Game {}  scores_str {:#?}", game_number, scores_strs);
    for score_str in scores_strs {
        let (_, score) = (parse_score).parse(score_str)?;
        //println!("dd1 Game {}  score {:#?}", game_number, score);
        game.round_scores.push(score);
    }
    Ok((input, game))
}
fn check_game_possible(g: &Game) -> bool {
    let (mr, mg, mb) = (12, 13, 14);
    for round in &g.round_scores {
        if round.red > mr || round.green > mg || round.blue > mb {
            return false;
        }
    }
    true
}
fn parse_score(input: &str) -> IResult<&str, Score> {
    let mut score = Score {
        blue: 0,
        green: 0,
        red: 0,
    };
    let games: Vec<&str> = input.split(",").collect();
    for g in games {
        let (i, _) = tag(" ")(g)?;
        let (i, number) = (get_number).parse(i)?;
        let (i, _) = tag(" ")(i)?;
        let (_, color) = take_while_m_n(3, 5, |c: char| c.is_alphabetic())(i)?;
        // println!("d1 parse single {} >> {}", color, number);
        match color {
            "red" => score.red = number,
            "green" => score.green = number,
            "blue" => score.blue = number,
            _ => panic!("Invalid color"),
        }
    }
    // println!("d2 input:{input} score:{:#?}<<", score);
    Ok(("", score))
}
fn get_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while_m_n(1, 3, is_digit), num_from_str).parse(input)
}
fn num_from_str(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse()
}
fn is_digit(c: char) -> bool {
    c.to_digit(10) != None
}
#[derive(Debug)]
struct Game {
    number: usize,
    //total_score: Score,
    pub round_scores: Vec<Score>,
}
#[derive(Debug)]
struct Score {
    red: usize,
    green: usize,
    blue: usize,
}
fn read_lines(file_name: &str) -> Vec<String> {
    let input = std::fs::read_to_string(file_name)
        .expect("Error reading file")
        .lines()
        .map(|l| l.to_string())
        .collect();
    input
}
