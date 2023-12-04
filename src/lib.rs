use chess::{Board, ChessMove, Color, File, Game, GameResult, MoveGen, Piece, Rank, Square, EMPTY};


/// Represents a MyChess game.
///
/// This struct encapsulates the state of a MyChess game, including the MyChess board,
/// the current player's color, and the game rules.
pub struct MyChess {
    pub board: Board,
    pub color: Color,
    pub game: Game,
}

/// Creates a new `MyChess` instance with default settings.
///
/// This function is used to create a new MyChess game with the default starting position
/// and the starting player set to `Color::White`.
impl Default for MyChess {
    fn default() -> Self {
        Self::new()
    }
}

/// Logic for `MyChess` struct
impl MyChess {
    /// Creates a new instance of the `MyChess` struct.
    ///
    /// This function initializes a new MyChess game by creating a new instance of the `Game`
    /// struct and extracting the initial MyChess board, current player's color, and game state.
    pub fn new() -> Self {
        let game = Game::new();
        MyChess {
            board: game.current_position(),
            color: game.side_to_move(),
            game,
        }
    }

    /// Prints the current state of the MyChess board along with an evaluation score.
    ///
    /// This method prints the ASCII representation of the current MyChess board and includes
    /// an evaluation score based on the current game state. The evaluation score can provide
    /// insights into the relative strength of the position for the player whose turn it is.
    pub fn print_board(&self) {
        println!(
            "{} {}",
            self.board,
            Self::evaluate_board(&self.board, self.game.side_to_move())
        );
    }

    /// Starts a new MyChess game, resetting the board and game state.
    ///
    /// This method creates a new instance of the `Game` struct, initializes a new MyChess game,
    /// and resets the internal state of the `MyChess` struct with the initial MyChess board and
    /// starting player's color. The new `Game` instance is returned for further interactions.
    pub fn new_game(&mut self) -> Game {
        let game = Game::new();
        self.board = game.current_position();
        self.color = game.side_to_move();
        game
    }

    /// Makes a MyChess move, updating the game state and current board.
    ///
    /// This method applies the specified MyChess move to the current game state, updating both
    /// the internal `Game` state and the current MyChess board. After making the move, the method
    /// also updates the current player's color based on the updated game state.
    ///
    /// # Arguments
    ///
    /// * `mov` - The MyChess move to be made.
    pub fn make_move(&mut self, mov: ChessMove) -> bool {
        let result = self.game.make_move(mov);
        self.board = self.game.current_position();
        self.color = self.game.side_to_move();
        result
    }

    /// Makes a MyChess move in a new game instance, updating the game state.
    ///
    /// This function takes a `Game` instance, applies the specified MyChess move to it, and returns
    /// a new `Game` instance with the updated state. The original `Game` instance remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `game` - The original game instance.
    /// * `mov` - The MyChess move to be made.
    ///
    /// # Returns
    ///
    /// A new `Game` instance with the updated state after applying the MyChess move.
    pub fn make_move_r(mut game: Game, mov: ChessMove) -> Game {
        game.make_move(mov);
        game
    }

    /// Returns an iterator over legal MyChess moves for the current position.
    ///
    /// This method creates and returns a `MoveGen` iterator over legal MyChess moves for the
    /// current position on the MyChess board. The iterator provides a sequence of valid moves
    /// that can be made in the current game state.
    ///
    /// # Returns
    ///
    /// A `MoveGen` iterator providing legal MyChess moves for the current position.
    pub fn move_iterator(&self) -> MoveGen {
        MoveGen::new_legal(&self.board)
    }

    /// Makes a MyChess move using an AI strategy, updating the game state and current board.
    ///
    /// This method generates and evaluates possible MyChess moves using a simple AI strategy,
    /// considering capturing moves first and then non-capturing moves. The AI performs a
    /// limited-depth search to evaluate potential future positions and chooses the move
    /// with the highest evaluation score. The internal state of the `MyChess` struct is then
    /// updated with the chosen move.

    pub fn make_ai_move(&mut self) {
        let mut iter = self.move_iterator();
        let mut move_results: Vec<(ChessMove, i32)> = Vec::new();

        // lets iterate over targets.
        let targets = self.board.color_combined(!self.board.side_to_move());
        iter.set_iterator_mask(*targets);

        for mov in &mut iter {
            // This move captures one of my opponents pieces (with the exception of en passant)
            let mut new_game = self.game.clone();
            new_game.make_move(mov);
            let result = Self::make_ai_move_r(&new_game, 4, i32::MIN + 1, i32::MAX);
            move_results.push((mov, result));
        }

        // now, iterate over the rest of the moves
        iter.set_iterator_mask(!EMPTY);
        for mov in &mut iter {
            // This move does not capture anything
            let mut new_game = self.game.clone();
            new_game.make_move(mov);
            let result = Self::make_ai_move_r(&new_game, 4, i32::MIN + 1, i32::MAX);
            move_results.push((mov, result));
        }

        if !move_results.is_empty() {
            move_results.sort_by(|(_, res1), (_, res2)| res2.cmp(res1));
            self.game.make_move(move_results[0].0);
            self.board = self.game.current_position();
            self.color = self.game.side_to_move();
        }
    }

    /// Performs a recursive evaluation of a MyChess move using a minimax algorithm with alpha-beta pruning.
    ///
    /// This function evaluates the potential outcome of a MyChess move by recursively exploring possible future
    /// positions up to a specified depth using the minimax algorithm with alpha-beta pruning. It considers both
    /// capturing and non-capturing moves and returns an evaluation score for the move.
    ///
    /// # Arguments
    ///
    /// * `game` - The current game state.
    /// * `depth` - The remaining depth of the search tree.
    /// * `a` - The alpha value (lower bound) for alpha-beta pruning.
    /// * `b` - The beta value (upper bound) for alpha-beta pruning.
    ///
    /// # Returns
    ///
    /// An evaluation score for the specified MyChess move.
    pub fn make_ai_move_r(game: &Game, depth: u8, mut a: i32, b: i32) -> i32 {
        let mut result1: i32 = i32::MIN + 1;
        let mut result: i32;
        let board = game.current_position();
        let mut game_over: bool = false;
        if let Some(result) = game.result() {
            if result == GameResult::BlackCheckmates
                || result == GameResult::WhiteCheckmates
                || result == GameResult::Stalemate
            {
                game_over = true;
            }
        }
        // Terminal cases
        if depth == 0 || game_over {
            return Self::evaluate_board(&board, game.side_to_move());
        } else {
            let mut iter = MoveGen::new_legal(&board);
            let targets = board.color_combined(!board.side_to_move());
            iter.set_iterator_mask(*targets);

            for mov in &mut iter {
                // This move captures one of the opponents pieces (no en passant)
                let new_game = Self::make_move_r(game.clone(), mov);
                result = -Self::make_ai_move_r(&new_game, depth - 1, -b, -a);
                result1 = result1.max(result);
                a = a.max(result);
                if a >= b {
                    break;
                }
            }

            iter.set_iterator_mask(!EMPTY);
            for mov in &mut iter {
                // This move does not capture anything
                let new_game = Self::make_move_r(game.clone(), mov);
                result = -Self::make_ai_move_r(&new_game, depth - 1, -b, -a);
                result1 = result1.max(result);
                a = a.max(result);
                if a >= b {
                    break;
                }
            }
        }
        result1
    }

    /// Evaluates the current MyChess board position by calculating the material advantage for a specified color.
    ///
    /// This function calculates the material advantage for a given color based on the piece values on the
    /// MyChess board. It considers the values of pawns, knights, bishops, rooks, and queens for both players,
    /// and returns a numerical score representing the material advantage for the specified color.
    ///
    /// # Arguments
    ///
    /// * `board` - The current MyChess board position.
    /// * `color` - The color for which the material advantage is calculated.
    ///
    /// # Returns
    ///
    /// A numerical score representing the material advantage for the specified color.
    fn evaluate_board(board: &Board, color: Color) -> i32 {
        let mut white_pieces = 0;
        let mut black_pieces = 0;

        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::make_square(Rank::from_index(rank), File::from_index(file));
                if let Some(piece) = board.piece_on(square) {
                    match board.color_on(square) {
                        Some(Color::White) => white_pieces += Self::piece_value(&piece),
                        Some(Color::Black) => black_pieces += Self::piece_value(&piece),
                        None => {}
                    }
                }
            }
        }
        if color == Color::White {
            return white_pieces - black_pieces;
        }
        black_pieces - white_pieces
    }

    /// Retrieves the numerical value associated with a MyChess piece.
    ///
    /// This function returns the numerical value associated with a MyChess piece based on traditional piece values
    /// used for evaluation. The values assigned are: pawn (1), knight (3), bishop (3), rook (5), queen (9), and king (0).
    /// You may want to adjust the value for the king based on the game state or specific evaluation criteria.
    ///
    /// # Arguments
    ///
    /// * `piece` - The MyChess piece for which the numerical value is retrieved.
    ///
    /// # Returns
    ///
    /// The numerical value associated with the specified MyChess piece.
    ///
    fn piece_value(piece: &Piece) -> i32 {
        match piece {
            Piece::Pawn => 1,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 0, // You might want to adjust this based on the game state
        }
    }

    /// Checks if the current MyChess game has reached a game-over state.
    ///
    /// This function checks whether the current MyChess game has reached a game-over state, including scenarios
    /// such as checkmate or stalemate. It queries the game result, and if the result is checkmate (for either
    /// white or black) or stalemate, it returns true, indicating that the game is over.
    ///
    /// # Returns
    ///
    /// `true` if the game is over, `false` otherwise.
    pub fn is_game_over(&self) -> bool {
        let mut game_over: bool = false;
        if let Some(result) = self.game.result() {
            if result == GameResult::BlackCheckmates
                || result == GameResult::WhiteCheckmates
                || result == GameResult::Stalemate
            {
                game_over = true;
            }
        }
        game_over
    }
}