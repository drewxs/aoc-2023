use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

pub fn part_1(input: &str) -> usize {
    let hands: Vec<(&str, usize)> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split.next().unwrap();
            let bid = split.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect();

    let mut scores: BTreeMap<u8, Vec<usize>> = BTreeMap::new();

    for (i, (hand, _)) in hands.iter().enumerate() {
        let mut cards: Vec<char> = hand.chars().collect();
        cards.sort_unstable();

        let pairs = get_pairs(&cards);
        let num_pairs = num_pairs(&pairs, 2);
        let mut score = 1;

        if n_of_a_kind(&cards, 5) {
            score = 7;
        } else if n_of_a_kind(&cards, 4) {
            score = 6;
        } else if full_house(&pairs) {
            score = 5;
        } else if n_of_a_kind(&cards, 3) {
            score = 4;
        } else if num_pairs == 2 {
            score = 3;
        } else if num_pairs == 1 {
            score = 2;
        }

        scores.entry(score).or_insert(vec![]).push(i);
    }

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

            same_label_cmp(hand_b, hand_a)
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

fn get_pairs(cards: &[char]) -> HashMap<char, i32> {
    cards.iter().fold(HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn n_of_a_kind(cards: &[char], n: usize) -> bool {
    cards.windows(n).any(pair)
}

fn num_pairs(map: &HashMap<char, i32>, n: i32) -> usize {
    map.values().filter(|&&v| v == n).count()
}

fn full_house(map: &HashMap<char, i32>) -> bool {
    num_pairs(map, 2) == 1 && num_pairs(map, 3) == 1
}

fn same_label_cmp(cards_a: &str, cards_b: &str) -> Ordering {
    for (a, b) in cards_a.chars().zip(cards_b.chars()) {
        let a_val = CARDS.iter().position(|&c| c == a).unwrap();
        let b_val = CARDS.iter().position(|&c| c == b).unwrap();

        if a_val != b_val {
            return a_val.cmp(&b_val);
        }
    }

    Ordering::Equal
}

pub fn part_2(_input: &str) -> usize {
    0
}
