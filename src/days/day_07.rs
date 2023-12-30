use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

pub fn part_1(input: &str) -> usize {
    let n = input.lines().clone().count();

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

    for i in 0..n {
        let (hand, _) = hands[i];
        let cards: Vec<char> = hand.chars().collect();

        let mut score = 1;

        if n_of_a_kind(&cards, 5) {
            score = 7;
        } else if n_of_a_kind(&cards, 4) {
            score = 6;
        } else if full_house(&cards) {
            score = 5;
        } else if n_of_a_kind(&cards, 3) {
            score = 4;
        } else if n_pair(&cards, 2) {
            score = 3;
        } else if n_pair(&cards, 1) {
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

fn n_of_a_kind(cards: &[char], n: usize) -> bool {
    let mut cards = cards.to_vec();

    cards.sort();
    cards.windows(n).any(pair)
}

fn n_pair(cards: &[char], n: usize) -> bool {
    let mut map = HashMap::new();

    cards.iter().for_each(|&c| {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    });

    map.values().filter(|&&v| v == 2).count() == n
}

fn full_house(cards: &[char]) -> bool {
    let mut map = HashMap::new();

    cards.iter().for_each(|&c| {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    });

    map.values().filter(|&&v| v == 2).count() == 1 && map.values().filter(|&&v| v == 3).count() == 1
}

fn same_label_cmp(cards_a: &str, cards_b: &str) -> Ordering {
    let mut a_cmp = 0;
    let mut b_cmp = 0;

    for (a, b) in cards_a.chars().zip(cards_b.chars()) {
        let a_val = CARDS.iter().position(|&c| c == a).unwrap();
        let b_val = CARDS.iter().position(|&c| c == b).unwrap();

        if a_val > b_val {
            a_cmp = 1;
            break;
        } else if a_val < b_val {
            b_cmp = 1;
            break;
        }
    }

    a_cmp.cmp(&b_cmp)
}

pub fn part_2(_input: &str) -> usize {
    0
}
