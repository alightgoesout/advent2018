use input::read_input;

mod find_repeating_frequency;
mod frequency;

pub fn execute() {
    let input = read_input("day1");
    println!(
        "1:1 — Frequency: {}",
        frequency::frequency(input.as_ref())
    );
    println!(
        "1:2 — Repeating frequency: {}",
        find_repeating_frequency::find_repeating_frequency(input.as_ref())
    );
}
