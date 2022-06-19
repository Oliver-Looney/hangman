use std::io;
use rand::Rng; // 0.8.0

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
        // if its not inrement lives
        if !change {
            lives = lives + 1;
        }
    }

    if lives == hangman_pics.len()-1{
        println!("You lose!");
        println!("The word you couldn't get was {}", get_result(&answer) );
        println!("{}", hangman_pics[hangman_pics.len()-1]);
    } else {
        println!("You win!");
        println!("
            +---+
            |   |
                |
       \\O/      |
        |       |
       / \\      |
    =============")
    }

}

fn get_game_answer() -> Vec<Letter> {
    let words = ["goats", "moats", "toads", "bacon", "jazz", "turds", "abruptly", "matrix","xylophone", "flyby", "food"];
    let word = words[rand::thread_rng().gen_range(0..words.len()-1)];
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
