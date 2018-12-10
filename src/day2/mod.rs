use std::fs;

mod checksum;
mod find_similar_ids;

pub fn execute() {
    let input = fs::read_to_string("src/day2/INPUT").expect("Could not read INPUT file for day1");
    println!("2:1 — Box IDs checksum: {}", checksum::checksum(input.as_ref()));
    println!(
        "2:2 — Box IDs common letters: {}",
        find_similar_ids::find_similar_ids(input.as_ref())
    );
}
