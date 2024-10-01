use crate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Active,
    Checkmate,
    Stalemate,
    FiftyMoveRule,
    ThreefoldRepetition,
}

pub fn str_to_idx(s: &str) -> usize {
    let s = s.to_lowercase();
    let x = s.chars().next().unwrap() as usize - 'a' as usize;
    let y = s.chars().nth(1).unwrap() as usize - '1' as usize;
    (7 - y) * 8 + x
}

pub fn idx_to_str(idx: usize) -> String {
    let x = idx % 8;
    let y = 7 - idx / 8;
    let x = (x as u8 + b'a') as char;
    let y = (y as u8 + b'1') as char;
    format!("{}{}", x, y)
}

pub fn generate_valid_moves(board: &mut Board) -> [Vec<usize>; 64] {
    const ARRAY_REPEAT_VALUE: Vec<usize> = Vec::new();
    let mut valid_moves = [ARRAY_REPEAT_VALUE; 64];

    let moves = filtered_moves(board);

    for m in moves.iter() {
        let idx = str_to_idx(&m[0..2]);
        valid_moves[idx].push(str_to_idx(&m[2..4]));
    }

    valid_moves
}

pub fn get_piece_color(piece: char) -> Color {
    match piece {
        'p' | 'r' | 'n' | 'b' | 'q' | 'k' => Color::White,
        'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => Color::Black,
        _ => Color::None,
    }
}

pub fn move_piece(board: &mut Board, from: usize, to: usize) {
    let from = idx_to_str(from);
    let to = idx_to_str(to);

    let movi = format!("{}{}", from, to);

    println!("Move: {}", movi);

    let before_turn = board.start;
    make_move(board, movi);

    if before_turn == board.start {
        let movi = format!("{}{}Q", from, to);
        make_move(board, movi);
    }
}

pub fn invert_boardstr(boardstr: String) -> String {
    //reverse every 8 characters
    let mut new_boardstr = String::new();
    for i in (0..64).rev().step_by(8) {
        let row = &boardstr[i - 7..i + 1];
        new_boardstr.push_str(row);
    }
    new_boardstr
}
