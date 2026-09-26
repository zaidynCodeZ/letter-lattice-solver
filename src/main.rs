use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::process;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 || args.len() > 3 {
        eprintln!("Incorrect arguments. run -- <WORD_1> <WORD_2>");
        process::exit(1);
    }
    if args[1].len() != args[2].len() {
        eprintln!("Words must be the same length.");
        process::exit(1);
    }

    let word1 = args[1].to_lowercase();
    let word2 = args[2].to_lowercase();
    
    let lattice = generate_lattice(&word1, &word2)?;
    let mut chain: Vec<String> = Vec::new();
    explore_lattice(&lattice, &word1, &word2, &mut chain);
    chain.pop();
    Ok(())
}

fn evaluate(word: &str, letters: &Vec<char>) -> bool {
    let mut l: Vec<char> = word.chars().collect();
    l.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    while i < l.len() && j < letters.len() {
        if l[i] == letters[j] {
            i += 1;
            j += 1;
        } else if l[i] > letters[j] {
            j += 1;
        } else {
            return false;
        }
    }
    if i < l.len() {
        return false;
    }
    true
}

fn distance(word: &str, target: &str) -> i32 {
    let mut pairs = 0;
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort_unstable();
    let mut target_chars: Vec<char> = target.chars().collect();
    target_chars.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    while i < word_chars.len() && j < target_chars.len() {
        if word_chars[i] == target_chars[j] {
            i += 1;
            j += 1;
            pairs += 1;
        } else if word_chars[i] < target_chars[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    word.len() as i32 - pairs
}

fn words_list(word1: &str, word2: &str) -> Result<Vec<String>, std::io::Error> {
    let mut words_list: Vec<String> = Vec::new();

    let words = File::open("processed_words_list.txt")?;
    let reader = BufReader::new(words);

    let word_len = word1.len();
    let mut letter_set = Vec::with_capacity(word_len * 2);
    for c in word1.chars() {
        letter_set.push(c);
    }
    for c in word2.chars() {
        letter_set.push(c);
    }
    letter_set.sort_unstable();
    
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.len() == word_len && evaluate(&line, &letter_set) {
            words_list.push(line);
        } else if line.len() > word_len {
            break;
        }
    }
    words_list.sort_unstable_by(|a, b| distance(a, word1).cmp(&distance(b, word1)));
    Ok(words_list)
}

fn process_words_list(words_list: &Vec<String>, target: &str) -> Vec<Vec<String>> {
    let mut processed_words_list: Vec<Vec<String>> = Vec::new();
    let mut current_distance = distance(&words_list[0], target);
    let mut current_distance_words: Vec<String> = Vec::new();
    for word in words_list {
        let d = distance(&word, target);
        if d == current_distance {
            current_distance_words.push(word.to_string());
        } else {
            processed_words_list.push(current_distance_words);
            current_distance_words = Vec::new();
            current_distance_words.push(word.to_string());
            current_distance = d;
        }
    }
    processed_words_list.push(current_distance_words);
    processed_words_list
}

fn generate_lattice(word1: &str, word2: &str) -> Result<HashMap<String, Vec<String>>, std::io::Error> {
    let words_list = words_list(word1, word2)?;
    if words_list.len() == 0 {
        eprintln!("No words found.");
        process::exit(1);
    }

    let levels: Vec<Vec<String>> = process_words_list(&words_list, word2);
    let mut lattice = HashMap::new();
    for i in 0..levels.len() - 1 {
        let dist = distance(&levels[i][0], word2);
        for word in &levels[i] {
            let mut next_words: Vec<String> = Vec::new();
            if word == word2 {
                continue;
            }
            if distance(word, word2) == 1 {
                next_words.push(word2.to_string());
                lattice.insert(word.to_string(), next_words);
                continue;
            }
            for next_word in &levels[i + 1] {
                if distance(word, next_word) == 1 && distance(next_word, word2) < dist {
                    next_words.push(next_word.to_string());
                }
            }
            lattice.insert(word.to_string(), next_words);
        }
    }

    Ok(lattice)
}

fn explore_lattice(lattice: &HashMap<String, Vec<String>>, current_word: &str, target_word: &str, chain: &mut Vec<String>) {
    chain.push(current_word.to_string());
    if current_word == target_word {
        println!("{:?}", chain);
    } else {
        if let Some(next_words) = lattice.get(current_word) {
            for next_word in next_words {
                explore_lattice(lattice, next_word, target_word, chain);
                chain.pop();
            }
        }
    }
}



