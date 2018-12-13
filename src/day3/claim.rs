use std::hash::{Hash, Hasher};
use regex::Regex;

pub struct Claim {
    pub id: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

lazy_static! {
    static ref claim_regex: Regex =
        Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)")
            .unwrap();
}

impl Claim {
    pub fn from(claim_description: &str) -> Option<Claim> {
        claim_regex.captures(claim_description).map(|c| Claim {
            id: c.name("id").unwrap().as_str().parse().unwrap(),
            left: c.name("left").unwrap().as_str().parse().unwrap(),
            top: c.name("top").unwrap().as_str().parse().unwrap(),
            width: c.name("width").unwrap().as_str().parse().unwrap(),
            height: c.name("height").unwrap().as_str().parse().unwrap(),
        })
    }
}

impl PartialEq for Claim {
    fn eq(&self, other: &Claim) -> bool {
        self.id == other.id
    }
}

impl Eq for Claim {}

impl Hash for Claim {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
    }
}

#[cfg(test)]
mod should {
    use super::Claim;

    #[test]
    fn parse_a_string_and_create_a_claim() {
        let claim_option = Claim::from("#1 @ 2,3: 4x5");

        assert!(claim_option.is_some());
        let claim = claim_option.unwrap();
        assert_eq!(claim.id, 1);
        assert_eq!(claim.left, 2);
        assert_eq!(claim.top, 3);
        assert_eq!(claim.width, 4);
        assert_eq!(claim.height, 5);
    }

    #[test]
    fn return_none_when_the_string_is_malformed() {
        let claim_option = Claim::from("1  2,3: 4x5");

        assert!(claim_option.is_none());
    }
}
