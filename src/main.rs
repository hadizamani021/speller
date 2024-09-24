use std::io::BufRead;
use std::{collections::HashMap, fs, io};

fn get_edit1(word: &str) -> Vec<String> {
    let mut res = vec![];
    let mut splited_substrings = vec![];
    let all_letters = "abcdefghijklmnopqrstuvwxyz";

    for i in 0..word.len() {
        splited_substrings.push(word.split_at(i))
    }
    // deletion
    for i in 0..splited_substrings.len() {
        let first_part = splited_substrings[i].0.to_owned();
        let second_part = splited_substrings[i].1;
        let deletion = format!("{}{}", first_part, &second_part[1..]);
        res.push(deletion)
    }
    // insertion
    for i in 0..splited_substrings.len() {
        for j in 0..all_letters.len() {
            let first_part = splited_substrings[i].0;
            let second_part = splited_substrings[i].1;
            let deletion = format!(
                "{}{}{}",
                first_part,
                all_letters.chars().nth(j).unwrap(),
                second_part
            );
            res.push(deletion)
        }
    }
    // replacement
    for i in 0..splited_substrings.len() {
        for j in 0..all_letters.len() {
            let first_part = splited_substrings[i].0;
            let second_part = splited_substrings[i].1;
            let deletion = format!(
                "{}{}{}",
                first_part,
                all_letters.chars().nth(j).unwrap(),
                &second_part[1..]
            );
            res.push(deletion)
        }
    }
    // transposition
    for i in 0..splited_substrings.len() {
        let first_part = splited_substrings[i].0;
        let second_part = splited_substrings[i].1;
        if second_part.len() < 2 {
            continue;
        }
        let deletion = format!(
            "{}{}{}{}",
            first_part,
            second_part.chars().nth(1).unwrap(),
            second_part.chars().nth(0).unwrap(),
            &second_part[2..],
        );
        res.push(deletion)
    }
    return res;
}

fn get_edit2(word: &str, edit1: Option<Vec<String>>) -> Vec<String> {
    let edit1 = match edit1 {
        Some(i) => i,
        _ => get_edit1(word),
    };
    let mut res = vec![];
    for word in edit1.iter() {
        res.append(&mut get_edit1(word));
    }
    return res;
}

fn get_til_edit2(word: &str) -> Vec<String> {
    let mut edit1 = get_edit1(word);
    let mut edit2 = get_edit2(word, Some(edit1.to_owned()));
    edit1.append(&mut edit2);
    return edit1;
}

fn get_number_of_occurrence_of_all_words() -> HashMap<String, i32> {
    let non_meaningfuls = "!@#$%^&*()-=+\"\',._-?/\\";
    let mut content = fs::read_to_string("big.txt")
        .expect("Should have been able to read the file")
        .to_lowercase();
    for letter in non_meaningfuls.chars() {
        content = content.replace(letter, "");
    }
    let lines = content.split("\n");
    let mut res: HashMap<String, i32> = HashMap::new();
    for line in lines {
        for word in line.split(" ") {
            *res.entry(word.trim().to_owned()).or_insert(0) += 1;
        }
    }
    return res;
}

fn get_probability_of_each_word() -> HashMap<String, f32> {
    let mut res: HashMap<String, f32> = HashMap::new();
    let number_of_occurrence_of_all_words = get_number_of_occurrence_of_all_words();
    let number_of_all_words = number_of_occurrence_of_all_words.len() as f32;
    for (key, value) in number_of_occurrence_of_all_words.into_iter() {
        res.insert(key, value as f32 / number_of_all_words);
    }
    return res;
}

fn main() {
    let mut misspelled_word = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut misspelled_word).unwrap();
    let probability_of_each_word = get_probability_of_each_word();
    let til_edit2 = get_til_edit2(&misspelled_word);
    let known_words = til_edit2
        .into_iter()
        .filter(|word| match probability_of_each_word.get(word) {
            Some(_) => true,
            _ => false,
        })
        .collect::<Vec<_>>();
    let mut probablity_of_each_known_word: HashMap<String, f32> = HashMap::new();
    for word in known_words.into_iter() {
        probablity_of_each_known_word
            .insert(word.clone(), *probability_of_each_word.get(&word).unwrap());
    }
    let mut sorted_known_words: Vec<_> = probablity_of_each_known_word.into_iter().collect();
    sorted_known_words.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
    println!("Maybe you wanted to type: ");
    for word_and_probability in sorted_known_words.into_iter() {
        println!("{}", word_and_probability.0)
    }
}
