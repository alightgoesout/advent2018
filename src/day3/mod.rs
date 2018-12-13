use std::fs;

mod claim;
mod overlap;
mod overlap_superficy;

pub fn execute() {
    let input = fs::read_to_string("src/day3/INPUT").expect("Could not read INPUT file for day3");
    println!("3:1 — Overlap superficy: {}", overlap_superficy::overlap_superficy(input.as_ref()));
}
