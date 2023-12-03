extern crate structopt;

use std::process::exit;
use std::process::Command;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "AoC solution runner.")]
struct Opt {
    #[structopt(name = "DAY", help = "Day number.  Between 1 and 25 (Dec 1 to 25)")]
    day: i32,

    #[structopt(name = "TASK", help = "Task number.  Either 1 or 2")]
    task: i32,

    #[structopt(short, long, help = "Only run with sample input")]
    sample_only: bool,
}

fn main() {
    let opt = Opt::from_args();
    if !(1 <= opt.day && opt.day <= 25) {
        println!("Invalid day {}!", opt.day);
        exit(1);
    }
    if !(1 <= opt.task && opt.task <= 2) {
        println!("Invalid task number {}! (must be 1 or 2)", opt.task);
        exit(1);
    }
    println!("Problem: day {} task {}", opt.day, opt.task);

    // run proc
    let in_sample_path = format!("inputs/day{}.s{}.txt", opt.day, opt.task);
    let in_path = format!("inputs/day{}.txt", opt.day);
    println!("Sample input path: {}", &in_sample_path);
    println!("Input path: {}", &in_path);

    // Get soln path
    let soln_ext = match std::env::consts::OS {
        "windows" => ".exe",
        _ => ""
    };
    let soln_path = format!("target/debug/day{}{}", opt.day, soln_ext);
    println!("Solution path: {}", soln_path);

    println!();

    let run_with_input_file = |path: &str| {
        match Command::new(&soln_path).arg(&opt.task.to_string()).arg(path).output() {
            Ok(o) => {
                if !o.stderr.is_empty() {
                    print!("ERR: Program returned error: ");
                    match String::from_utf8(o.stderr) {
                        Ok(s) => println!("{}", s),
                        Err(_) => println!("<could not convert to utf8>\n")
                    }
                }

                println!("Program output:");
                match String::from_utf8(o.stdout) {
                    Ok(s) => println!("{}", s),
                    Err(_) => println!("<could not convert to utf8>\n")
                }
            }
            Err(e) => println!("Execution failed with err \"{}\"", e)
        }
    };

    println!("=== Running with sample input ===");
    run_with_input_file(&in_sample_path);

    if !opt.sample_only {
        println!("=== Running with real input ===");
        run_with_input_file(&in_path);
    }
}