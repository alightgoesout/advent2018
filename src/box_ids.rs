use std::collections::HashMap;
use std::vec::Vec;

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
    mod check_repeating_letters {
        use box_ids::check_repeating_letters;

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
        use box_ids::checksum;

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

    mod id_split {
        use box_ids::IdSplit;

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
        use box_ids::find_similar_ids;

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

pub const INPUT: &str = "bpacnmelhhzpygfsjoxtvkwuor
biacnmelnizqygfsjoctvkwudr
bpaccmllhizyygfsjoxtvkwudr
rpacnmelhizqsufsjoxtvkwudr
bfacnmelhizqygfsjoxtvwwudp
bpacnmelhizqynfsjodtvkyudr
bpafnmelhizqpgfsjjxtvkwudr
bpackmelhizcygfsjoxtvkwudo
bmacnmilhizqygfsjoltvkwudr
bpafnmelhizuygfsjoxtvkwsdr
boacnmylhizqygfsjoxtvxwudr
bpbcjmelhizqygfsjoxtgkwudr
bpacnmglhizqygfsjixtlkwudr
bpacnmclhizqygfsjoxtvkwtqr
bpacnmelhczqygtsjoptvkwudr
bpacnmelhizqywfsaoxtvkbudr
apacnmelhizqygcsjoxtvkwhdr
bpacnmelrizqygfsbpxtvkwudr
tpkcnmelpizqygfsjoxtvkwudr
bpacnmelhizqlgfsjobtmkwudr
npacnmelhizqygffjoxtvkwudf
bpacnmeehqzqygqsjoxtvkwudr
bpecnmelhizqigfsjvxtvkwudr
bpacnmelhizqysfsjoxtvkdfdr
bpacnfelhkzqygfsjoxtvkwfdr
bpacnbelvizqygfsjoxthkwudr
bpacnoelhizqygfejoxtvkwudn
bpacnmelhizqygfzpkxtvkwudr
bpahnmelhizqyufsjoxmvkwudr
bpacnmelhizqygfsnoxtvkwmmr
bpacnmelhizqygfsjoatvkludf
bpacnmylhizqygfsjlxtvksudr
bpacnmekhpzqygysjoxtvkwudr
bpacnselhizqogfswoxtvkwudr
bpacnmelhizqprfsjoxwvkwudr
bpatnmelhinqygfsjoctvkwudr
bpacnqelhqzqygfsxoxtvkwudr
bpabnmelhiyqygfsjoxtykwudr
bpacnivlhizqygfsjoxtviwudr
bpkcnmylhizqygfsjoxtvkwcdr
bpafnmflhizqygtsjoxtvkwudr
bpachmelhizqygfsjixtvkwudg
bpacymelhizqygfsjoxtykwuar
bpacnkelhizqdgfsjoxtskwudr
bpacnmezhizqggbsjoxtvkwudr
bpacnmqlhizqygrsjoxzvkwudr
bpaczmelhizqyhfsjoxfvkwudr
bdacnmelhyzqygusjoxtvkwudr
bpacbmelhizqywfsjostvkwudr
bpacnmelhihzygfstoxtvkwudr
bpactmelhizqygfsjcxtvkwydr
bkacnmethizqytfsjoxtvkwudr
bpacnmalhizqydfskoxtvkwudr
spacnmelbizqygfsjoxdvkwudr
lpalnmelhizoygfsjoxtvkwudr
bpacjmeghizqygfsjoxtviwudr
bpacnmeqhizxygfsjoxgvkwudr
bpacnmelhizqygosjoxtvkkuhr
bpacnmelhiznbxfsjoxtvkwudr
bgacnmelhizqygfsjbxivkwudr
bpacnmelhizqygfjjowtvswudr
bpacnmelhizqygfsjovtgkmudr
bpacnmelcmzqygfspoxtvkwudr
bpvcnmelhizqyvfcjoxtvkwudr
bpacnmeahizqjgfsjoxtvkwukr
bpacnoelwizqygfsjoxtvkaudr
xpacnmelhizqygfsjoxdvkwedr
mpacnmelqizqygfsjoxtvkwudx
bppcnmelhizqygfsjfxtvkhudr
bpacnmclhizqyhfsjaxtvkwudr
opacsmelhizqygfsjmxtvkwudr
bpafnmelhizqjgfsjoxtvkrudr
bpdcnmilhizqygfsjoxtvkludr
bpainmelhizqygfsjtntvkwudr
bradnmelhizqygfsjextvkwudr
bpacnmelhizqygfmsoxtvkwudg
bpacneelhizqygvrjoxtvkwudr
bpacnpelhizqygfsjoxyvkwudf
bpacnmelhizqygfsqoqtvkwodr
bpacnmelhizjyghsjoxcvkwudr
bpacnmelmibqygfsjoxtvnwudr
jpacnmelaizqygfwjoxtvkwudr
zpachmelhizqygfsjsxtvkwudr
bpacnmelfizqykfsjomtvkwudr
bpacnmllwizqygfsjoxtvkwusr
bpaynmelhizqygfsjoxtvowcdr
jpacnmqlhizqygfsjoxtvknudr
bpacxmelhizqyffsjoxtvkwugr
apawnmelhizqygfsjtxtvkwudr
mpacnmelhitqigfsjoxtvkwudr
bpacnmelhhzqygfsjoxtvkyzdr
gpacnmelhizqynfsjoxtvkwudm
bnacnkelhizqygfsjoxtpkwudr
bpacnmelfizqygfsumxtvkwudr
bpacnmelhisqygfsjohtvowudr
bpacnmelhimqygxsjoxtvkwudn
bpscnmeliizqygfsjoxtvkwunr
qpacnmelhizqycfsjoxtvkwndr
bpacnmelhijqygfsjohtvkyudr
bpacnmelhizqykfsjkxtvknudr
bpacnqilhizqygfsjoxtvkoudr
bpacnmelhizqzgmsjoxtvkwurr
bpdcnmelhizqygfsjoutukwudr
bpecnmeghizqygfsjoxgvkwudr
bpicnmelhizqygfrjoxtvlwudr
bpacnmelhizfygfsroxtvkwodr
buacnmelhizqygjsjoxtvkvudr
bpacnmelhixqykfsjoxtvrwudr
bpacnmelhizqygvejcxtvkwudr
bpacnmjlhizqylfsjoxtvkwuor
qpacnmelhizqygfsjoxfdkwudr
bpfcnmemhizqygfsjoxtvknudr
bpacnmelhizqoffsjqxtvkwudr
hpacnielhiqqygfsjoxtvkwudr
gpacnmelhizqygfsewxtvkwudr
bpacnmellizqylxsjoxtvkwudr
bpacnmenhizqymfsjoxtvkmudr
bpacnfelhizqygcsjoltvkwudr
bpacnmelhqqqygfsjoxtvkuudr
bplgnmelhiqqygfsjoxtvkwudr
bpacnzelhizqygfgjoxtvnwudr
bpacnmelhizqygfsjoktvknunr
bpacnmdlhioqygfnjoxtvkwudr
epacnmelwizqyjfsjoxtvkwudr
bpacxmelhazfygfsjoxtvkwudr
bpacnmejhezqygfsjoxtskwudr
bpacnqelhihqyzfsjoxtvkwudr
bpacnbelhizqyrfsjoxtvkmudr
bpacnmelhizqygfsjoxtylwzdr
bpacnmelwizqygfsjodtvkhudr
bpacnnelhizqygfsjoxtwkwadr
bpacimelhizqygfsnoxtvkwuor
bpacnmelhizqyaasjoxtlkwudr
bpacnmelhizqyeffjoxtvkwuds
bpacnmenhizqygxscoxtvkwudr
bpacnmelhidqygfsjowtskwudr
bpacnmeliizqygfsjoxhvkwucr
bpacimelhizqygfsjoxtvktuwr
bpainmelhhzqygfsjzxtvkwudr
bpacamelhizqygfsjogtvkwbdr
bpccnmelgizqygfsjoxtykwudr
bpacnmelhizwegfsjoxtvkwadr
bpackmelhbzqygqsjoxtvkwudr
bpacymeihizqyffsjoxtvkwudr
bpacnielhczqygfsjoxtvkwudk
bpacnmejhizqygffjoxjvkwudr
ppacnmelhizqygfsjoxtigwudr
bpjcnmolhizqygfsjoxtvkwndr
bpacnmelcizqygrsjoxtakwudr
cpawnmelhizqygfsjoxmvkwudr
bwacnmelhizqygesjoxtakwudr
bpacnmelhizqygfsjexsvkwddr
bpaunmelhiuqygfsjoxtvkwtdr
bpacnmellimqygfsjextvkwudr
bpacnmerhizqygfsaoxvvkwudr
bpacnmglhizqygfsjixtukwudr
ppacnmelhizqygfsjoxtvkdudp
bpacnmedhizqygukjoxtvkwudr
bpccnmelhizqngfsjoxtvkwadr
bgacnmeldizqygfscoxtvkwudr
bpacngelhizsygfsjoxtvkwkdr
bpacnpelhizqygfsjoxctkwudr
bpacnmylhizqygfcjoxtvkwmdr
npacnmelhizqygfsjoxtwkwuds
bpaxnmelhizqydfsjoxyvkwudr
bpacnhelhizjygfsjoxtvkmudr
bpacnkelhczqygfnjoxtvkwudr
bfacnmelhizrygfsjoxtvkwodr
bpycnmelhizqygfofoxtvkwudr
qpacpselhizqygfsjoxtvkwudr
bpvcnmelhezqygfsjoxttkwudr
bpacnmwlhizqygfijoxtmkwudr
bsacnmelhikqygfsjoxttkwudr
bpccnxelhizqyafsjoxtvkwudr
bpacnmelhizqygfswhxtvewudr
vpacnmzlhizqygfsvoxtvkwudr
bpacnmelhihqygfsjoxtvkqurr
bpacnmelhixqygazjoxtvkwudr
bpavnmelhizqygfsjozpvkwudr
bpacnmclhizuygfsjoxmvkwudr
bpacnmelhizryufsjoxtkkwudr
bpacnmelhtzqygfsjobtvkwufr
bpacnmelhizqmlfsjoxtvkwudq
bpaaneelhizqygfsjlxtvkwudr
bpacnmelhxzqygfsjoxthkwuhr
bpacnmeshizqygfcjoxtvkwude
bpacnzqlhizqygfsxoxtvkwudr
bgaanmelhizqycfsjoxtvkwudr
bpacnmexhizqygfsroxtvkwudn
bpmmnmelhizqygfajoxtvkwudr
bpacnmelhizqylfsjoxtckwhdr
bpicnmelhizqyrfsjoxtvkwudi
zpacnmelhizvycfsjoxtvkwudr
bpamnmkllizqygfsjoxtvkwudr
bpacnmelhrzqyrfsjoxgvkwudr
bpadnmelhczqygfsjoxtlkwudr
bpacrmelhizqygrsjoxtvkiudr
lpacnmelhizqygfsjoxtgkwxdr
fpacnmalhiuqygfsjoxtvkwudr
bpacnmelhizqygfsjixtvfwcdr
bpccnmelhxzqygfkjoxtvkwudr
bpacnmepaizqygfsjoctvkwudr
tpacnmelhivqygfsxoxtvkwudr
kpacnfelhitqygfsjoxtvkwudr
baacnzelhizqygfsjoxtvkwudx
bcycnmeghizqygfsjoxtvkwudr
wpacotelhizqygfsjoxtvkwudr
bpacnmsshizqygrsjoxtvkwudr
blacnmelhizqygfsjoxtykwvdr
bkacnmelhizqygfsjoxuvkludr
bpacnmelhizaugfsjoxtvhwudr
fpavnmelhizqygfsgoxtvkwudr
bpachmelnizqygfsjextvkwudr
bpacnmelhizqpgfsjoxtvkwldu
bpacnmelhizqygfsloftvywudr
bpacntelhvzqygfejoxtvkwudr
bpacnmeldizqygfsjmxtvkdudr
byacnmelhizqygfsjsxtvkwudh
bpacnmellizqygssxoxtvkwudr
bpacnmelhizqygfsjootvknuir
bpacnmelhitqjgfsjoxivkwudr
bpacnmelhazaygfsjoxtvfwudr
bpacnzenhizqygfsjzxtvkwudr
bpacnmelhizqypfsdoxtvkwuar
bpannmelhizqygnsjoxtvkwndr
bracnmeldizsygfsjoxtvkwudr
bpacnmelhizwygfsjugtvkwudr
bpatnmelhizqygfsjoytvkwulr
upacnmelhizqygfsjurtvkwudr
bpaenmezhizqygfsjostvkwudr
bpacnmelhizpygfsjodhvkwudr
bpacnmelhizqygfsjogtvkguwr
bpacnmelhisqygfsjoxtpkuudr
bxacnmelhizqygfsjdxtvkfudr
bpacnmelhizqygfsjohqvkwudu
bzacnmtlhizqygfsjoxsvkwudr
bpacnmplhixrygfsjoxtvkwudr
bpacnmelhizqhgfsjomtvkwudg
bpacnmezhizqygfsjxxtykwudr
bpacnmwlhizqygfujoxtzkwudr
tpacnmelhizqygfsjoxkvpwudr
bpawsmenhizqygfsjoxtvkwudr
bpacnmelhizqtgfsjoxttkwuqr
bpkcbmelhizqygfsjoxtvkwucr
bpacfmekhizqygfsjoxtvkwuds
bpacnmethizqynfajoxtvkwudr
bpocnmclhizqygfsjoxtvkwukr
zpacnmwlhizqygfsjoxzvkwudr
bpacpoelhqzqygfsjoxtvkwudr
bpacnlelhizqyzfsjoxtvkwukr
";
