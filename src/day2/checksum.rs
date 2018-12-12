use std::collections::HashMap;

pub fn checksum(box_ids: &str) -> i32 {
    let mut box_ids_it = box_ids.split("\n");
    let (mut doubled_count, mut tripled_count) = (0, 0);
    loop {
        match box_ids_it.next() {
            Some(id) => {
                let (doubled, tripled) = check_repeating_letters(id);
                if doubled {
                    doubled_count += 1
                }
                if tripled {
                    tripled_count += 1
                }
            }
            None => break,
        }
    }
    doubled_count * tripled_count
}

fn check_repeating_letters(box_id: &str) -> (bool, bool) {
    let letters_count = count_letters(box_id);
    let mut letters_count_it = letters_count.values();
    let (mut doubled, mut tripled) = (false, false);
    loop {
        match letters_count_it.next() {
            Some(v) => {
                if *v == 2 {
                    doubled = true;
                }
                if *v == 3 {
                    tripled = true;
                }
            }
            None => break,
        }
    }
    (doubled, tripled)
}

fn count_letters(box_id: &str) -> HashMap<char, i32> {
    let mut letters_count = HashMap::new();
    let mut box_id_letters = box_id.chars();
    loop {
        match box_id_letters.next() {
            Some(c) => {
                letters_count.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
            None => break,
        }
    }
    letters_count
}

#[cfg(test)]
mod test {
    mod check_repeating_letters {
        use super::super::check_repeating_letters;

        #[test]
        fn should_return_false_false_when_input_has_no_doubled_or_tripled_letter() {
            assert_eq!(check_repeating_letters("abcdef"), (false, false));
        }

        #[test]
        fn should_return_true_false_when_input_has_a_doubled_letter_and_no_tripled_letter() {
            assert_eq!(check_repeating_letters("abbcde"), (true, false));
        }

        #[test]
        fn should_return_false_true_when_input_has_no_doubled_letter_and_a_trippled_letter() {
            assert_eq!(check_repeating_letters("abcccd"), (false, true));
        }

        #[test]
        fn should_return_true_true_when_input_has_a_doubled_letter_and_a_trippled_letter() {
            assert_eq!(check_repeating_letters("bababc"), (true, true));
        }
    }
    mod checksum {
        use super::super::checksum;

        #[test]
        fn should_return_0_when_input_is_empty() {
            assert_eq!(checksum(""), 0);
        }

        #[test]
        fn should_return_0_when_input_is_abcdef() {
            assert_eq!(checksum("abcdef"), 0);
        }

        #[test]
        fn should_return_12_when_input_contains_four_ids_whit_doubled_letters_and_three_ids_with_tripled_letter(
        ) {
            assert_eq!(
                checksum(
                    "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
                ),
                12
            );
        }
    }
}
