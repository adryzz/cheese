use crate::{PieceType, Board, Cell, game::{Move, AdditionalMoveData::Promotion}, Player, utils};

use super::board_add;

pub fn append_all_moves(board: &Board, src: Cell, moves: &mut Vec<Move>) {

    let owner = match board[src] {
        Some(p) => {
            if p.piece != PieceType::Pawn {
                return;
            }

            p.owner
        }
        None => return
    };

    let dir = match owner {
        Player::White => 1,
        Player::Black => -1,
    };

    // forward

    let (row, col) = src.to_row_col();

    if let Some(v) = board_add(row as isize, dir) {
        let piece = board[v][col];

        if piece.is_none() {
            let dst = Cell::from_row_col(v, col);

            let is_promotion = v == 7 && owner == Player::White || v == 0 && owner == Player::Black;

            if is_promotion {
                for piece in utils::ALL_PROMOTABLE_PIECES {
                    let this = Move {
                        src,
                        dst,
                        piece: PieceType::Pawn,
                        capture: None,
                        extra: Some(Promotion(piece))
                    };
                    moves.push(this);
                }
            } else {
                let this = Move {
                    src,
                    dst,
                    piece: PieceType::Pawn,
                    capture: None,
                    extra: None
                };

                moves.push(this);
            }
        }
    }

    // capture

    // en pissant
}