use input::read_input;

mod claim;
mod overlap;
mod overlap_superficy;

pub fn execute() {
    let input = read_input("day3");
    println!("3:1 — Overlap superficy: {}", overlap_superficy::overlap_superficy(input.as_ref()));
}
