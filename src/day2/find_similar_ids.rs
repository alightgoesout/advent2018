use std::collections::HashMap;
use std::vec::Vec;

pub fn find_similar_ids(box_ids: &str) -> String {
    let mut id_splits = HashMap::new();
    let mut box_ids_it = box_ids.split("\n");
    loop {
        match box_ids_it.next() {
            Some(id) => {
                let mut id_iterator = IdSplit {
                    id: &id,
                    next_index: 0,
                }
                .enumerate();
                loop {
                    match id_iterator.next() {
                        Some((i, (before, after))) => {
                            let index_splits = id_splits.entry(i).or_insert(HashMap::new());
                            let mut afters = index_splits.entry(before).or_insert(Vec::new());
                            if afters.contains(&after) {
                                return before.to_owned() + after;
                            }
                            afters.push(after);
                        }
                        None => break,
                    }
                }
            }
            None => break,
        }
    }
    String::from("")
}

struct IdSplit<'a> {
    id: &'a str,
    next_index: usize,
}

impl<'a> Iterator for IdSplit<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.id.len() {
            return None;
        }
        let result = Some((
            &self.id[..self.next_index],
            &self.id[(self.next_index + 1)..],
        ));
        self.next_index += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    mod id_split {
        use super::super::IdSplit;

        #[test]
        fn should_iterate_over_id_splits() {
            let id = String::from("abcde");
            let mut iterator = IdSplit {
                id: &id,
                next_index: 0,
            };

            assert_eq!(Some(("", "bcde")), iterator.next());
            assert_eq!(Some(("a", "cde")), iterator.next());
            assert_eq!(Some(("ab", "de")), iterator.next());
            assert_eq!(Some(("abc", "e")), iterator.next());
            assert_eq!(Some(("abcd", "")), iterator.next());
            assert_eq!(None, iterator.next());
        }
    }

    mod find_similar_ids {
        use super::super::find_similar_ids;

        #[test]
        fn should_return_common_letters_of_the_two_similar_ids() {
            assert_eq!(
                find_similar_ids(
                    "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
                ),
                "fgij"
            )
        }
    }
}
