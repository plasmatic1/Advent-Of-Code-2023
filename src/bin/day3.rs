use std::collections::HashMap;

use aoc::aoc;
use itertools::Itertools;

// given a list of positions, return a list of adjacent positions that are symbols
fn main() {
    let grid: Vec<_> = aoc::get_input_lines().iter().map(|line| line.chars().collect::<Vec<_>>()).collect();
    let n = grid.len();
    let m = grid[0].len();
    let task_no = aoc::get_args().task;

    // whether a location is a symbol
    let is_sym_fun =
        if task_no == 1 { |c: &char| !c.is_digit(10) && *c != '.' }
        else { |c: &char| *c == '*' };

    // get list of adj symbols
    let adj_syms = |pos: &[(usize, usize)]| {
        let mut adj_pos = vec![];
        for (i, j) in pos {
            for k in 0..=2 {
                for l in 0..=2 {
                    if i + k >= 1 && j + l >= 1 {
                        adj_pos.push((i + k - 1, j + l - 1));
                    }
                }
            }
        }
        adj_pos.sort();
        adj_pos.dedup();

        adj_pos.into_iter().filter(|(i, j)| {
            grid.get(*i)
                .and_then(|r| r.get(*j))
                .and_then(|c| Some(is_sym_fun(c)))
                .unwrap_or(false)
        }).collect_vec()
    };

    let mut sym_map = HashMap::<(usize, usize), Vec<i32>>::new();
    let mut ans = 0;

    // get all #s
    for i in 0..n {
        let mut j: usize = 0;
        while j < m {
            if !grid[i][j].is_digit(10) {
                j += 1;
                continue;
            }

            let k = match grid[i].iter().skip(j).position(|c| !c.is_digit(10)) {
                Some(d) => j + d,
                None => m
            };

            let cur_adj = adj_syms(&(j..k).map(|y| (i, y)).collect_vec()[..]);
            // println!("{} {} {:?}", i, j, cur_adj);
            let cur_val = i32::from_str_radix(grid[i][j..k].iter().collect::<String>().as_str(), 10).unwrap();
            if task_no == 1 {
                if !cur_adj.is_empty() {
                    ans += cur_val;
                }
            }
            else {
                for p in cur_adj {
                    sym_map.entry(p).or_insert(vec![]).push(cur_val);
                }
            }

            j = k;
        }
    }

    // print ans
    if task_no == 1 {
        println!("{}", ans);
    }
    else {
        // println!("{:?}", sym_map);
        for vs in sym_map.values() {
            if vs.len() == 2 {
                ans += vs[0] * vs[1];
            }
        }
        println!("{}", ans);
    }
}