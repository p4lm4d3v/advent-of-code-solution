use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CardType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
    Default,
}

impl CardType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Clone, Eq, PartialEq, Copy)]
struct Card {
    char: char,
}

impl Card {
    fn new(char: char) -> Self {
        Self { char: char }
    }

    fn get_strength(&self) -> usize {
        let map: HashMap<char, usize> = HashMap::from([
            ('A', 13),
            ('K', 12),
            ('Q', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('J', 1),
        ]);

        *map.get(&self.char).unwrap()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Hand {
    old_cards: Vec<Card>,
    new_cards: Vec<Card>,
    bid: usize,
    card_type: CardType,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            old_cards: vec![],
            new_cards: vec![],
            bid: 0,
            card_type: CardType::Default,
        }
    }
}

fn card_type_to_int(card_type: &CardType) -> usize {
    match card_type {
        CardType::FiveKind => 7,
        CardType::FourKind => 6,
        CardType::FullHouse => 5,
        CardType::ThreeKind => 4,
        CardType::TwoPair => 3,
        CardType::OnePair => 2,
        CardType::HighCard => 1,
        _ => 0,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = card_type_to_int(&self.card_type);
        let other_rank = card_type_to_int(&other.card_type);

        println!(
            "{}: {}",
            self.new_cards
                .iter()
                .map(|f| f.char.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self_rank
        );
        println!(
            "{}: {}",
            other
                .new_cards
                .iter()
                .map(|f| f.char.to_string())
                .collect::<Vec<String>>()
                .join(""),
            other_rank
        );
        println!();

        match self_rank.cmp(&other_rank) {
            Ordering::Equal => {
                // If ranks are equal, compare cards one by one based on their strength
                for i in 0..5 {
                    let self_strength = self.old_cards[i].get_strength();
                    let other_strength = other.old_cards[i].get_strength();

                    match self_strength.cmp(&other_strength) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                // All cards are equal, so the hands are equal
                Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(old_cards: Vec<Card>, bid: usize) -> Self {
        let mut map: HashMap<char, usize> = HashMap::new();

        for card in old_cards.iter() {
            if !map.contains_key(&card.char) {
                map.insert(card.char, 1);
            } else {
                *map.get_mut(&card.char).unwrap() += 1;
            }
        }

        let max = map.iter().max_by(|v1, v2| v1.1.cmp(v2.1)).unwrap();

        let new_cards = old_cards
            .iter()
            .map(|card| {
                if card.char == 'J' {
                    Card::new(*max.0)
                } else {
                    *card
                }
            })
            .collect::<Vec<Card>>();

        // println!(
        //     "New Card: {}",
        //     new_cards
        //         .iter()
        //         .map(|f| f.char.to_string())
        //         .collect::<Vec<String>>()
        //         .join("")
        // );

        for card in new_cards.iter() {
            if !map.contains_key(&card.char) {
                map.insert(card.char, 1);
            } else {
                *map.get_mut(&card.char).unwrap() += 1;
            }
        }

        Self {
            old_cards: old_cards,
            new_cards: new_cards,
            bid: bid,
            card_type: Hand::get_card_type(map),
        }
    }

    fn get_card_type(map: HashMap<char, usize>) -> CardType {
        let card_type = if Hand::is_five_of_a_kind(&map) {
            CardType::FiveKind
        } else if Hand::is_four_of_a_kind(&map) {
            CardType::FourKind
        } else if Hand::is_full_house(&map) {
            CardType::FullHouse
        } else if Hand::is_three_of_a_kind(&map) {
            CardType::ThreeKind
        } else if Hand::is_two_pair(&map) {
            CardType::TwoPair
        } else if Hand::is_one_pair(&map) {
            CardType::OnePair
        } else if Hand::is_high_card(&map) {
            CardType::HighCard
        } else {
            CardType::HighCard
        };
        card_type
    }

    fn is_five_of_a_kind(map: &HashMap<char, usize>) -> bool {
        let values = map.values().collect::<Vec<&usize>>();

        values.iter().any(|v| **v == 5)
    }

    fn is_four_of_a_kind(map: &HashMap<char, usize>) -> bool {
        let values = map.values().collect::<Vec<&usize>>();

        values.iter().any(|v| **v == 4)
    }

    fn is_full_house(map: &HashMap<char, usize>) -> bool {
        let values = map.values().collect::<Vec<&usize>>();

        values.iter().any(|v| **v == 3) && values.iter().any(|v| **v == 2)
    }

    fn is_three_of_a_kind(map: &HashMap<char, usize>) -> bool {
        let values = map.values().collect::<Vec<&usize>>();

        values.iter().any(|v| **v == 3)
    }

    fn is_two_pair(map: &HashMap<char, usize>) -> bool {
        let kvps = map.iter().collect::<Vec<(&char, &usize)>>();
        let filtered = kvps
            .iter()
            .filter(|kvp| *kvp.1 == 2)
            .collect::<Vec<&(&char, &usize)>>();

        filtered.len() == 2
    }

    fn is_one_pair(map: &HashMap<char, usize>) -> bool {
        let values = map.values().collect::<Vec<&usize>>();

        values.iter().any(|v| **v == 2)
    }

    fn is_high_card(map: &HashMap<char, usize>) -> bool {
        map.values()
            .collect::<Vec<&usize>>()
            .iter()
            .all(|v| **v == 1)
    }
    fn to_string(&self) -> String {
        let hand = self
            .new_cards
            .iter()
            .map(|f| f.char.to_string())
            .collect::<Vec<String>>()
            .join("");
        format!(
            "Hand: {};\t Bid: {};\t Rank: {}",
            hand,
            self.bid,
            self.card_type.to_string()
        )
    }
}

fn process(input: &str) -> usize {
    let mut hands: Vec<Hand> = Vec::new();
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();

        let cards = split
            .first()
            .unwrap()
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| Card::new(s.chars().next().unwrap()))
            .collect::<Vec<Card>>();
        let bid = split.last().unwrap().parse::<usize>().unwrap();

        let hand = Hand::new(cards.clone(), bid);
        hands.push(hand);
    }

    let mut sorted_hands = (0..hands.len())
        .map(|_| Hand::default())
        .collect::<Vec<Hand>>();
    sorted_hands.clone_from_slice(&hands[0..]);

    sorted_hands.sort_by(|h1, h2| h1.cmp(h2));

    let mut res: Vec<usize> = Vec::new();
    for (i, hand) in sorted_hands.iter().enumerate() {
        res.push((i + 1) * hand.bid);
        println!("#{} {}", i + 1, hand.to_string())
    }

    let mut sum: usize = 0;
    res.iter().for_each(|f| sum += f);
    sum
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        );
        assert_eq!(result, 0);
    }
}
