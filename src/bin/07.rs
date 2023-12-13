use std::cmp::Ordering;
use std::collections::BTreeMap;

use std::collections::BinaryHeap;

use counter::Counter;

use crate::HandType::*;

advent_of_code::solution!(7);

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn compute_hand_type(cards: &str) -> HandType {
    let cards_counter = cards.chars().collect::<Counter<_>>().most_common_ordered();

    let n_unique_cards = cards_counter.len();

    assert!(n_unique_cards <= 5);
    assert!(n_unique_cards >= 1);

    match n_unique_cards {
        5 => HighCard,
        4 => OnePair,
        3 => match cards_counter[0].1 {
            3 => ThreeOfAKind,
            2 => TwoPair,
            _ => unreachable!(),
        },
        2 => match cards_counter[0].1 {
            3 => FullHouse,
            4 => FourOfAKind,
            _ => unreachable!(),
        },
        1 => FiveOfAKind,
        0 | 6.. => unreachable!(),
    }
}

fn compute_hand_type_joker(cards: &str) -> HandType {
    let mut cards_counter = cards.chars().collect::<Counter<_>>();

    let n_jokers = cards_counter.remove(&'J');

    match n_jokers {
        Some(5) => cards_counter[&'J'] += 5,
        Some(n) => {
            let most_common_card = cards_counter.most_common_ordered().first().unwrap().0;
            cards_counter[&most_common_card] += n;
        }
        None => (),
    }

    let cards_counter = cards_counter.most_common_ordered();

    let n_unique_cards = cards_counter.len();

    assert!(n_unique_cards <= 5);
    assert!(n_unique_cards >= 1);

    match n_unique_cards {
        5 => HighCard,
        4 => OnePair,
        3 => match cards_counter[0].1 {
            3 => ThreeOfAKind,
            2 => TwoPair,
            _ => unreachable!(),
        },
        2 => match cards_counter[0].1 {
            3 => FullHouse,
            4 => FourOfAKind,
            _ => unreachable!(),
        },
        1 => FiveOfAKind,
        0 | 6.. => unreachable!(),
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    value: u64,
    card_order: &'a str,
}

static CARD_ORDER: &str = "AKQJT98765432";
static CARD_ORDER_JOKER: &str = "AKQT98765432J";

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (ca, cb) in self.cards.chars().zip(other.cards.chars()) {
            let ca_strength = self.card_order.find(ca).expect("did not find card");
            let cb_strength = self.card_order.find(cb).expect("did not find card");

            if ca_strength < cb_strength {
                return Ordering::Greater;
            } else if cb_strength < ca_strength {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand<'_> {}

pub fn part_one(input: &str) -> Option<u32> {
    let hands: Vec<(&str, u64)> = parse_hands(&input);

    let mut map_by_type: BTreeMap<HandType, BinaryHeap<Hand>> = BTreeMap::new();

    for (cards, value) in &hands {
        let hand_type = compute_hand_type(&cards);

        let entry = map_by_type.entry(hand_type).or_default();

        entry.push(Hand {
            cards: cards,
            value: *value,
            card_order: CARD_ORDER,
        });
    }

    let mut ranked_hands: Vec<Hand> = Vec::new();

    for hands in map_by_type.into_values() {
        ranked_hands.append(&mut hands.into_sorted_vec());
    }

    Some(
        ranked_hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u32 * hand.value as u32)
            .sum(),
    )
}

fn parse_hands(input: &str) -> Vec<(&str, u64)> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let tuple: Vec<&str> = line.split_ascii_whitespace().collect();

        hands.push((tuple[0], tuple[1].parse().expect("failed to parse value")));
    }

    hands
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands: Vec<(&str, u64)> = parse_hands(&input);

    let mut map_by_type: BTreeMap<HandType, BinaryHeap<Hand>> = BTreeMap::new();

    for (cards, value) in &hands {
        let hand_type = compute_hand_type_joker(&cards);

        let entry = map_by_type.entry(hand_type).or_default();

        entry.push(Hand {
            cards: cards,
            value: *value,
            card_order: CARD_ORDER_JOKER,
        });
    }

    let mut ranked_hands: Vec<Hand> = Vec::new();

    for hands in map_by_type.into_values() {
        ranked_hands.append(&mut hands.into_sorted_vec());
    }

    let mut result = 0;

    for (i, hand) in ranked_hands.into_iter().enumerate() {
        result += ((i as u32 + 1) * hand.value as u32) as u32;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
