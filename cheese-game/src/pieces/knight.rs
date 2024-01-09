use crate::{game::Move, Board, Cell, PieceType};

use super::board_add;

pub fn append_all_moves(board: &Board, src: Cell, moves: &mut Vec<Move>) {
    let owner = match board[src] {
        Some(p) => {
            if p.piece != PieceType::Knight {
                return;
            }

            p.owner
        }
        None => return,
    };

    let cells = move_dir(src);

    for c in cells {
        if let Some(dst) = c {
            let capture = board[dst];

            if let Some(c) = capture {
                if c.owner == owner {
                    continue;
                }
            }

            let this = Move {
                piece: PieceType::Knight,
                src,
                dst,
                capture,
                extra: None,
            };
            moves.push(this);
        }
    }
}

fn move_dir(pos: Cell) -> [Option<Cell>; 8] {
    let pos = pos.to_row_col();

    let mut ret = [None; 8];
    let moves = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    for i in 0..8 {
        let new_x = board_add(pos.0 as isize, moves[i].0);
        let new_y = board_add(pos.1 as isize, moves[i].1);

        match (new_x, new_y) {
            (Some(x), Some(y)) => ret[i] = Some(Cell::from_row_col(x, y)),
            _ => {}
        }
    }

    ret
}
