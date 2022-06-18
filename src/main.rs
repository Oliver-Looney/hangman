use std::io;

struct Letter {
    letter: char,
    revealed: bool
}

fn main() {
    // let words = ["cheese", "food", "test"];
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
    let answer = [Letter { letter: 'f', revealed: false},
        Letter { letter: 'o', revealed: true},
        Letter { letter: 'o', revealed: false},
        Letter { letter: 'd', revealed: false}];
    let guess = read_user_input_character();
    let user_answer = get_user_answer(&answer);
    println!("{}", user_answer);
    println!("{}", lives);

    println!("This is hangman game!");
}


fn get_user_answer(answer: &[Letter]) -> String {
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
