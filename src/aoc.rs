use std::{fs::File, io::Read};
use std::env;

const USAGE: &'static str = "Usage: ./dayX <task # (1/2)> <input_path>";

pub struct AOCArgs {
    pub task: i32,
    pub path: String
}

/**
 * Read input contents with proper error checking
 */
fn get_input() -> String {
    let args = get_args();
    let mut file = File::open(args.path).expect("File not found");
    let sz = file.metadata().expect("Could stat file").len();
    let mut buf: Vec<u8> = Vec::with_capacity(sz as usize);
    file.read_to_end(&mut buf).expect("Could not read file");
    String::from_utf8(buf).expect("Could not convert contents to UTF-8")
}

/**
 * Resolve task number
 */
pub fn get_args() -> AOCArgs {
    let args: Vec<String> = env::args().collect();
    AOCArgs {
        task: i32::from_str_radix(&args.get(1).expect(USAGE), 10).expect(USAGE),
        path: args.get(2).expect(USAGE).clone()
    }
}

/**
 * Splits a string into tokens (by whitespace)
 */
pub fn get_input_tokens() -> Vec<String> {
    get_input().split(char::is_whitespace).filter(|s| !s.is_empty()).map(str::trim).map(str::to_owned).collect()
}

/**
 * Splits a string into tokens (by lines)
 */
pub fn get_input_lines() -> Vec<String> {
    get_input().trim().split('\n').map(str::trim).map(str::to_owned).collect()
}