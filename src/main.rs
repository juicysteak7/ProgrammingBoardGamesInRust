use chess::ChessMove;
use std::io;
use std::str::FromStr;
use my_chess::MyChess;

fn main() {
    let mut chess = MyChess::new();
    chess.print_board();
    while !chess.is_game_over() {
        // Read user input
        println!("Enter your move (e.g., e2e4):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse user input into a Move
        match ChessMove::from_str(input.trim()) {
            Ok(chess_move) => {
                // Apply the move to the board
                chess.make_move(chess_move);
                chess.print_board();
            }
            Err(_) => {
                println!("Invalid move. Please enter a move in UCI format (e.g., e2e4).");
            }
        }
        println!("AI is thinking....");
        chess.make_ai_move();
        chess.print_board();
    }
}
