use cheese_game::{cells, game::ChessProcessor, Board};

fn main() {
    let mut board = Board::new();
    board[cells::F1] = None;
    board[cells::G1] = None;
    board[cells::H1] = None;

    let formatted = cheese_game::format_utils::format_board_fancy(&board);
    println!("chess board");
    print!("{}", formatted);

    let moves = ChessProcessor::get_all_moves(&board, cheese_game::Player::White);

    for this in moves {
        println!(
            "moving a {:?} from {} to {}",
            this.piece, this.src, this.dst
        );
    }
}
