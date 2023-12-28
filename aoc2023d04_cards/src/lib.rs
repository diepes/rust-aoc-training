pub mod nom_parse;

pub fn run(input: &str) {
    println!("input \n{}", input);
    let cards = nom_parse::parse_cards(&input);
    println!("cards \n{:?}", cards);
    let mut cnt_total = 0;
    for n in 0..cards.len() {
        //let mut cnt_win = cnt_card_matches(c);
        //println!("{cnt_win}");
        cnt_total += get_num_cards(&cards[n..]);
    }
    println!("Total: {cnt_total}");
}
fn get_num_cards(cards: &[nom_parse::Card]) -> usize {
    let mut cards_cnt = 1; // current card
    // number of cards to copy
    let extra_cards = cnt_card_matches(&cards[0]);
    for n in 1..=extra_cards {
        cards_cnt += get_num_cards(&cards[n..])
    }
    cards_cnt
}

fn cnt_card_matches(c: &nom_parse::Card) -> usize {
    let mut cnt_win = 0;
    for n in c.numbers {
        if c.winning.contains(&n) {
            cnt_win += 1;
        }
    }
    cnt_win
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_get_num_cards() {
        let test_in = " Card 1: 41 48 83 86 17 99 99 99 99 99 | 83 86  6 31 17  9 48 53 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                        Card 2: 13 32 20 16 61 99 99 99 99 99 | 61 30 68 82 17 32 24 19 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                        Card 3:  1 21 53 59 44 99 99 99 99 99 | 69 82 63 72 16 21 14  1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                        Card 4: 41 92 73 84 69 99 99 99 99 99 | 59 84 76 51 58  5 54 83 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                        Card 5: 87 83 26 28 32 99 99 99 99 99 | 88 30 70 12 93 22 82 36 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                        Card 6: 31 18 13 56 72 99 99 99 99 99 | 74 77 10 23 35 67 36 11 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let cards = nom_parse::parse_cards(&test_in);
        assert_eq!(cnt_card_matches(&cards[0]),4); //#1 = 4
        assert_eq!(cnt_card_matches(&cards[1]),2); //#2 = 2 (last 3 is zero)
        assert_eq!(cnt_card_matches(&cards[2]),2); //#3 = 2 (last 3 is zero)
        assert_eq!(cnt_card_matches(&cards[3]),1); //#4 = 2 (last 3 is zero)
        assert_eq!(cnt_card_matches(&cards[4]),0); //#5 = 2 (last 3 is zero)
        assert_eq!(cnt_card_matches(&cards[5]),0); //#6 = 2 (last 3 is zero)
        // Game #1 + 2 + 3
        assert_eq!(get_num_cards(&cards),15);  //14 //#1=1 + 4#1(2,3,4,5) +2#2(3,4) +2x2#3(4,5) +3x1#4(5) +0#5
        assert_eq!(get_num_cards(&cards[1..]),7);
        assert_eq!(get_num_cards(&cards[2..]),4);
        assert_eq!(get_num_cards(&cards[3..]),2);
        assert_eq!(get_num_cards(&cards[4..]),1);
        assert_eq!(get_num_cards(&cards[5..]),1);

    }

    fn test_cnt_card_matches() {
        let test_in = " Card 197: 38  3 57 72 97 45 66 73 56  8 | 83 68 28 64 58 66 85 15 53 65 23  3 37 87 20 17 47 63 55 69 88 70 62 92 76
                        Card 198: 98 66 29 17 83  9  6 84 36 70 | 21 10 31 84 93 14 67 29 24 91 12 41 99 19  5 56 83 74  2  8 79 95 64 49 53
                        Card 199: 0 0 0 0 0 0 0 0 0 0 | 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
                        Card 200: 0 0 0 0 0 0 0 0 0 0 | 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
                        Card 201: 0 0 0 0 0 0 0 0 0 0 | 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1";
        let cards = nom_parse::parse_cards(&test_in);
        assert_eq!(cnt_card_matches(&cards[0]),2);
        assert_eq!(cnt_card_matches(&cards[1]),3);
        assert_eq!(cnt_card_matches(&cards[2]),0);
        assert_eq!(cnt_card_matches(&cards[3]),0);
        assert_eq!(cnt_card_matches(&cards[4]),0);
        let m_cnt = cnt_card_matches(&nom_parse::Card {
            id: 169,
            winning: [1, 2, 22, 2, 22, 2, 22, 3, 33, 3],
            numbers: [
                00, 1, 5, 2, 5, 5, 5, 5, 5, 5, 5, 55, 55, 5, 5, 5, 5, 55, 5, 5, 5, 5, 5, 5, 5,
            ],
        });
        assert_eq!(m_cnt, 2);
    }
}
