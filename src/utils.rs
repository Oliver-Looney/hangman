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