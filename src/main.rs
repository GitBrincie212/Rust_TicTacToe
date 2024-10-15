use std::collections::{HashMap, HashSet};
use std::io::{stdout, Write};

#[derive(Clone, Debug)]
struct Player {
    symbol: char,
    colour: String
}

fn read_player_count() -> Vec<Player> {
    let mut player_count: u8 = 0;
    while player_count <= 1 {
        let player_count_str: String = prompt(&("How Many Players? ".to_string()));
        player_count = player_count_str.trim().parse().unwrap();
    }
    let mut players: Vec<Player> = Vec::new();
    let mut symbols: HashSet<char> = HashSet::new();
    let mut color_name_to_ansii: HashMap<String, String> = HashMap::new();
    color_name_to_ansii.insert("red".to_string(), "\x1B[31m".to_string());
    color_name_to_ansii.insert("blue".to_string(), "\x1B[34m".to_string());
    color_name_to_ansii.insert("green".to_string(), "\x1B[32m".to_string());
    color_name_to_ansii.insert("yellow".to_string(), "\x1B[33m".to_string());
    color_name_to_ansii.insert("magenta".to_string(), "\x1B[35m".to_string());
    color_name_to_ansii.insert("cyan".to_string(), "\x1B[36m".to_string());
    color_name_to_ansii.insert("".to_string(), "".to_string());
    while symbols.len() < player_count as usize {
        let input_symbol: char = prompt(&(
            format!("Player {}# Symbol? ", symbols.len() + 1))
        ).trim().parse().unwrap();
        if symbols.contains(&input_symbol) {
            println!("A Player has already assigned this symbol!");
            continue;
        }
        let mut input_colour: String = prompt(&(
            format!("Player {}# Colour? ", symbols.len() + 1))
        ).trim().parse().unwrap();
        input_colour = input_colour.trim().to_lowercase();
        if !color_name_to_ansii.contains_key(&input_colour) {
            println!("The Colour Does Not Exist!");
            continue;
        }
        input_colour = "\x1B[1m".to_string() + &*color_name_to_ansii[&input_colour];
        symbols.insert(input_symbol);
        players.push(Player{symbol: input_symbol, colour: input_colour});
    }
    players
}

fn display_board(board: &Vec<Vec<Player>>, players: Vec<Player>, winning_index: i8, colour_func: fn(&Player, &Vec<Player>, i8) -> String) {
    let mut output: String = "".to_string();
    let vertical_sep: String = "\n--+---+--\n".to_string();
    for column in 0..3{
        for row in 0..3 {
            let element = &board[column as usize][row as usize];
            output += &format!("{}{}\x1b[0m",colour_func(element, &players, winning_index), element.symbol);
            if row != 2 {
                output += " | ";
            }
        }
        if column != 2 {
            output += &vertical_sep;
        }
    }
    println!("{}\n", output);
}

fn append_to_board(board: &mut Vec<Vec<Player>>, position: u8, players_list: &Vec<Player>, index: u8) -> bool {
    let index_width =  position % 3;
    let index_height =  position / 3;
    if board[index_height as usize][index_width as usize].symbol != ' ' {
        println!("Cannot append to this occupied position");
        return false;
    }
    board[index_height as usize][index_width as usize] = players_list[index as usize].clone();
    true
}

fn check_row_winning_position(board: Vec<Vec<Player>>, player: &Player) -> bool {
    for row in board {
        if row.iter().filter(|iter_player| iter_player.symbol == player.symbol && iter_player.symbol != ' ').count() == 3 {
            return true;
        }
    }
    false
}

fn check_column_winning_position(board: Vec<Vec<Player>>, player: &Player) -> bool {
    for index in 0..2 {
        let mut result = true;
        for row in &board {
            if row[index].symbol == ' ' || row[index].symbol != player.symbol {
                result = false;
                break;
            }
        }
        if result { return true; }
    }
    false
}

fn check_diagonal_left_winning_position(board: &Vec<Vec<Player>>, player: &Player) -> bool {
    for (index, row) in board.iter().enumerate() {
        if row[index].symbol != player.symbol || row[index].symbol == ' ' {
            return false;
        }
    }
    true
}

fn check_diagonal_right_winning_position(board: Vec<Vec<Player>>, player: &Player) -> bool {
    for (index, row) in board.iter().enumerate() {
        if row[2 - index].symbol != player.symbol || row[2 - index].symbol == ' ' {
            return false;
        }
    }
    true
}

fn check_diagonal_winning_position(board: Vec<Vec<Player>>, player: &Player) -> bool {
    check_diagonal_left_winning_position(&board, player)
        || check_diagonal_right_winning_position(board.clone(), player)
}

fn check_winning_condition(board: &mut Vec<Vec<Player>>, players: &Vec<Player>) -> i8 {
    for (player_index, player) in players.iter().enumerate() {
        let row_result = check_row_winning_position((**board).to_owned(), &player);
        let column_result = check_column_winning_position((**board).to_owned(), &player);
        let diagonals_result = check_diagonal_winning_position((**board).to_owned(), &player);
        if row_result || column_result || diagonals_result {
            return player_index as i8;
        }
    }
    -1
}

fn main() {
    let mut continue_choice: bool = true;
    while continue_choice {
        let players: Vec<Player> = read_player_count();
        let player_count: u8 = players.len() as u8;

        let mut winning_index: i8 = -1;
        let mut curr_player_index: u8 = 0;
        let mut board = vec!(vec!(Player{symbol: ' ', colour: "".to_string()}; 3); 3);
        while winning_index == -1 {
            print!("\x1B[2J\x1B[1;1H");
            display_board(&board, players.clone(), winning_index, |player, _, _| {
                player.clone().colour
            });
            let mut exit_code = false;
            while !exit_code {
                let position_str: String = prompt(
                    &(format!("Pick A Position Player {} >> ", players[curr_player_index as usize].symbol).to_string())
                );
                let position: u8 = position_str.trim().parse().expect("Failed to parse position");
                if position >= 9 {
                    println!("Position is out of bounds");
                    continue;
                }
                exit_code = append_to_board(&mut board, position, &players, curr_player_index);
            }
            winning_index = check_winning_condition(&mut board, &players);

            curr_player_index += 1;
            curr_player_index = curr_player_index % player_count;
        }
        print!("\x1B[2J\x1B[1;1H");
        display_board(&board, players.clone(), winning_index, |player, players, win_index| {
            for _ in 0..players.len() {
                if player.symbol == players[win_index as usize].symbol {
                    return "\x1B[1m\x1B[32m".to_string();
                }
            }
            "\x1B[0m".to_string()
        });
        println!("Player #{} Has Won The Game", curr_player_index);
        let mut continue_choice_char: char = ' ';
        while continue_choice_char != 'Y' && continue_choice_char != 'N' {
            let continue_choice_str =  prompt(&"Would You Like To Continue(Y/N)? ".to_string());
            continue_choice_char = continue_choice_str.trim().parse().unwrap();
        }
        continue_choice = continue_choice_char == 'Y';
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn prompt(input_str: &String) -> String {
    use std::io::{stdin};

    let mut input = String::new();
    print!("{}", input_str);
    let _ = stdout().flush();
    stdin().read_line(&mut input).unwrap();
    input
}