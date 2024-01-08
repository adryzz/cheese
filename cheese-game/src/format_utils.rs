use crate::{
    Board, Piece,
    PieceType::{Bishop, King, Knight, Pawn, Queen, Rook},
    Player::{Black, White},
    Row, EMPTY_ROW, Cell, utils,
};

pub fn format_board_ascii(board: &Board) -> String {
    let mut formatted = String::with_capacity(200);
    for (rowi, row) in board.0.iter().enumerate() {
        for col in 0..8 {
            match row[col] {
                Some(p) => {
                    let piece = format_ascii_piece(p);
                    formatted.push(' ');
                    formatted.push(piece[0]);
                    formatted.push(piece[1]);
                },
                None => {
                    formatted.push(' ');
                    formatted.push(' ');
                    if utils::is_square_colored(rowi, col) {
                        formatted.push('+');
                    } else {
                        formatted.push('-');
                    }
                }
            }
        }
        formatted.push('\n');
    }

    formatted
}

pub fn format_ascii_piece(piece: Piece) -> [char; 2] {
    let letter: char = match piece.piece {
        King => 'K',
        Queen => 'Q',
        Rook => 'R',
        Knight => 'N',
        Bishop => 'B',
        Pawn => 'P'
    };

    let owner_char = match piece.owner {
        White => 'w',
        Black => 'b'
    };

    [owner_char, letter]
}

pub fn format_board_fancy(board: &Board) -> String {
    let mut formatted = String::with_capacity(200);
    for (rowi, row) in board.0.iter().enumerate() {
        for col in 0..8 {
            if utils::is_square_colored(rowi, col) {
                formatted.push_str("\x1B[47m");
            } else {
                formatted.push_str("\x1B[30m");
            }
            match row[col] {
                Some(p) => {
                    let piece = format_fancy_piece(p);
                    formatted.push(' ');
                    formatted.push(piece);
                    formatted.push(' ');
                },
                None => {
                    formatted.push_str("   ");
                }
            }
            
            // reset colors
            formatted.push_str("\x1B[0m");
        }
        formatted.push('\n');
    }

    formatted
}

pub fn format_fancy_piece(piece: Piece) -> char {
    match (piece.piece, piece.owner) {
        (King, White) => '\u{2654}',
        (Queen, White) => '\u{2655}',
        (Rook, White) => '\u{2656}',
        (Bishop, White) => '\u{2657}',
        (Knight, White) => '\u{2658}',
        (Pawn, White) => '\u{2659}',

        (King, Black) => '\u{265A}',
        (Queen, Black) => '\u{265B}',
        (Rook, Black) => '\u{265C}',
        (Bishop, Black) => '\u{265D}',
        (Knight, Black) => '\u{265E}',
        (Pawn, Black) => '\u{265F}',
    }
}