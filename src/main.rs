use std::io;
use rand::Rng; // 0.8.0

use std::fs::File;
use std::io::prelude::*;

struct Letter {
    letter: char,
    revealed: bool
}

fn main() {
    let hangman_pics = [
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
    let mut lives = 0;
    let mut answer = get_game_answer();

    println!("This is hangman game!");
    let mut flag = false;
    while !flag & (lives<hangman_pics.len()-1){
        // print start & blanks
        println!("{}", hangman_pics[lives]);
        println!("{}", get_user_answer(&answer));
        let guess = read_user_input_character();
        flag = true;
        let mut change = false;
        for i in 0..answer.len(){
            if answer[i].letter == guess{
                answer[i].revealed = true;
                change = true;
            }
            flag = flag & answer[i].revealed;
        }
        // if its not increment lives
        if !change {
            lives = lives + 1;
        }
    }

    if lives == hangman_pics.len()-1{
        println!("{}", hangman_pics[hangman_pics.len()-1]);
        println!("You lose!");
        println!("The word you couldn't get was {}", get_result(&answer) );
    } else {
        println!("
            +---+
            |   |
                |
       \\O/      |
        |       |
       / \\      |
    =============");
        println!("{}", get_result(&answer));
        println!("You win!");
    }
}

pub fn getWord(index: i32) -> String {
    let mut f = File::open("src/words.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let vec = contents.split("\n").collect::<Vec<&str>>();
    return String::from(vec[index as usize]);
}

fn get_game_answer() -> Vec<Letter> {
    let word = getWord(rand::thread_rng().gen_range(1..855));
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

fn get_result(answer: &Vec<Letter>) -> String {
    let mut result = String::new();
    for letter in answer{
        result.push(letter.letter);
    }
    return result;
}

fn get_user_answer(answer: &Vec<Letter>) -> String {
    let mut result = String::new();
    for letter in answer{
        if letter.revealed {
            result.push(letter.letter);
        }
        else{
            result.push('_');
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_user_answer(){
        let mut input_test:Vec<Letter> = vec![
            Letter{
                letter: 'h',
                revealed: true
            },
            Letter{
                letter: 'i',
                revealed: true
            }
        ];
        assert_eq!(get_user_answer(&input_test),"hi");
        input_test[1].revealed = false;
        assert_eq!(get_user_answer(&input_test),"h_");
    }

    #[test]
    fn test_get_result(){
        let mut input_test:Vec<Letter> = vec![
            Letter{
                letter: 'h',
                revealed: true
            },
            Letter{
                letter: 'i',
                revealed: true
            }
        ];
        assert_eq!(get_result(&input_test),"hi");
        input_test[1].revealed = false;
        assert_eq!(get_result(&input_test),"hi");
    }
}


fn read_user_input_character() -> char {
    let mut user_input = String::new();
    return match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { c }
                None => { '*' }
            }
        }
        Err(_) => { '*' }
    }
}