use std::collections::HashMap;
use aoc::aoc;

fn main() {
    if aoc::get_args().task == 1 {
        let lim = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);

        let ans: usize = aoc::get_input_lines().iter().enumerate().map(|(i, line)| {
            let mut counts = (&line[line.find(':').unwrap() + 1..]).split(|c| c == ';' || c == ',').map(str::trim);
            let ok = counts.all(|tok| {
                match tok.split(' ').collect::<Vec<_>>()[..] {
                    [num_str, clr] => i32::from_str_radix(num_str, 10).unwrap() <= lim[clr],
                    _ => false
                }
            });
            if ok { i + 1 } else { 0 }
        }).sum();
        
        println!("{}", ans);
    } else {
        let ans: i32 = aoc::get_input_lines().iter().map(|line| {
            let counts = (&line[line.find(':').unwrap() + 1..]).split(|c| c == ';' || c == ',').map(str::trim);
            let counts_tup = counts.map(|tok| {
                match tok.split(' ').collect::<Vec<_>>()[..] {
                    [num_str, clr] => (clr, i32::from_str_radix(num_str, 10).unwrap()),
                    _ => panic!("Invalid token {}", tok)
                }
            });

            let mut map = HashMap::<&str, i32>::new();
            for (clr, num) in counts_tup {
                map.entry(clr).and_modify(|cnum| { if num > *cnum { *cnum = num; } }).or_insert(num);
            }

            map.values().product::<i32>()
        }).sum();
        
        println!("{}", ans);
    }
}