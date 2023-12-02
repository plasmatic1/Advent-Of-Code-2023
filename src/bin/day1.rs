use std::cmp::{max, min};

use aoc::aoc;

fn main() {
    let mut dmap = Vec::<(String, i32)>::new();
    for i in 1..10 {
        dmap.push((i.to_string(), i));
    }
    if aoc::get_args().task == 2 {
        for (i, w) in vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
            dmap.push((w.to_string(), (i + 1) as i32));
        }
    }

    let ans: i32 = aoc::get_input_lines().iter().map(|line| {
        let d1 = dmap.iter().map(|(w, d)| (line.find(w.as_str()).and_then(|x| Some(x as i32)).unwrap_or(i32::MAX), *d))
            .fold((i32::MAX, 0), min::<(i32, i32)>).1;
        let d2 = dmap.iter().map(|(w, d)| (line.rfind(w.as_str()).and_then(|x| Some(x as i32)).unwrap_or(i32::MIN), *d))
            .fold((i32::MIN, 0), max::<(i32, i32)>).1;

        d1 * 10 + d2
    }).sum();

    println!("{}", ans);
}