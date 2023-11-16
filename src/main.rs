use Chess::Chess;
use chess::{ ChessMove, Square };

fn main() {
    let mut chess = Chess::new();
    chess.print_board();
    while !chess.is_game_over() {
        chess.make_ai_move();
        chess.print_board();
    }
}