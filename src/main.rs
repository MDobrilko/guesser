use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

const WORD_LENGTH: usize = 6;

fn main() {
    algorithm(read_words());
}

fn read_words() -> Vec<String> {
    let file = File::open("./russian_nouns.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut dict = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => dict.push(line.trim().to_string()),
            Err(e) => println!("Cant readline {} because of {}", i, e),
        }
    }

    dict
}

fn algorithm(dict: Vec<String>) {
    let mut dict = dict;

    let ignored_letters: HashSet<char> = "".chars().collect();
    let forbidden_letters: Vec<(char, Vec<usize>)> = vec![
        // ('', vec![]),
    ];
    let known_letters = "_ _ _ _ _ _";

    let forbidden_letters: HashMap<char, Vec<usize>> = HashMap::from_iter(
        forbidden_letters
            .into_iter()
            .map(|(l, idxs)| (l, idxs.into_iter().map(|i| i - 1).collect::<Vec<usize>>())),
    );
    let containing_letters: HashSet<char> = forbidden_letters.keys().copied().collect();
    let known_letters: Vec<Option<char>> = known_letters
        .chars()
        .filter(|symb| !symb.is_whitespace())
        .map(|symb| {
            if symb.is_alphabetic() {
                Some(symb)
            } else {
                None
            }
        })
        .chain(std::iter::repeat(None))
        .take(WORD_LENGTH)
        .collect();

    dict.retain(|word| {
        if word.chars().count() != WORD_LENGTH {
            return false;
        }
        if word.chars().any(|symb| !symb.is_alphabetic()) {
            return false;
        }

        let mut word_letters = HashSet::new();
        for (i, letter) in word.chars().enumerate() {
            if ignored_letters.contains(&letter) {
                return false;
            } else if forbidden_letters
                .get(&letter)
                .unwrap_or(&vec![])
                .iter()
                .any(|&idx| idx == i)
            {
                return false;
            } else if known_letters[i].map(|symb| symb != letter).unwrap_or(false) {
                return false;
            }

            word_letters.insert(letter);
        }

        !containing_letters
            .difference(&word_letters)
            .next()
            .is_some()
    });

    for word in dict.iter() {
        println!("{}", *word);
    }
    
    println!("Total words count: {}", dict.len());
}
