use crate::{game::Move, Board, Cell, PieceType, Piece};

use super::board_add;

pub fn append_all_moves(board: &Board, src: Cell, moves: &mut Vec<Move>) {
    let piece = match board[src] {
        Some(p) => p,
        None => return,
    };

    let straight_moves = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    let diag_moves = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let all_moves = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    match piece.piece {
        PieceType::Rook => append_moves(board, src, moves, piece, &straight_moves),
        PieceType::Bishop => append_moves(board, src, moves, piece, &diag_moves),
        PieceType::Queen | PieceType::King => append_moves(board, src, moves, piece, &all_moves),
        _ => {}
    }
}

fn append_moves(
    board: &Board,
    src: Cell,
    moves: &mut Vec<Move>,
    piece: Piece,
    patterns: &[(isize, isize)],
) {
    let (row, col) = src.to_row_col();



    let depth = if piece.piece == PieceType::King {1} else {7};


    for current in patterns {
        for i in 0..depth {
            let new = (current.0 * i, current.1 * i);
            let nrow = board_add(row as isize, new.0);
            let ncol = board_add(col as isize, new.1);

            if let (Some(r), Some(c)) = (nrow, ncol) {
                let dst = Cell::from_row_col(r, c);
                let capture = board[r][c];

                let this = Move {
                    src,
                    dst,
                    piece: piece.piece,
                    capture,
                    extra: None
                };
                
                if let Some(p) = capture {
                    if p.owner != piece.owner {
                        moves.push(this);
                    }

                    break;
                }
                moves.push(this);
            }
        }
    }
}
