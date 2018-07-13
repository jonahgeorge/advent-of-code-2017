use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let valid_passphrase_count1 = input
        .clone()
        .lines()
        .map(|passphrase| {
            let mut uniq = HashSet::new();
            passphrase.split(" ").all(move |word| uniq.insert(word))
        })
        .fold(0, |sum, valid| if valid { sum + 1 } else { sum });
    println!("Valid Passphrases: {}", valid_passphrase_count1);

    let valid_passphrase_count2 = input
        .clone()
        .lines()
        .map(|passphrase| {
            let mut word_set = BTreeSet::new();
            let valid: Vec<bool> = passphrase
                .split(" ")
                .map(|word| {
                    let mut char_map = BTreeMap::new();
                    word.chars().for_each(|c| {
                        let val = char_map.get(&c).unwrap_or(&0) + 1;
                        char_map.insert(c, val);
                    });
                    word_set.insert(char_map)
                })
                .collect();
            !valid.contains(&false)
        })
        .fold(0, |sum, valid| if valid { sum + 1 } else { sum });
    println!("Valid Passphrases: {}", valid_passphrase_count2);
}
