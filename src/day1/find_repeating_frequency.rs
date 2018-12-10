use std::collections::HashSet;

pub fn find_repeating_frequency(input: &str) -> i32 {
    let mut frequency = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(frequency);
    loop {
        let mut input_it = input.split("\n");
        loop {
            match input_it.next() {
                Some(s) => {
                    let parsed = s.parse::<i32>();
                    if parsed.is_ok() {
                        frequency += parsed.unwrap();
                        if frequencies.contains(&frequency) {
                            return frequency;
                        }
                        frequencies.insert(frequency);
                    }
                }
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod should {
    use super::find_repeating_frequency;

    #[test]
    fn return_0_when_input_is_plus_1_minus_1() {
        assert_eq!(
            find_repeating_frequency(
                "+1
-1"
            ),
            0
        );
    }

    #[test]
    fn return_2_when_input_is_plus_1_minus_2_plus_3_plus_1() {
        assert_eq!(
            find_repeating_frequency(
                "+1
-2
+3
+1"
            ),
            2
        );
    }

    #[test]
    fn return_10_when_input_is_plus_3_plus_3_plus_4_minus_2_minus_4() {
        assert_eq!(
            find_repeating_frequency(
                "+3
+3
+4
-2
-4"
            ),
            10
        );
    }
}
