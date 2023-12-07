#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use aoc::aoc;
use itertools::Itertools;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum HandKind {
    High,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

/// Handrank is a tuple with (HandKind, usize), where usize is an identifier for the hand based on first, second, etc. character
type HandRank = (HandKind, usize);

const CARDS: &'static str = "X23456789TJQKA";

lazy_static! {
    static ref DIG_RANK: HashMap::<char, usize> = {
        let v = CARDS.char_indices().collect_vec();
        v.into_iter().map(|(i, c)| (c, i)).collect()
    };

    static ref TASK: i32 = aoc::get_args().task;
}

fn hand_kind(hand: &str) -> HandKind {
    // Freq sorted in reversed order
    let fres = hand.chars()
        .map(|c| (c, c))
        .into_group_map()
        .into_iter()
        .map(|(_, vals)| vals.len())
        .sorted().rev()
        .collect_vec();

    match fres[..] {
        [5] => HandKind::FiveKind,
        [4, 1] => HandKind::FourKind,
        [3, 2] => HandKind::FullHouse,
        [3, 1, 1] => HandKind::ThreeKind,
        [2, 2, 1] => HandKind::TwoPair,
        [2, 1, 1, 1] => HandKind::Pair,
        [1, 1, 1, 1, 1] => HandKind::High,
        _ => panic!("invalid freq array {:?}", fres)
    }
}

fn hand_sid(hand: &str) -> usize {
    hand.chars().fold(0, |acc, c| acc * DIG_RANK.len() + DIG_RANK[&c])
}

fn hand_rank(hand: &str) -> HandRank {
    let joke_ind= hand.chars().enumerate().filter(|(_, c)| *c == 'X').map(|(i, _)| i).collect_vec();
    let joke_cnt = joke_ind.len();
    let candidate = {
        let mut v = hand.chars().unique().collect_vec();
        let mut added = 0;
        for c in CARDS.chars().rev() {
            if !v.contains(&c) {
                v.push(c);
                added += 1;
            }
            if added == 2 {
                break
            }
        }
        v
    };

    fn f(s: Vec<char>, n: usize, candidate: &Vec<char>, joke_ind: &Vec<usize>) -> HandKind {
        if n == 0 {
            hand_kind(String::from_iter(s.into_iter()).as_str())
        }
        else {
            candidate.iter()
                .map(|c| {
                    let mut s1 = s.clone();
                    s1[joke_ind[n - 1]] = *c;
                    f(s1, n - 1, candidate, joke_ind)
                })
                .max().unwrap()
        }
    }
    let kind = f(hand.chars().collect_vec(), joke_cnt, &candidate, &joke_ind);
    (kind, hand_sid(hand)) // IMPORTANT!: determine hand sid from original hand (including jokers)
}

/// maps "J" => "X" if TASK == 2, to distinguish jacks and jokers
fn fix_hand(hand: &str) -> String {
    if *TASK == 2 {
        hand.replace("J", "X")
    }
    else {
        hand.into()
    }
}

fn main() {
    let cards = aoc::get_input_tokens().chunks(2)
        .map(|toks| {
            match toks {
                [tok1, tok2] => {
                    let value = tok2.parse::<usize>().unwrap();
                    (hand_rank(fix_hand(tok1.as_str()).as_str()), value)
                }
                _ => panic!("invalid card {:?}", toks)
            }
        })
        .collect_vec();

    let ans: usize = cards.iter().sorted().enumerate().map(|(i, (_, v))| (i + 1) * v).sum();
    println!("{}", ans);
}