use core::cmp::Ordering;
use nom;

pub fn run(input: &str) {
    println!("input: {:?}", input);
    let mut games: Vec<Game> = parse_games(&input);
    games.sort();
    let mut total = 0;
    for (i, game) in games.iter_mut().enumerate() {
        let rank = i as u64 + 1;
        game.rank = rank;
        total = total + rank * game.bet;
    }
    println!("parse {:#?}", games);
    println!("total: {total} . == 6440 != ⬆️252,389,310 ⬇️249,770,018 ⬇️248,611,190");
}

// In order to make Game sortable you need four traits Eq, PartialEq, Ord and PartialOrd.
#[derive(Debug, Eq)]
struct Game<'a> {
    cards: &'a str,
    card_values: [u32; 5],
    bet: u64,
    rank: u64,
    hand: HandType,
}
impl Ord for Game<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand != other.hand {
            return self.hand.cmp(&other.hand).reverse();
        } else {
            // case where we have same hand, now highest card wins.
            for i in 0..self.card_values.len() {
                let co = &other.cards.chars().nth(i).unwrap();
                if self.card_values[i] == other.card_values[i] {
                    continue;
                };
                return self.card_values[i].cmp(&other.card_values[i]);
            }
        };
        panic!(
            "Error in card compare {:?} {:?}",
            self.card_values, other.card_values
        );
    }
}

impl PartialOrd for Game<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //Some(self.hand.cmp(&other.hand))
        Some(self.cmp(other))
    }
}

impl PartialEq for Game<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl<'a> Game<'a> {
    fn new(cards: &'a str, bet: u64) -> Game {
        let mut card_values = [0_u32; 5];
        for (i, c) in cards.chars().enumerate() {
            card_values[i] = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1, //joker < 2
                'T' => 10,
                '2'..='9' => c.to_digit(10).unwrap() - '0'.to_digit(10).unwrap(),
                _ => panic!(" hand {cards:?} contain invalid card {c}"),
            }
        }
        Game {
            cards: cards,
            card_values: card_values,
            bet: bet,
            rank: 0,
            hand: HandType::calc_hand_type(&cards),
        }
    }
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandType {
    fn calc_hand_type(cards: &str) -> HandType {
        // part 2 nor J = Jokers. 🤯
        let mut hand = HandType::HighCard;
        let mut found = ['🦀'; 4]; // skip cards we already counted
        let mut count_jokers = 0;
        for (i, c) in cards.chars().enumerate() {
            let mut cnt = 1; //start with this card
            if found.contains(&c) {
                //skip card already matched
                continue;
            };
            if c == 'J' {
                //skip joker and count
                count_jokers += 1;
                continue;
            }
            for cm in cards[(i + 1)..5].chars() {
                if cm == 'J' {
                    //skip joker match
                    continue;
                }
                if c == cm {
                    cnt += 1;
                    found[i] = c;
                }
            }
            match cnt {
                5 => hand = HandType::FiveOfAKind,
                4 => hand = HandType::FourOfAKind,
                3 => {
                    if hand == HandType::OnePair {
                        hand = HandType::FullHouse;
                    } else {
                        assert_eq!(
                            hand,
                            HandType::HighCard,
                            "If not fullhouse, should be ::HighCard hand {}, i:{i}, c:{c}, cnt:{cnt}, {:?}" ,cards, found
                        );
                        hand = HandType::ThreeOfAKind;
                    }
                }
                2 => {
                    if hand == HandType::OnePair {
                        hand = HandType::TwoPair;
                    } else if hand == HandType::ThreeOfAKind {
                        hand = HandType::FullHouse;
                    } else {
                        hand = HandType::OnePair;
                    }
                }
                x => {
                    assert_eq!(x, 1, "Should only be this card");
                    println!(" Skip single {c} ...");
                }
            };
            println!(" Debug {cards:?} c:{c}, cnt:{cnt}, hand:{hand:#?}");
        }
        // handle jokers, and return upgraded hand
        match (hand, count_jokers) {
            (hand, 0) => hand, // no jokers no change Inc FiveOfAKind and Fullhouse
            (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind,
            (HandType::TwoPair, 1) => HandType::FullHouse,
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,
            (HandType::OnePair, 2) => HandType::FourOfAKind,
            (HandType::OnePair, 3) => HandType::FiveOfAKind,
            (HandType::HighCard, 1) => HandType::OnePair,
            (HandType::HighCard, 2) => HandType::ThreeOfAKind,
            (HandType::HighCard, 3) => HandType::FourOfAKind,
            (HandType::HighCard, 4) => HandType::FiveOfAKind,
            (HandType::HighCard, 5) => HandType::FiveOfAKind,
            _ => panic!("Invalid hand + jokers {cards:?}  Jokers {count_jokers}"),
        }
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    let parse_result = nom_parse(&input);
    match parse_result {
        Ok((input, games)) => {
            assert_eq!(input, "", "After parsing input should be empty.");
            games
        }
        Err(e) => panic!("Error reading input file {:#?}", e),
    }
}
fn nom_parse(input: &str) -> nom::IResult<&str, Vec<Game>> {
    //let mut games: Vec<Game> = vec![];
    let (input, games) =
        nom::multi::separated_list1(nom::character::complete::multispace1, nom_parse_one)(input)?;
    Ok((input, games))
}
fn nom_parse_one(input: &str) -> nom::IResult<&str, Game> {
    let (input, cards) = nom::character::complete::alphanumeric1(input)?; //.expect("No Time: heading");
    assert_eq!(cards.len(), 5, "Expect 5 cards in each hand");
    let (input, space) = nom::character::complete::space1(input)?;
    let (input, bet) = nom::character::complete::u64(input)?;

    Ok((input, Game::new(cards, bet)))
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_logic() {
        let s = "12345AKQJT9";
        for i in 0..(s.len() - 1) {
            let c1 = s.chars().nth(i).unwrap();
            let c2 = s.chars().nth(i + 1).unwrap();
            assert_eq!(c1.cmp(&c2), std::cmp::Ordering::Less, "ERROR! {c1} {c2}");
        }
    }
}
