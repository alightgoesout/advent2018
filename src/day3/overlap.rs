use std::cmp;
use std::collections::HashSet;
use super::claim::Claim;

pub fn overlap(c1: &Claim, c2: &Claim) -> HashSet<(u32, u32)> {
    let mut overlap = HashSet::new();
    let x_min = cmp::max(c1.left, c2.left);
    let x_max = cmp::min(c1.left + c1.width, c2.left + c2.width);
    let y_min = cmp::max(c1.top, c2.top);
    let y_max = cmp::min(c1.top + c1.height, c2.top + c2.height);
    for x in x_min..x_max {
        for y in y_min..y_max {
            overlap.insert((x, y));
        }
    }
    overlap
}

#[cfg(test)]
mod should {
    use super::super::claim::Claim;
    use super::overlap;

    #[test]
    fn should_return_the_overlap_between_two_claims() {
        let c1 = Claim::from("#1 @ 1,3: 4x4").unwrap();
        let c2 = Claim::from("#2 @ 3,1: 4x4").unwrap();

        let result = overlap(&c1, &c2);

        assert_eq!(result.len(), 4);
        assert!(result.contains(&(3, 3)));
        assert!(result.contains(&(3, 4)));
        assert!(result.contains(&(4, 3)));
        assert!(result.contains(&(4, 4)));
    }

    #[test]
    fn should_return_an_empty_set_when_there_is_no_overlap() {
        let c1 = Claim::from("#1 @ 1,3: 4x4").unwrap();
        let c2 = Claim::from("#2 @ 6,5: 4x4").unwrap();

        let result = overlap(&c1, &c2);

        assert_eq!(result.len(), 0);
    }
}
