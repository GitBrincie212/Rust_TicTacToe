use crate::participant::Participant;

fn check_row_winning_position(
    board: Vec<Vec<Participant>>, player: &Participant
) -> Vec<Vec<i8>> {
    let scale_value = board.len();
    for (col_index, row) in board.iter().enumerate() {
        let iter = row.iter();
        let filtered_iter = iter.filter(|iter_player|
            iter_player.symbol == player.symbol && iter_player.symbol != ' '
        );
        if filtered_iter.count() == scale_value {
            let column_index = col_index as i8;
            let mut result = vec![];
            for i in 0..(scale_value as i8) {
                result.push(vec![column_index, i])
            }
            return result;
        }
    }
    vec![]
}

fn check_column_winning_position(
    board: Vec<Vec<Participant>>, player: &Participant
) -> Vec<Vec<i8>> {
    let mut result: Vec<Vec<i8>> = vec![];
    let scale_value_minus_one = board.len() - 1;
    for index in 0..scale_value_minus_one {
        for (col_index, row) in board.iter().enumerate() {
            if row[index].symbol == ' ' || row[index].symbol != player.symbol {
                return vec![];
            }
            result.push(vec![col_index as i8, index as i8])
        }
        if result.len() == 3 {
            return result
        }
    }
    result
}

fn check_diagonal_left_winning_position(
    board: &Vec<Vec<Participant>>, player: &Participant
) -> Vec<Vec<i8>> {
    let mut vector_result: Vec<Vec<i8>> = vec![];
    let mut col_index: i8 = 0;
    for (index, row) in board.iter().enumerate() {
        if row[index].symbol != player.symbol || row[index].symbol == ' ' {
            return vec![];
        }
        vector_result.push(vec![col_index, index as i8]);
        col_index += 1;
    }
    vector_result
}

fn check_diagonal_right_winning_position(
    board: Vec<Vec<Participant>>, player: &Participant
) -> Vec<Vec<i8>> {
    let mut vector_result: Vec<Vec<i8>> = vec![];
    let mut col_index: i8 = 0;
    let scale_value_minus_one = board.len() - 1;
    for (index, row) in board.iter().enumerate() {
        if row[scale_value_minus_one - index].symbol != player.symbol || row[scale_value_minus_one - index].symbol == ' ' {
            return vec![];
        }
        vector_result.push(vec![col_index, (scale_value_minus_one - index) as i8]);
        col_index += 1;
    }
    vector_result
}

fn check_diagonal_winning_position(
    board: Vec<Vec<Participant>>, player: &Participant
) -> Vec<Vec<i8>> {
    let left = check_diagonal_left_winning_position(&board, player);
    let right = check_diagonal_right_winning_position(board.clone(), player);
    let mut result = vec![];
    if  left.len() != 0 {
        result.append(&mut left.clone());
    } else if right.len() != 0 {
        result.append(&mut right.clone());
    }
    result
}

pub(crate) fn check_winning_condition(
    board: &mut Vec<Vec<Participant>>, players: &Vec<Participant>
) -> (i8, Vec<Vec<i8>>) {
    let mut found_unoccupied_space = false;
    'row_loop: for row in &mut *board {
        for entry in row {
            if entry.symbol == ' ' {
                found_unoccupied_space = true;
                break 'row_loop;
            }
        }
    }
    for (player_index, player) in players.iter().enumerate() {
        let mut row_result = check_row_winning_position((**board).to_owned(), &player);
        let mut column_result = check_column_winning_position((**board).to_owned(), &player);
        let mut diagonals_result = check_diagonal_winning_position((**board).to_owned(), &player);
        row_result.append(&mut column_result);
        row_result.append(&mut diagonals_result);
        if row_result.len() > 0 {
            return (player_index as i8, row_result);
        }
    }
    if found_unoccupied_space {
        return (-1, vec![]);
    }
    (-2, vec![])
}