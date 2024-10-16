use crate::board_utils::read_position;
use crate::participant::Participant;

mod win_check;
mod game_init;
mod board_utils;
mod utils;
mod participant;

fn main() {
    let mut continue_choice: bool = true;
    while continue_choice {
        let players: Vec<Participant> = game_init::read_player_count();
        let player_count: u8 = players.len() as u8;

        let mut winning_index: i8 = -1;
        let mut winning_highlights: Vec<Vec<i8>> = vec![];
        let mut curr_player_index: u8 = 0;
        let player_empty: Participant = Participant{
            symbol: ' ', colour: "".to_string(), is_bot: false
        };
        let scale_value: usize = 3;
        let mut board = vec!(vec!(player_empty; scale_value); scale_value);
        while winning_index == -1 {
            print!("\x1B[2J\x1B[1;1H");
            board_utils::display_board(
                &board,
                (winning_index, winning_highlights),
                |player, _, _| {
                player.clone().colour
            });
            read_position(&mut board, players.clone(), curr_player_index);
            (winning_index, winning_highlights) = win_check::check_winning_condition(&mut board, &players);

            curr_player_index += 1;
            curr_player_index = curr_player_index % player_count;
        }
        print!("\x1B[2J\x1B[1;1H");
        /*
        When winning index is -2 it means the game ended
        in a draw I know I could use a struct for that
         */
        if winning_index != -2 {
            board_utils::display_board(
                &board.clone(),
                (winning_index, winning_highlights),
                |_, win_info, position | {
                    if win_info.1.contains(&vec![position.0, position.1]) {
                        return "\x1B[1m\x1B[32m".to_string();
                    }
                    "\x1B[2m".to_string()
                });
            println!("Player #{} Has Won The Game", winning_index);
        } else {
            board_utils::display_board(
                &board.clone(),
                (winning_index, winning_highlights),
                |_, _, _ | {
                    "\x1B[2m".to_string()
                });
            println!("The Game Ended In A Draw. No one won");
        }
        let mut continue_choice_char: char = ' ';
        while continue_choice_char != 'Y' && continue_choice_char != 'N' {
            let continue_choice_str = utils::prompt(&"Would You Like To Continue(Y/N)? ".to_string());
            continue_choice_char = continue_choice_str.trim().to_uppercase().parse().unwrap();
        }
        continue_choice = continue_choice_char == 'Y';
        print!("\x1B[2J\x1B[1;1H");
    }
}