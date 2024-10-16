use crate::utils;
use crate::participant::Participant;
use rand::{thread_rng, Rng};

pub(crate) fn display_board(
    board: &Vec<Vec<Participant>>,
    win_info: (i8, Vec<Vec<i8>>),
    colour_func: fn(&Participant, (i8, Vec<Vec<i8>>), (i8, i8)) -> String
) {
    let scale_value = board.len() as i8;
    let mut output: String = "┏━━━━━┳━━━━━┳━━━━━┓\n".to_string();
    let vertical_sep: String = "\n┣━━━━━╋━━━━━╋━━━━━┫\n".to_string();
    let mut index: i16 = 0;
    let start_border = "┃  ".to_string();
    let end_border = "  ┃".to_string();
    let middle_border = "  ┃  ".to_string();
    for column in 0..scale_value {
        output += start_border.as_str();
        for row in 0..scale_value {
            let element = &board[column as usize][row as usize];
            if row != 0 {
                output += middle_border.as_str();
            }
            let mut symbol: String = element.symbol.to_string();
            if symbol == " " && win_info.0 == -1 {
                symbol = format!("\x1B[2m{}", index.to_string().parse::<char>().unwrap())
            }
            output += &format!(
                "{}{}\x1b[0m",
                colour_func(element, win_info.clone(), (column, row)),
                symbol
            );
            index += 1
        }
        output += end_border.as_str();
        if column != 2 {
            output += &vertical_sep;
        }
    }
    println!(
        "{}\n┗━━━━━┻━━━━━┻━━━━━┛",
        output,
    );
}

#[allow(dead_code)]
pub(crate) fn append_to_board(
    board: &mut Vec<Vec<Participant>>,
    position: u8,
    players_list: &Vec<Participant>,
    index: u8
) -> bool {
    let index_width =  position % 3;
    let index_height =  position / 3;
    if board[index_height as usize][index_width as usize].symbol != ' ' {
        println!("Cannot append to this occupied position");
        return false;
    }
    board[index_height as usize][index_width as usize] = players_list[index as usize].clone();
    true
}

fn human_compute(_: &mut Vec<Vec<Participant>>, player: &Participant) -> (u8, bool) {
    let mut should_return = false;
    let position_str: String = utils::prompt(
        &(format!("Pick A Position Player {} >> ", player.symbol).to_string())
    );
    let position: u8 = position_str.trim().parse().unwrap_or_else(|_| {
        println!("Failed to parse position");
        should_return = true;
        0
    });
    if should_return {
        return (0, true);
    } else if position >= 9 {
        println!("Position is out of bounds");
        return (0, true);
    }
    (position, false)
}

fn bot_compute(board: &mut Vec<Vec<Participant>>, computer: &Participant) -> (u8, bool) {
    // todo: Add the actual logic to the computer participant
    (thread_rng().gen_range(0..9), false)
}

#[allow(dead_code)]
pub(crate) fn read_position(
    board: &mut Vec<Vec<Participant>>,
    players: Vec<Participant>,
    curr_player_index: u8
) {
    let mut exit_code = false;
    while !exit_code {
        let curr_player: &Participant = &players[curr_player_index as usize];
        let data: (u8, bool) = match curr_player.is_bot {
            true => {bot_compute(board, curr_player)}
            false => human_compute(board, curr_player)
        };
        if data.1 {
            continue;
        }
        exit_code = append_to_board(board, data.0, &players, curr_player_index);
    }
}