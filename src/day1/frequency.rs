pub fn frequency(input: &str) -> i32 {
    let mut frequency = 0;
    let mut input_it = input.split("\n");
    loop {
        match input_it.next() {
            Some(s) => {
                let parsed = s.parse::<i32>();
                if parsed.is_ok() {
                    frequency += parsed.unwrap();
                }
            }
            None => break,
        }
    }
    frequency
}

#[cfg(test)]
mod should {
    use super::frequency;

    #[test]
    fn return_0_with_an_empty_input() {
        assert_eq!(frequency(""), 0);
    }

    #[test]
    fn return_1_when_input_is_1() {
        assert_eq!(frequency("+1"), 1);
    }

    #[test]
    fn return_negative_1_when_input_is_negative_one() {
        assert_eq!(frequency("-1"), -1);
    }

    #[test]
    fn return_the_sum_of_all_numbers_in_input() {
        assert_eq!(
            frequency(
                "+1
-2
+3
+1"
            ),
            3
        );
    }
}
