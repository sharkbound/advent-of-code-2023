#[allow(unused_variables)]

use std::fmt::{Display, Formatter};
use nom::character::complete::{alphanumeric1, digit1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use daytemplate::{Day, DayPart};
use rustutils::{join_to_string};
use rustutils::map_to::MapToExt;
use rustutils::nom_helpers::consume_empty_space;


pub struct Day7Part2 {}

impl Day for Day7Part2 {
    type ParseOutput = Vec<Hand>;

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        7
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let nom_parsed = nom_parse(input);
        return nom_parsed.unwrap().1;
    }


    fn solve(&self) {
        let input = self.sample("part_1");
        // let input = self.input();
        let parsed = self.parse(&input);
        let mut processed = parsed.iter().map(process_hand).collect::<Vec<_>>();

        processed.sort_by_key(cmp_calculated_hand_result);

        // println!("{}", join_to_string!(&processed, "\n"));
        let mut score = 0_u64;
        for (hand, multiplier) in processed.iter().zip((1..=processed.len()).rev()) {
            score += hand.hand.bet as u64 * multiplier as u64;
            // println!("{} | {}: {}x", hand, hand.hand.bet, multiplier);
        }
        println!("Day 7 Part 2: {}", score);
        // wrong: 255419998
    }
}

fn cmp_calculated_hand_result(hand: &CalculatedHandResult) -> (i32, [i32; 5]) {
    let cmp_card_val = |hand: &Hand, idx: usize| -(hand.cards[idx].value as i32);
    (-(hand_score_with_upgraded_jokers(hand).score as i32),
     [cmp_card_val(&hand.hand, 0), cmp_card_val(&hand.hand, 1), cmp_card_val(&hand.hand, 2), cmp_card_val(&hand.hand, 3), cmp_card_val(&hand.hand, 4)]
    )
}

#[derive(Debug)]
struct UpgradedHand {
    hand_match: HandMatch,
    score: u32,
    hand: Hand,
}

fn hand_score_with_upgraded_jokers(hand: &CalculatedHandResult) -> UpgradedHand {
    let joker_count = hand.hand.cards.iter().filter(|card| card.is_joker()).count();
    if joker_count == 0 {
        return hand.hand_match.map_to(|x| UpgradedHand {
            hand_match: x,
            score: x.score(),
            hand: hand.hand,
        });
    }

    let most_common = find_most_common_card(&hand.hand);
    process_hand(&Hand {
        cards: hand.hand.cards.map(|c| if c.is_joker() { most_common.card } else { c }),
        bet: hand.hand.bet,
    }).map_to(|new_hand| UpgradedHand {
        hand_match: new_hand.hand_match,
        score: new_hand.hand_match.score(),
        hand: new_hand.hand,
    })
}

fn find_most_common_card(hand: &Hand) -> CondensedCard {
    let ignored = &['J'];
    if let Some(card) = find_card_by_count(hand, 5, ignored) {
        return card;
    }
    if let Some(card) = find_card_by_count(hand, 4, ignored) {
        return card;
    }
    if let Some(card) = find_card_by_count(hand, 3, ignored) {
        return card;
    }
    if let Some(card) = find_card_by_count(hand, 2, ignored) {
        return card;
    }
    CondensedCard { card: hand.cards[0], count: 1 }
}

#[derive(Copy, Clone, Debug)]
struct Card {
    label: char,
    value: u32,
}

impl Card {
    fn new(char: char) -> Self {
        /*
        A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
        13 12 11 10 9  8  7  6  5  4  3  2  1
         */
        Self {
            label: char,
            value: match char {
                'J' => 1,
                ch @ '2'..='9' => ch.to_digit(10).unwrap(),
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => unreachable!("Invalid card char: {}", char),
            },
        }
    }

    fn empty() -> Self {
        Self {
            label: '_',
            value: 0,
        }
    }

    fn is_joker(&self) -> bool {
        self.label == 'J'
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hand {
    cards: [Card; 5],
    bet: u32,
}

impl From<([char; 5], u32)> for Hand {
    fn from((cards, bet): ([char; 5], u32)) -> Self {
        Self {
            cards: cards.map(|x| Card::new(x)),
            bet,
        }
    }
}

impl<'a> From<(&'a str, u32)> for Hand {
    fn from((cards, bet): (&'a str, u32)) -> Self {
        Self {
            cards: cards.chars().map_to(|mut it| [
                it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap()
            ].map(|x| Card::new(x))),
            bet,
        }
    }
}

impl Display for CalculatedHandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hand {{Bet: {}, Cards: {}}}",
            self.hand.bet,
            join_to_string!(self.hand.cards.iter().map(|x| x.label), ""),
        )
    }
}

fn nom_parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, nom_parse_hand)(input)
}

fn nom_parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (hand_chars, _, bet)) = tuple((alphanumeric1, consume_empty_space, digit1))(input)?;
    let chars = hand_chars.chars().collect::<Vec<char>>();
    Ok((input, Hand {
        cards: [
            Card::new(chars[0]),
            Card::new(chars[1]),
            Card::new(chars[2]),
            Card::new(chars[3]),
            Card::new(chars[4])
        ],
        bet: bet.parse().unwrap(),
    }))
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HandMatch {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandMatch {
    fn score(&self) -> u32 {
        match self {
            HandMatch::FiveOfAKind => 7,
            HandMatch::FourOfAKind => 6,
            HandMatch::FullHouse => 5,
            HandMatch::ThreeOfAKind => 4,
            HandMatch::TwoPair => 3,
            HandMatch::OnePair => 2,
            HandMatch::HighCard => 1,
        }
    }
}

#[derive(Debug)]
struct CalculatedHandResult {
    hand: Hand,
    hand_match: HandMatch,
    labels: Vec<char>,
}


/*
Five of a kind, where all five cards have the same label:
    AAAAA
Four of a kind, where four cards have the same label and one card has a different label:
    AAAA8
Full house, where three cards have the same label, and the remaining two cards share a different label:
    33322
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand:
    TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label:
    22334
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other:
    234AA
High card, where all cards' labels are distinct:
    23456
*/

struct CondensedCard {
    card: Card,
    count: u32,
}

fn process_hand(hand: &Hand) -> CalculatedHandResult {
    // let counts = label_to_count.iter().map(|(&label, &count)| CondensedCard { card: Card::new(label), count }).collect_to_vec();
    // let find_by_count = |count: u32| counts.iter().filter(|c| c.count == count).next();
    // five of a kind
    if let Some(card) = find_card_by_count(&hand, 5, &[]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::FiveOfAKind,
            labels: vec![card.card.label],
        };
    }
    // four of a kind
    if let Some(card) = find_card_by_count(&hand, 4, &[]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::FourOfAKind,
            labels: vec![card.card.label],
        };
    }
    // full house
    if let Some(matches) = find_many_cards_by_counts(&hand, &[3, 2]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::FullHouse,
            labels: matches.iter().map(|m| m.card.label).collect(),
        };
    }
    // three of a kind
    if let Some(card) = find_card_by_count(&hand, 3, &[]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::ThreeOfAKind,
            labels: vec![card.card.label],
        };
    }
    // two pair
    if let Some(matches) = find_many_cards_by_counts(&hand, &[2, 2]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::TwoPair,
            labels: matches.iter().map(|m| m.card.label).collect(),
        };
    }
    // one pair
    if let Some(card) = find_card_by_count(&hand, 2, &[]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::OnePair,
            labels: vec![card.card.label],
        };
    }
    // high card
    if let Some(card) = find_many_cards_by_counts(&hand, &[1, 1, 1, 1, 1]) {
        return CalculatedHandResult {
            hand: *hand,
            hand_match: HandMatch::HighCard,
            labels: card.iter().map(|m| m.card.label).collect(),
        };
    }

    unreachable!("This should never be reached; if this is reached, please panic calmly and exit in a orderly fashion. However, if the issue is \
    that you are stuck in vim, don't bother asking for help. You are forever stuck in vim.")
}

fn find_card_by_count(hand: &Hand, expected_count: u32, excluded: &[char]) -> Option<CondensedCard> {
    for card in &hand.cards {
        if excluded.contains(&card.label) {
            continue;
        }
        let count = (&hand.cards).iter().filter(|c| c.label == card.label).count() as u32;
        if count == expected_count {
            return Some(CondensedCard { card: *card, count });
        }
    }
    None
}

fn find_many_cards_by_counts(hand: &Hand, expected_counts: &[u32]) -> Option<Vec<CondensedCard>> {
    let mut excluded = Vec::with_capacity(5);
    let mut out = Vec::with_capacity(expected_counts.len());
    for expected_count in expected_counts {
        match find_card_by_count(hand, *expected_count, &excluded) {
            None => return None,
            Some(val) => {
                excluded.push(val.card.label);
                out.push(val);
            }
        }
    }
    if out.len() == expected_counts.len() {
        return Some(out);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_upgrades_one_to_two() {
        let hand = Hand::from(("274J6", 100));
        let processed = process_hand(&hand);
        let upgraded = hand_score_with_upgraded_jokers(&processed);
        assert_eq!(upgraded.hand_match, HandMatch::OnePair);
    }

    #[test]
    fn test_hand_upgrades_two_to_three() {
        let hand = Hand::from(("224J6", 100));
        let processed = process_hand(&hand);
        let upgraded = hand_score_with_upgraded_jokers(&processed);
        assert_eq!(upgraded.hand_match, HandMatch::ThreeOfAKind);
    }

    #[test]
    fn test_hand_upgrades_three_to_four() {
        let hand = Hand::from(("QQQJ6", 100));
        let processed = process_hand(&hand);
        let upgraded = hand_score_with_upgraded_jokers(&processed);
        assert_eq!(upgraded.hand_match, HandMatch::FourOfAKind);
    }

    #[test]
    fn test_hand_upgrades_four_to_five() {
        let hand = Hand::from(("2222J", 100));
        let processed = process_hand(&hand);
        let upgraded = hand_score_with_upgraded_jokers(&processed);
        assert_eq!(upgraded.hand_match, HandMatch::FiveOfAKind);
    }

    #[test]
    fn test_hand_upgrades_no_upgrades() {
        let hand = Hand::from(("27496", 100));
        let processed = process_hand(&hand);
        let upgraded = hand_score_with_upgraded_jokers(&processed);
        assert_eq!(upgraded.hand_match, HandMatch::HighCard);
    }

    #[test]
    fn test_hand_upgrades_two_pair_to_full_house() {
        let hand = Hand::from(("22JQQ", 100));
        let processed = process_hand(&hand);
        let upgraded = hand_score_with_upgraded_jokers(&processed);
        assert_eq!(upgraded.hand_match, HandMatch::FullHouse);
    }
}