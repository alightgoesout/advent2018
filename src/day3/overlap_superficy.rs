use super::claim::Claim;
use super::overlap::overlap;
use std::collections::HashSet;

pub fn overlap_superficy(input: &str) -> usize {
    let mut claims = HashSet::new();
    let mut overlaps = HashSet::<(u32, u32)>::new();
    input.split("\n").for_each(|c| {
        let claim_option = Claim::from(c);
        if claim_option.is_some() {
            let claim = claim_option.unwrap();
            claims.iter().for_each(|c2| overlaps.extend(overlap(&claim, c2)));
            claims.insert(claim);
        }
    });
    overlaps.len()
}

#[cfg(test)]
mod should {
    use super::overlap_superficy;

    #[test]
    fn return_0_for_an_empty_input() {
        assert_eq!(overlap_superficy(""), 0);
    }

    #[test]
    fn return_0_when_there_is_no_overlaps_between_claims() {
        assert_eq!(
            overlap_superficy(
                "#1 @ 1,3: 4x4
#2 @ 6,5: 4x4"
            ),
            0
        );
    }

    #[test]
    fn return_the_overlap_superficy_of_all_claims() {
        assert_eq!(
            overlap_superficy(
                "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
            ),
            4
        )
    }
}
