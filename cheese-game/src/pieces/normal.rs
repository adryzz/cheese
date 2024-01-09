use crate::{
    cells,
    game::{AdditionalMoveData::LongCastle, AdditionalMoveData::ShortCastle, Move},
    Board, Cell, Piece, PieceType,
};

use super::board_add;

pub fn append_all_moves(board: &Board, src: Cell, moves: &mut Vec<Move>) {
    let piece = match board[src] {
        Some(p) => p,
        None => return,
    };

    let straight_moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let diag_moves = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

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

    let depth = if piece.piece == PieceType::King { 1 } else { 7 };

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
                    extra: None,
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

    // castling
    if piece.piece != PieceType::King {
        return;
    }

    match piece.owner {
        crate::Player::White => {
            if src != cells::E1 {
                return;
            }

            match (
                board[cells::A1],
                board[cells::B1],
                board[cells::C1],
                board[cells::D1],
            ) {
                (Some(Piece { owner: _, piece }), None, None, None) if piece == PieceType::Rook => {
                    let this = Move {
                        src,
                        dst: cells::C1,
                        piece: PieceType::King,
                        capture: None,
                        extra: Some(LongCastle),
                    };
                    moves.push(this);
                }
                _ => {}
            }

            // short castle
            match (board[cells::F1], board[cells::G1], board[cells::H1]) {
                (None, None, Some(Piece { owner: _, piece })) if piece == PieceType::Rook => {
                    let this = Move {
                        src,
                        dst: cells::C1,
                        piece: PieceType::King,
                        capture: None,
                        extra: Some(ShortCastle),
                    };
                    moves.push(this);
                }
                _ => {}
            }
        }
        crate::Player::Black => {
            if src != cells::E8 {
                return;
            }

            match (
                board[cells::A8],
                board[cells::B8],
                board[cells::C8],
                board[cells::D8],
            ) {
                (Some(Piece { owner: _, piece }), None, None, None) if piece == PieceType::Rook => {
                    let this = Move {
                        src,
                        dst: cells::C8,
                        piece: PieceType::King,
                        capture: None,
                        extra: Some(LongCastle),
                    };
                    moves.push(this);
                }
                _ => {}
            }

            // short castle
            match (board[cells::F8], board[cells::G8], board[cells::H8]) {
                (None, None, Some(Piece { owner: _, piece })) if piece == PieceType::Rook => {
                    let this = Move {
                        src,
                        dst: cells::C8,
                        piece: PieceType::King,
                        capture: None,
                        extra: Some(ShortCastle),
                    };
                    moves.push(this);
                }
                _ => {}
            }
        }
    }
}
