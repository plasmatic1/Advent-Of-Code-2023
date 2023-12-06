/**
 * On: Intel i5-9600K @ 3.7GHz (6 cores)
 * * t=1 threads: 103.2046917s
 * * t=6 threads: 25.6020192s
 */

use std::{ops::Range, collections::VecDeque, sync::{mpsc, Mutex, Arc}, thread};

use aoc::aoc;
use itertools::Itertools;

struct Rule {
    in_range: Range<i64>,
    shift: i64
}

impl Clone for Rule {
    fn clone(&self) -> Self {
        Rule {
            in_range: self.in_range.clone(),
            shift: self.shift
        }
    }
}

trait Apply {
    fn apply(&self, x: Option<i64>) -> Option<i64>;
}

/// For Rule: applies the rule, returning none if out of range or if input is None
impl Apply for Rule {
    fn apply(&self, x: Option<i64>) -> Option<i64> {
        if self.in_range.contains(&x?) {
            Some(x? + self.shift)
        }
        else {
            None
        }
    }
}

/// For vec: applies the first rule possible
/// 
/// If it does not work, return original value
impl Apply for Vec<Rule> {
    fn apply(&self, x: Option<i64>) -> Option<i64> {
        for rule in self {
            match rule.apply(x) {
                y@Some(_) => {
                    return y
                }
                _ => {}
            };
        }
        x
    }
}

fn main() {
    let input_lines = aoc::get_input_lines();
    let (seeds_block, map_blocks) = {
        let mut blocks = input_lines.split(|line| line.is_empty());
        (blocks.next().unwrap().first().unwrap(), blocks.collect_vec())
    };

    // list of all seed ranges
    let mut n_seeds = 0;
    let seed_ranges_ref: VecDeque<Range<i64>> = {
        if aoc::get_args().task == 1 {
            seeds_block.split_once(": ").unwrap().1
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .map(|x| x..(x+1))
                .map(|x| { n_seeds += 1; x })
                .collect()
        }
        else {
            seeds_block.split_once(": ").unwrap().1
                .split(' ').collect_vec().chunks(2).map(|_v| {
                    let (x, n) = _v.iter().map(|x| x.parse::<i64>().unwrap()).next_tuple().unwrap();
                    n_seeds += n;
                    x..(x+n)
                })
                .collect()
        }
    };

    // vector of all mapping tables
    // each mapping table is a Vec<Rule>
    let range_blocks = map_blocks.into_iter().map(|lines| {
        lines[1..].iter().map(|line| {
            let mut nums = line.split(' ').map(|x| x.parse::<i64>().unwrap());
            let (x, y, d) = nums.next_tuple().unwrap();
            Rule { in_range: y..(y+d), shift: x - y }
        }).collect_vec()
    }).collect_vec();

    // calculate ans -- multithreaded!!!
    const N_THREAD: i64 = 6;
    const BLOCK_SZ: usize = 10000000;

    let (tx, rx) = mpsc::channel::<i64>();
    let seed_ranges = Arc::new(Mutex::new(seed_ranges_ref));

    for i in 0..N_THREAD {
        // set up variables... mostly just cloning many things
        let tx1 = tx.clone();
        let sr1 = seed_ranges.clone(); // ARC to same seed ranges
        let rb1 = range_blocks.clone();
        let mut buf1: Vec<i64> = vec![0; BLOCK_SZ];

        thread::spawn(move || {
            loop {
                // get next block
                let mut n = 0;
                {
                    let mut sr = sr1.lock().unwrap();

                    // get next block
                    'get_block: while n < BLOCK_SZ {
                        // println!("sr={:?} n={}", sr, n);
                        match sr.front_mut() {
                            Some(range) => {
                                match (range).next() {
                                    Some(x) => {
                                        buf1[n] = x;
                                        n += 1;
                                    },
                                    None => { sr.pop_front(); }
                                }
                            },
                            None => { break 'get_block; }
                        };
                    }
                }

                // println!("got new buf");

                // exit if no more blocks
                if n == 0 {
                    break;
                }

                // println!("start proc");

                // process block
                for x in buf1[..n].iter_mut() {
                    let result = rb1.iter()
                        .fold(Some(*x), |y, block| block.apply(y))
                        .unwrap();
                    *x = result;
                }

                // println!("processed");
            
                // send result back to main thread
                let result = buf1[..n].iter().min().unwrap();
                tx1.send(*result).unwrap();

                // println!("sent");
            }
        });
        println!("spawned thread {}", i);
    }
    drop(tx);

    // ans is min from all threads
    // rx closes once all tx are dropped
    let mut n_blocks_recv: i64 = 0;
    let ans = rx.iter().map(|x| {
        n_blocks_recv += 1;
        println!("recv {} / approx. {} blocks", n_blocks_recv, n_seeds / (BLOCK_SZ as i64));
        x
    }).min().unwrap(); // reads until all clones of the channel are done
    println!("{}", ans);
}