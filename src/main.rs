mod utils;
use utils::pre_game_set_up;

fn main() {
    let hangman_pics = pre_game_set_up::get_drawings();
    let mut lives = 0;
    let mut answer = pre_game_set_up::get_game_answer();

    println!("This is hangman game!");
    let mut flag = false;
    while !flag & (lives<hangman_pics.len()-1){
        // print start & blanks
        println!("{}", hangman_pics[lives]);
        println!("{}", utils::get_user_answer(&answer));
        let guess = utils::read_user_input_character();
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

    utils::calc_and_output_result(
        hangman_pics.len(),
        lives,
        answer
    );
}