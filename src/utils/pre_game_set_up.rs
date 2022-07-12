use rand::Rng; // 0.8.0
use std::fs::File;
use std::io::prelude::*;

pub struct Letter {
    pub(crate) letter: char,
    pub(crate) revealed: bool
}

pub fn get_game_answer() -> Vec<Letter> {
    let word = get_word(rand::thread_rng().gen_range(1..855));
    let mut result: Vec<Letter> = Vec::new();

    for c in word.chars() {
        let temp = Letter{
            letter: c,
            revealed: false
        };
        result.push(temp)
    }
    return result;
}

fn get_word(index: i32) -> String {
    let mut f = File::open("src/words.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let vec = contents.split("\n").collect::<Vec<&str>>();
    return String::from(vec[index as usize]);
}

pub fn get_drawings() -> [&'static str;7] {
    return [
        "
        +---+
        |   |
            |
            |
            |
            |
    =========",
        "
        +---+
        |   |
        O   |
            |
            |
            |
    =========",
        "
        +---+
        |   |
        O   |
        |   |
            |
            |
    =========",
        "
        +---+
        |   |
        O   |
        |\\  |
            |
            |
    =========",
        "
        +---+
        |   |
        O   |
       /|\\  |
            |
            |
    =========",
        "
        +---+
        |   |
        O   |
       /|\\  |
       /    |
            |
    =========",
        "
        +---+
        |   |
        O   |
       /|\\  |
       / \\  |
            |
    =========",
    ];
}