use std::fs;

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_value(ht: &HandType) -> i32 {
    if *ht == HandType::FiveOfAKind {
        return 7;
    } else if *ht == HandType::FourOfAKind {
        return 6;
    } else if *ht == HandType::FullHouse {
        return 5;
    } else if *ht == HandType::ThreeOfAKind {
        return 4;
    } else if *ht == HandType::TwoPair {
        return 3;
    } else if *ht == HandType::OnePair {
        return 2;
    } else if *ht == HandType::HighCard {
        return 1;
    } else {
        return -1;
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        get_hand_value(self).cmp(&get_hand_value(other))
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(get_hand_value(self).cmp(&get_hand_value(other)))
    }
}

fn get_card_strength(card: char) -> i32 {
    if card == 'A' {
        return 14;
    } else if card == 'K' {
        return 13;
    } else if card == 'Q' {
        return 12;
    } else if card == 'J' {
        return 11;
    } else if card == 'T' {
        return 10;
    } else {
        return card as i32 - b'0' as i32;
    }
}

fn get_hand_strength(cards: &Vec<i32>) -> HandType {
    let mut max_count = 0;
    let mut max_count_card = -1;
    for i in 0..cards.len() {
        let mut count = 0;
        for j in 0..cards.len() {
            if cards[i] == cards[j] {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
            max_count_card = cards[i];
        }
    }
    if max_count == 5 {
        return HandType::FiveOfAKind;
    } else if max_count == 4 {
        return HandType::FourOfAKind;
    } else if max_count == 3 {
        let other_cards: Vec<&i32> = cards.iter().filter(|c| **c != max_count_card).collect();
        if other_cards[0] == other_cards[1] {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    } else if max_count == 2 {
        let other_cards: Vec<&i32> = cards.iter().filter(|c| **c != max_count_card).collect();

        if other_cards[0] == other_cards[1]
            || other_cards[0] == other_cards[2]
            || other_cards[1] == other_cards[2]
        {
            return HandType::TwoPair;
        } else {
            return HandType::OnePair;
        }
    } else {
        return HandType::HighCard;
    }
}

fn sort_hands(h1: (&str, i32), h2: (&str, i32)) -> Ordering {
    let h1_cards: Vec<i32> = h1.0.chars().map(|c| get_card_strength(c)).collect();
    let h2_cards: Vec<i32> = h2.0.chars().map(|c| get_card_strength(c)).collect();

    let h1_type = get_hand_strength(&h1_cards);
    let h2_type = get_hand_strength(&h2_cards);

    if h1_type == h2_type {
        for i in 0..h1_cards.len() {
            if h1_cards[i] > h2_cards[i] {
                return Ordering::Greater;
            } else if h1_cards[i] < h2_cards[i] {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    } else {
        return h1_type.cmp(&h2_type);
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    //     let input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";

    let lines: Vec<&str> = input.lines().collect();

    let mut hands = Vec::new();

    for line in lines {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line_chars = line.split(" ").collect::<Vec<&str>>()[0];
        let bid_amount: i32 = line.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

        hands.push((line_chars, bid_amount));
    }

    hands.sort_by(|a, b| sort_hands(*a, *b));

    let mut total_winnings = 0;
    for i in 0..hands.len() {
        total_winnings += (i as i32 + 1) * hands[i].1;
    }
    println!("{total_winnings}")
}
