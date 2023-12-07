use nom::{
    branch::alt,
    character::complete::{self, line_ending, space1},
    combinator::value,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering, collections::BTreeMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveAfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter)]
pub enum Card {
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

fn is_full_house(card_map: BTreeMap<Card, u32>) -> bool {
    card_map.len() == 2 && card_map.values().cloned().any(|val| val == 3)
}

pub fn get_highest_hand(hands: &Vec<Hand>) -> Hand {
    assert!(!hands.is_empty(), "empty hand vector");
    *hands
        .iter()
        .max()
        .expect("there should be at least one hand")
}

fn hand_type(cards: [Card; 5]) -> HandType {
    let mut s: BTreeMap<Card, u32> = BTreeMap::new();
    for card in cards {
        s.entry(card).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut value_map: BTreeMap<u32, u32> = BTreeMap::new();
    s.values().for_each(|val| {
        value_map.entry(*val).and_modify(|e| *e += 1).or_insert(1);
    });
    match s.values().max().expect("at least a high card") {
        1 => HandType::HighCard,
        2 => {
            // since we are looking at the most found cards this cannot be a full house
            if value_map
                .get(&2)
                .expect("there should be at least one pair")
                == &2
            {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        3 => {
            if is_full_house(s) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        4 => HandType::FourOfAKind,
        5 => HandType::FiveAfAKind,
        _ => panic!("Malformed input"),
    }
}

pub fn permutate_hand(hand: &Hand) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    hands.push(*hand);
    if hand.cards.contains(&Card::CJ) {
        for idx in 0..5 {
            if hand.cards[idx] == Card::CJ {
                for card in Card::iter().filter(|x| x != &Card::CJ) {
                    let mut new_cards = hand.cards;
                    new_cards[idx] = card;
                    let new_hand_type = hand_type(new_cards);
                    // if new_hand_type >= hand.hand_type {
                    let new_hand = Hand {
                        hand_type: new_hand_type,
                        cards: new_cards,
                    };
                    let new_permutations = permutate_hand(&new_hand);
                    let new_hand = get_highest_hand(&new_permutations);
                    hands.push(new_hand);
                    // }
                }
            }
        }
    }
    hands
}

#[derive(Eq, Debug, Clone, Copy)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: [Card; 5],
}

#[derive(Debug)]
pub struct Game {
    pub hand: Hand,
    pub bet: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                let mut ordering: Option<Ordering> = None;
                for (own_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match own_card.cmp(other_card) {
                        Ordering::Equal => continue,
                        other => {
                            ordering = Some(other);
                            break;
                        }
                    };
                }
                match ordering {
                    Some(ordering) => ordering,
                    None => Ordering::Equal,
                }
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal && (self.hand_type == other.hand_type)
    }
}

pub fn card_parser(input: &str) -> IResult<&str, Card> {
    alt((
        value(Card::C2, complete::char('2')),
        value(Card::C3, complete::char('3')),
        value(Card::C4, complete::char('4')),
        value(Card::C5, complete::char('5')),
        value(Card::C6, complete::char('6')),
        value(Card::C7, complete::char('7')),
        value(Card::C8, complete::char('8')),
        value(Card::C9, complete::char('9')),
        value(Card::CT, complete::char('T')),
        value(Card::CJ, complete::char('J')),
        value(Card::CQ, complete::char('Q')),
        value(Card::CK, complete::char('K')),
        value(Card::CA, complete::char('A')),
    ))(input)
}

pub fn hand_parser(input: &str) -> IResult<&str, Hand> {
    let (input, hand) = count(card_parser, 5)(input)?;
    let cards = hand
        .try_into()
        .expect("Parsed Data should contain exactly 5 cards");
    Ok((
        input,
        Hand {
            hand_type: hand_type(cards),
            cards,
        },
    ))
}

pub fn line_parser(input: &str) -> IResult<&str, Game> {
    let (input, (hand, bet)) = separated_pair(hand_parser, space1, complete::u32)(input)?;
    Ok((input, Game { hand, bet }))
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, mut games): (_, Vec<Game>) = separated_list1(line_ending, line_parser)(input).unwrap();
    // println!("{:?}", games);
    games.iter_mut().for_each(|game| {
        game.hand.hand_type = get_highest_hand(&permutate_hand(&game.hand)).hand_type;
    });
    games.sort_by(|a, b| a.hand.cmp(&b.hand));
    let result: u32 = games
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, game)| acc + (idx + 1) as u32 * game.bet);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering_hand_type() {
        assert!(HandType::FiveAfAKind > HandType::TwoPair);
        assert!(HandType::FiveAfAKind > HandType::ThreeOfAKind);
        assert!(HandType::FiveAfAKind > HandType::FullHouse);
        assert!(HandType::FiveAfAKind > HandType::FourOfAKind);
        assert!(HandType::FiveAfAKind == HandType::FiveAfAKind);
    }

    #[test]
    fn test_ordering_card() {
        assert!(
            Hand {
                cards: [Card::C3, Card::C3, Card::C3, Card::C3, Card::C2],
                hand_type: HandType::FourOfAKind
            } > Hand {
                cards: [Card::C2, Card::CA, Card::CA, Card::CA, Card::CA],
                hand_type: HandType::FourOfAKind
            }
        )
    }

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
