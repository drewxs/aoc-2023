use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARDS_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn part_1(input: &str) -> usize {
    let hands = get_hands(&input);
    let mut scores: BTreeMap<u8, Vec<usize>> = BTreeMap::new();

    for (i, (hand, _)) in hands.iter().enumerate() {
        let mut cards: Vec<char> = hand.chars().collect();
        cards.sort_unstable();

        let pairs_map = pairs(&cards);
        let pairs = num_pairs(&pairs_map, 2);
        let mut score = 1;

        let kind = highest_n_of_a_kind(&cards);

        if kind == 5 {
            score = 7;
        } else if kind == 4 {
            score = 6;
        } else if full_house(&pairs_map) {
            score = 5;
        } else if kind == 3 {
            score = 4;
        } else if pairs == 2 {
            score = 3;
        } else if pairs == 1 {
            score = 2;
        }

        scores.entry(score).or_insert(vec![]).push(i);
    }

    total_winnings(&CARDS, &hands, &mut scores)
}

pub fn part_2(input: &str) -> usize {
    let hands = get_hands(&input);
    let mut scores: BTreeMap<u8, Vec<usize>> = BTreeMap::new();

    for (i, (hand, _)) in hands.iter().enumerate() {
        let mut cards: Vec<char> = hand.chars().collect();
        cards.sort_unstable();

        let jokers = cards
            .iter()
            .fold(0, |acc, &c| if c == 'J' { acc + 1 } else { acc });

        let pairs_map = pairs_j(&cards);
        let pairs = num_pairs(&pairs_map, 2);
        let mut score = 1;

        let kind = highest_n_of_a_kind_j(&cards);

        if jokers >= 4
            || (jokers == 3 && (pairs >= 1 || kind >= 2))
            || (jokers == 2 && kind >= 3)
            || (jokers == 1 && kind >= 4)
            || kind == 5
        {
            score = 7;
        } else if jokers == 3
            || (jokers == 2 && pairs >= 1)
            || (jokers == 1 && kind >= 3)
            || kind == 4
        {
            score = 6;
        } else if (jokers == 2 && pairs == 1)
            || (jokers == 1 && pairs == 2)
            || full_house(&pairs_map)
        {
            score = 5;
        } else if (jokers == 2) || (jokers == 1 && pairs >= 1) || kind == 3 {
            score = 4;
        } else if (jokers == 1 && pairs >= 1) || pairs == 2 {
            score = 3;
        } else if jokers == 1 || pairs == 1 {
            score = 2;
        }

        scores.entry(score).or_insert(vec![]).push(i);
    }

    total_winnings(&CARDS_2, &hands, &mut scores)
}

fn get_hands(input: &str) -> Vec<(&str, usize)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split.next().unwrap();
            let bid = split.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect()
}

fn total_winnings(
    order: &[char],
    hands: &[(&str, usize)],
    scores: &mut BTreeMap<u8, Vec<usize>>,
) -> usize {
    let mut curr_rank = 1;
    let mut total_winnings = 0;

    for (_, values) in scores.iter_mut() {
        if values.len() == 1 {
            let idx = values[0];
            let (_, bid) = hands[idx];

            total_winnings += bid * curr_rank;

            curr_rank += 1;
            continue;
        }

        values.sort_unstable_by(|&a, &b| {
            let (hand_a, _) = hands[a];
            let (hand_b, _) = hands[b];

            same_label_cmp(order, hand_b, hand_a)
        });

        for &idx in values.iter() {
            let (_, bid) = hands[idx];

            total_winnings += bid * curr_rank;

            curr_rank += 1;
        }
    }

    total_winnings
}

fn pair(cards: &[char]) -> bool {
    cards.iter().all(|&c| c == cards[0])
}

fn pair_j(cards: &[char]) -> bool {
    cards.iter().all(|&c| c == cards[0] && c != 'J')
}

fn pairs(cards: &[char]) -> HashMap<char, i32> {
    cards.iter().fold(HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn pairs_j(cards: &[char]) -> HashMap<char, i32> {
    cards.iter().fold(HashMap::new(), |mut acc, &c| {
        if c == 'J' {
            return acc;
        }
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn highest_n_of_a_kind(cards: &[char]) -> usize {
    (2..=5)
        .rev()
        .find(|&n| cards.windows(n).any(pair))
        .unwrap_or(0)
}

fn highest_n_of_a_kind_j(cards: &[char]) -> usize {
    (2..=5)
        .rev()
        .find(|&n| cards.windows(n).any(pair_j))
        .unwrap_or(0)
}

fn num_pairs(map: &HashMap<char, i32>, n: i32) -> usize {
    map.values().filter(|&&v| v == n).count()
}

fn full_house(map: &HashMap<char, i32>) -> bool {
    num_pairs(map, 2) == 1 && num_pairs(map, 3) == 1
}

fn same_label_cmp(order: &[char], cards_a: &str, cards_b: &str) -> Ordering {
    for (a, b) in cards_a.chars().zip(cards_b.chars()) {
        let a_idx = order.iter().position(|&c| c == a).unwrap();
        let b_idx = order.iter().position(|&c| c == b).unwrap();

        if a_idx != b_idx {
            return a_idx.cmp(&b_idx);
        }
    }

    Ordering::Equal
}
