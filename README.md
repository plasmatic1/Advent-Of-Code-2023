# advent-of-code-2023

Advent of Code 2023, in Rust.

To run individual solutions, use `cargo run` or `cargo build` then `./target/debug/aoc[.exe]`.

## Project hierarchy

* `lib.rs`: Common libraries used for AoC
* **Runner** (main binary):
    * Found in `target/debug/aoc[.exe]`
    * Executor for individual solutions.  Run executable for help
* **Solutions** (binaries in `bin/`):
    * Solutions for individual days.  Each binary is executed with `./dayX <task_no> <input_file_path>`
* Input files are in `inputs/`, where `dayX.txt` is the full input for that day, while `dayX.s1.txt` and `dayX.s2.txt` are the sample inputs for both tasks 1 and 2 respectively

### Notes

* Other directory `misc`: Contains Python solution and misc notes for Day 1.  Didn't have time to get ready

## Completed Days

* Day 1 `**`: Rust, Python
* Day 2 `**`: Rust