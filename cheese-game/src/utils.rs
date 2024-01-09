use std::ops::BitXor;

use crate::{
    Board, Cell, Piece,
    PieceType::{self, Bishop, King, Knight, Pawn, Queen, Rook},
    Player::{Black, White},
    Row, EMPTY_ROW,
};

const WHITE_FIRST: Row = Row([
    Some(Piece::new(White, Rook)),
    Some(Piece::new(White, Knight)),
    Some(Piece::new(White, Bishop)),
    Some(Piece::new(White, Queen)),
    Some(Piece::new(White, King)),
    Some(Piece::new(White, Bishop)),
    Some(Piece::new(White, Knight)),
    Some(Piece::new(White, Rook)),
]);

const WHITE_SECOND: Row = Row([Some(Piece::new(White, Pawn)); 8]);

const BLACK_FIRST: Row = Row([
    Some(Piece::new(Black, Rook)),
    Some(Piece::new(Black, Knight)),
    Some(Piece::new(Black, Bishop)),
    Some(Piece::new(Black, Queen)),
    Some(Piece::new(Black, King)),
    Some(Piece::new(Black, Bishop)),
    Some(Piece::new(Black, Knight)),
    Some(Piece::new(Black, Rook)),
]);

const BLACK_SECOND: Row = Row([Some(Piece::new(Black, Pawn)); 8]);

pub fn init_board() -> Board {
    Board([
        WHITE_FIRST,
        WHITE_SECOND,
        EMPTY_ROW,
        EMPTY_ROW,
        EMPTY_ROW,
        EMPTY_ROW,
        BLACK_SECOND,
        BLACK_FIRST,
    ])
}

pub const ALL_PIECES: [PieceType; 6] = [King, Queen, Rook, Knight, Bishop, Pawn];

pub const ALL_PROMOTABLE_PIECES: [PieceType; 5] = [King, Queen, Rook, Knight, Bishop];

pub fn is_square_colored(row: usize, col: usize) -> bool {
    (col % 2 == 0).bitxor(row % 2 == 0)
}
