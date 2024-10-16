use std::collections::{HashMap, HashSet};
use crate::participant::Participant;
use crate::utils::prompt;

fn init_colour_hashmap(color_map: &mut HashMap<String, String>) {
    color_map.insert("red".to_string(), "\x1B[31m".to_string());
    color_map.insert("blue".to_string(), "\x1B[34m".to_string());
    color_map.insert("green".to_string(), "\x1B[32m".to_string());
    color_map.insert("yellow".to_string(), "\x1B[33m".to_string());
    color_map.insert("magenta".to_string(), "\x1B[35m".to_string());
    color_map.insert("cyan".to_string(), "\x1B[36m".to_string());
    color_map.insert("".to_string(), "".to_string());
}

pub(crate) fn get_players(
    continue_choice_char: char, stored_players: Option<Vec<Participant>>
) -> (Vec<Participant>, Option<Vec<Participant>>) {
    if continue_choice_char == '3' {
        if stored_players.clone().is_none() {
            println!("Cannot Retrieve Stored Players");
            let players= read_player_count();
            return (players.clone(), Option::from(players));
        }
        return (stored_players.clone().unwrap(), stored_players);
    }
    let players= read_player_count();
    (players.clone(), Option::from(players))
}

fn read_player_count() -> Vec<Participant> {
    let mut player_count: u8 = 0;
    while player_count <= 1 {
        let player_count_str: String = prompt(&("How Many Players? ".to_string()));
        player_count = player_count_str.trim().parse().unwrap_or_else(|_| {
            println!("Cannot Parse The Count. Please Type A Valid Non-Zero Positive Number");
            0
        });
    }
    let mut players: Vec<Participant> = Vec::new();
    let mut symbols: HashSet<char> = HashSet::new();
    let mut color_name_to_ansii: HashMap<String, String> = HashMap::new();
    init_colour_hashmap(&mut color_name_to_ansii);
    while symbols.len() < player_count as usize {
        let input_symbol: char = prompt(&(format!("Player {}# Symbol? ", symbols.len() + 1)))
            .trim()
            .parse()
            .unwrap_or_else(|_| {
                println!("Cannot Parse This Symbol. Please Type A Valid Character");
                return '\0';
            });
        match input_symbol {
            '\0' => { continue; }
            ' ' => {
                println!("Cannot input an empty symbol to use!");
                continue;
            }
            _ => {}
        }
        if symbols.contains(&input_symbol) {
            println!("A Player has already assigned this symbol!");
            continue;
        }
        let mut input_colour: String = prompt(&(
            format!("Player {}# Colour? ", symbols.len() + 1))
        ).trim().parse().unwrap_or_else(|_| {
            println!("Cannot Parse This Colour String. Please Type A Valid Colour String!");
            return "".to_string();
        });
        input_colour = input_colour.trim().to_lowercase();
        if !color_name_to_ansii.contains_key(&input_colour) {
            println!("The Colour Does Not Exist!");
            continue;
        }
        let is_bot_char: char = prompt(&(
            format!("Player {}# Computer(Y) Or Human(N)? ", symbols.len() + 1))
        ).trim().parse().unwrap_or_else(|_| {
            '\0'
        });
        if  is_bot_char.to_lowercase().to_string() != "n" && is_bot_char.to_lowercase().to_string() != "y" {
            println!("Please Enter Yes Or No As (Y/N)!");
            continue;
        }
        let is_bot: bool = is_bot_char.to_lowercase().to_string() == "y";
        input_colour = "\x1B[1m".to_string() + &*color_name_to_ansii[&input_colour];
        symbols.insert(input_symbol);
        players.push(Participant{
            symbol: input_symbol, colour: input_colour, is_bot
        });
    }
    players
}