/**
 * total time = T
 * record = D
 * 
 * if wait w secs, then travel for T-w secs at speed w
 * d(w) = w * (T - w)
 * 
 *    d(w) >= D 
 * -> w * (T - w) > D
 * -> wT - w^2 > D
 * -> -w^2 + wT - D > 0
 */

use aoc::aoc;
use itertools::Itertools;

fn parse_line(line: &str) -> Vec<i64> {
    line.split_once(": ").unwrap().1.trim()
        .split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i64>().unwrap())
        .collect_vec()
}

fn quadratic(a: i64, b: i64, c: i64) -> Option<(f64, f64)> {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    let d = b*b - 4.0*a*c;
    let d = d.sqrt();

    match d {
        d if d < 0.0 => return None,
        _ => {
            let x1 = (-b + d) / (2.0*a);
            let x2 = (-b - d) / (2.0*a);
            Some((x1, x2))
        }
    }
}

const EPS: f64 = 1e-5;

fn num_int_range(time: i64, dist: i64) -> i64 {
    match quadratic(-1, time, -dist) {
        Some((r1, r2)) => {
            let a = if r1 <= r2 { r1 } else { r2 };
            let b = if r1 >= r2 { r1 } else { r2 };
            // println!("{} {} {}", a, b, (b - EPS).floor() - (a + EPS).ceil());
            (((b - EPS).floor() - (a + EPS).ceil()) as i64) + 1
        },
        None => 0
    }
}

fn main() {
    let time_dist = {
        let lines = aoc::get_input_lines();

        if aoc::get_args().task == 1 {
            let time = parse_line(lines[0].as_str());
            let dist = parse_line(lines[1].as_str());
            time.into_iter().zip(dist.into_iter()).collect_vec()
        }
        else {
            let time = String::from_iter(lines[0].chars().filter(|c| char::is_numeric(*c))).parse::<i64>().unwrap();
            let dist = String::from_iter(lines[1].chars().filter(|c| char::is_numeric(*c))).parse::<i64>().unwrap();
            vec![(time, dist)]
        }
    };

    let ans = time_dist.into_iter()
        .map(|(t, d)| { let r = num_int_range(t, d); r } )
        .product::<i64>();
    println!("{}", ans);
}