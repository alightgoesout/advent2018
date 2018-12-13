#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::stdin;
use std::io::stdout;
use std::io::Error;
use std::io::Write;

mod input;
mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Error> {
    let puzzle = read_console_input()?;
    execute_puzzle(puzzle);
    Ok(())
}

fn read_console_input() -> Result<String, Error> {
    print!("Puzzle to run: ");
    stdout().flush()?;
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line)
}

fn execute_puzzle(puzzle: String) {
    match puzzle.as_ref() {
        "1\n" => day1::execute(),
        "2\n" => day2::execute(),
        "3\n" => day3::execute(),
        s => println!("Unknown puzzle: '{}'", s),
    }
}
