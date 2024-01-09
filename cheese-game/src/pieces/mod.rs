pub mod knight;
pub mod normal;
pub mod pawn;

fn board_add(pos: isize, rhs: isize) -> Option<usize> {
    let res = pos + rhs;
    if res > 7 || res < 0 || res == pos {
        return None;
    }

    Some(res as usize)
}
