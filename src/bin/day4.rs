use std::cmp::min;

use aoc::aoc;
use itertools::Itertools;

fn parse_nums(input: &str) -> Vec<i32> {
    input.split(' ').filter(|x| !x.is_empty()).map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let input_lines = aoc::get_input_lines();
    let counts = input_lines.iter().map(|line| {
        let (cards_str_1, cards_str_2) = line.split_once(":").unwrap().1.split_once(" | ").unwrap();
        // println!("{:?} {:?} {:?}", line, cards_str_1, cards_str_2);
        let cards_1 = parse_nums(cards_str_1);
        let cards_2 = parse_nums(cards_str_2);

        cards_2.iter().filter(|c| cards_1.contains(c)).count() as i32
    }).collect_vec();

    if aoc::get_args().task == 1 {
        let ans: i32 = counts.into_iter().map(|x| if x == 0 { 0 } else { 2i32.pow((x - 1) as u32) }).sum();
        println!("{}", ans);
    }
    else {
        let n = counts.len();
        let mut card_cnt = vec![1; n];
        for i in 0..n {
            for j in (i + 1)..=min(i + counts[i] as usize, n - 1) {
                card_cnt[j] += card_cnt[i];
            }
        }
        // println!("{:?}", card_cnt);

        let ans: i32 = card_cnt.into_iter().sum();
        println!("{}", ans);
    }
}