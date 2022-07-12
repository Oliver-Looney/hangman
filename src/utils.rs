use std::io;

pub mod pre_game_set_up;

pub fn get_result(answer: &Vec<pre_game_set_up::Letter>) -> String {
    let mut result = String::new();
    for letter in answer{
        result.push(letter.letter);
    }
    return result;
}

pub fn get_user_answer(answer: &Vec<pre_game_set_up::Letter>) -> String {
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

pub fn calc_and_output_result(hangman_pics_length: usize, lives: usize, answer: Vec<pre_game_set_up::Letter>) {
    if lives == hangman_pics_length - 1 {
        println!("{}",
                 "
        +---+
        |   |
        O   |
       /|\\  |
       / \\  |
            |
    =========");
        println!("You lose!");
        println!("The word you couldn't get was {}", get_result(&answer));
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_user_answer(){
        let mut input_test:Vec<pre_game_set_up::Letter> = vec![
            pre_game_set_up::Letter{
                letter: 'h',
                revealed: true
            },
            pre_game_set_up::Letter{
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
        let mut input_test:Vec<pre_game_set_up::Letter> = vec![
            pre_game_set_up::Letter{
                letter: 'h',
                revealed: true
            },
            pre_game_set_up::Letter{
                letter: 'i',
                revealed: true
            }
        ];
        assert_eq!(get_result(&input_test),"hi");
        input_test[1].revealed = false;
        assert_eq!(get_result(&input_test),"hi");
    }
}

pub fn read_user_input_character() -> char {
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