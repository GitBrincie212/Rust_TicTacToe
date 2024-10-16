use crate::board_utils::read_position;
use crate::game_init::get_players;
use crate::participant::Participant;

mod win_check;
mod game_init;
mod board_utils;
mod utils;
mod participant;

fn main() {
    let mut continue_choice: bool = true;
    let mut continue_choice_char = '1';
    let mut stored_players: Option<Vec<Participant>> = None;
    while continue_choice {
        let players;
        (players, stored_players) = get_players(continue_choice_char, stored_players);
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
            clean_screen!();
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
        clean_screen!();
        /*
        When winning index is -2 it means the game ended
        in a draw I know I could use a struct for that
         */
        if winning_index != -2 {
            board_utils::display_board(
                &board.clone(),
                (winning_index, winning_highlights),
                |player, win_info, position | {
                    if win_info.1.contains(&vec![position.0, position.1]) {
                        return format!("\x1B[1m{}", player.colour);
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
        continue_choice_char = ' ';
        let menu: &String = &"Would You Like To Continue?\n1. New Game\n2. Abort Game\n3. Replay Game\n\n>> ".to_string();
        while continue_choice_char != '1' && continue_choice_char != '2' && continue_choice_char != '3' {
            let continue_choice_str = utils::prompt(menu);
            continue_choice_char = continue_choice_str.trim().to_uppercase().parse().unwrap_or_else(|_| {
                    println!("Cannot Select A Valid Number On The Menu");
                    clean_screen!();
                    return ' ';
                }
            )
        }
        continue_choice = continue_choice_char == '1' || continue_choice_char == '3';
        clean_screen!();
    }
}