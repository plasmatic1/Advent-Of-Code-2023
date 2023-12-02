use std::process::exit;
use std::process::Command;

const USAGE: &str = "Usage: ./main <day_no> <task_no>";

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();
    let day_no = i32::from_str_radix(&args.get(1).expect(USAGE), 10).expect(USAGE);
    let task_no = i32::from_str_radix(&args.get(2).expect(USAGE), 10).expect(USAGE);
    if !(1 <= day_no && day_no <= 25) {
        println!("Invalid day {}!", day_no);
        exit(1);
    }
    if !(1 <= task_no && task_no <= 2) {
        println!("Invalid task number {}! (must be 1 or 2)", task_no);
        exit(1);
    }
    println!("Problem: day {} task {}", day_no, task_no);

    // run proc
    let in_sample_path = format!("inputs/day{}.s{}.txt", day_no, task_no);
    let in_path = format!("inputs/day{}.txt", day_no);
    println!("Sample input path: {}", &in_sample_path);
    println!("Input path: {}", &in_path);

    // Get soln path
    let soln_ext = match std::env::consts::OS {
        "windows" => ".exe",
        _ => ""
    };
    let soln_path = format!("target/debug/day{}{}", day_no, soln_ext);
    println!("Solution path: {}", soln_path);

    println!();

    let run_with_input_file = |path: &str| {
        match Command::new(&soln_path).arg(&task_no.to_string()).arg(path).output() {
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

    println!("=== Running with real input ===");
    run_with_input_file(&in_path);
}