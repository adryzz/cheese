use cheese_game::Board;

fn main() {
    let board = Board::new();
    let formatted = cheese_game::format_utils::format_board_fancy(&board);
    println!("chess board");
    print!("{}", formatted);
}
