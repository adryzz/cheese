use crate::{Cell, Piece, PieceType};


pub struct ChessProcessor;

impl ChessProcessor {
    pub fn get_all_moves() -> Vec<Move> {
        vec![]
    }

    pub fn get_all_possible_moves() -> Vec<Move> {
        vec![]
    }
}

pub struct Move {
    pub src: Cell,
    pub dst: Cell,
    pub piece: Piece,
    pub capture: Option<Piece>,
    pub extra: Option<AdditionalMoveData>
}

pub enum AdditionalMoveData {
    LongCastle,
    ShortCastle,
    Promotion(PieceType)
}