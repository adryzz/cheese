use cheese_game::{Board, game::ChessProcessor};

fn main() {
    let board = Board::new();
    let formatted = cheese_game::format_utils::format_board_fancy(&board);
    println!("chess board");
    print!("{}", formatted);

    let moves = ChessProcessor::get_all_moves(&board, cheese_game::Player::White);

    for this in moves {
        println!("moving a {:?} from {} to {}", this.piece, this.src, this.dst);
    }
}
