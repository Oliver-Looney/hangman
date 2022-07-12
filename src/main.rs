mod utils;

use std::io;

fn main() {
    let hangman_pics = utils::pre_game_set_up::get_drawings();
    let mut lives = 0;
    let mut answer = utils::pre_game_set_up::get_game_answer();

    println!("This is hangman game!");
    let mut flag = false;
    while !flag & (lives<hangman_pics.len()-1){
        // print start & blanks
        println!("{}", hangman_pics[lives]);
        println!("{}", utils::get_user_answer(&answer));
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
        println!("The word you couldn't get was {}", utils::get_result(&answer) );
    } else {
        println!("
            +---+
            |   |
                |
       \\O/      |
        |       |
       / \\      |
    =============");
        println!("{}", utils::get_result(&answer));
        println!("You win!");
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