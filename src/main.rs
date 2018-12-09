use std::io::Error;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod box_ids;
mod frequency;

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
        "1:1\n" => println!(
            "1:1 — Frequency: {}",
            frequency::frequency(frequency::INPUT)
        ),
        "1:2\n" => println!(
            "1:2 — Repeating frequency: {}",
            frequency::find_repeating_frequency(frequency::INPUT)
        ),
        "2:1\n" => println!(
            "2:1 — Box IDs checksum: {}",
            box_ids::checksum(box_ids::INPUT)
        ),
        "2:2\n" => println!(
            "2:2 — Box IDs common letters: {}",
            box_ids::find_similar_ids(box_ids::INPUT)
        ),
        s => println!("Unknown puzzle: '{}'", s),
    }
}
