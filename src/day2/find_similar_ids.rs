use std::collections::HashMap;
use std::vec::Vec;

pub fn find_similar_ids(box_ids: &str) -> String {
    let mut splitted_ids = SplittedIds::new();
    let mut box_ids_it = box_ids.split("\n");
    loop {
        match box_ids_it.next() {
            Some(id) => {
                let mut id_iterator = IdSplitIterator::from(id).enumerate();
                loop {
                    match id_iterator.next() {
                        Some((i, (before, after))) => {
                            let mut splitted_id = splitted_ids.get(i);
                            let mut afters = splitted_id.get(before);
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

struct SplittedId<'a> {
    content: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> SplittedId<'a> {
    fn new() -> SplittedId<'a> {
        SplittedId {
            content: HashMap::new(),
        }
    }

    fn get(&mut self, before: &'a str) -> &mut Vec<&'a str> {
        self.content.entry(before).or_insert(Vec::new())
    }
}

struct SplittedIds<'a> {
    content: Vec<SplittedId<'a>>,
}

impl<'a> SplittedIds<'a> {
    fn new() -> SplittedIds<'a> {
        SplittedIds {
            content: Vec::new(),
        }
    }

    fn get(&mut self, i: usize) -> &mut SplittedId<'a> {
        if i == self.content.len() {
            self.content.push(SplittedId::new());
        }
        self.content.get_mut(i).unwrap()
    }
}

struct IdSplitIterator<'a> {
    id: &'a str,
    next_index: usize,
}

impl<'a> IdSplitIterator<'a> {
    fn from(id: &str) -> IdSplitIterator {
        return IdSplitIterator { id, next_index: 0 };
    }
}

impl<'a> Iterator for IdSplitIterator<'a> {
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
        use super::super::IdSplitIterator;

        #[test]
        fn should_iterate_over_id_splits() {
            let id = String::from("abcde");
            let mut iterator = IdSplitIterator {
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
