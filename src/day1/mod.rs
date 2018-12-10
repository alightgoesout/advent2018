use std::fs;

mod find_repeating_frequency;
mod frequency;

pub fn execute() {
    let input = fs::read_to_string("src/day1/INPUT").expect("Could not read INPUT file for day1");
    println!(
        "1:1 — Frequency: {}",
        frequency::frequency(input.as_ref())
    );
    println!(
        "1:2 — Repeating frequency: {}",
        find_repeating_frequency::find_repeating_frequency(input.as_ref())
    );
}
