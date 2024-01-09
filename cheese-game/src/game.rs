use crate::{
    pieces::{knight, normal, pawn},
    Board, Cell, Piece, PieceType, Player,
};

pub struct ChessProcessor;

impl ChessProcessor {
    pub fn get_all_moves(board: &Board, player: Player) -> Vec<Move> {
        let mut moves = Vec::new();

        for row in 0..8 {
            for col in 0..8 {
                let piece = board[row][col];
                if let Some(p) = piece {
                    if p.owner == player {
                        let cell = Cell::from_row_col(row, col);
                        knight::append_all_moves(board, cell, &mut moves);
                        pawn::append_all_moves(board, cell, &mut moves);
                        normal::append_all_moves(board, cell, &mut moves);
                    }
                }
            }
        }

        moves
    }

    pub fn get_all_possible_moves() -> Vec<Move> {
        vec![]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub src: Cell,
    pub dst: Cell,
    pub piece: PieceType,
    pub capture: Option<Piece>,
    pub extra: Option<AdditionalMoveData>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AdditionalMoveData {
    LongCastle,
    ShortCastle,
    Promotion(PieceType),
}
