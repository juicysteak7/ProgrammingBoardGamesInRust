use chess::{ Board, Color, Game, GameResult,ChessMove, Square, Piece, Rank, File, MoveGen, EMPTY };

pub struct Chess { 
    pub board: Board,
    pub color: Color,
    pub game: Game,
}

impl Chess {
    pub fn new() -> Self {
        let game = Game::new();
        Chess {
            board: game.current_position(),
            color: game.side_to_move(),
            game,
        }
    }

    pub fn print_board(&self) {
        println!("{} {}", self.board, Self::evaluate_board(&self.board, self.game.side_to_move()));
    }

    pub fn new_game(&mut self) -> Game {
        let game = Game::new();
        self.board = game.current_position();
        self.color = game.side_to_move();
        game
    }

    pub fn make_move(&mut self, mov: ChessMove) {
        self.game.make_move(mov);
        self.board = self.game.current_position();
        self.color = self.game.side_to_move();
    }

    pub fn make_move_r(mut game: Game, mov: ChessMove) -> Game {
        game.make_move(mov);
        game
    }

    pub fn move_iterator(&self) -> MoveGen {
        MoveGen::new_legal(&self.board)
    }

    pub fn make_ai_move(&mut self) {
        let mut iter = self.move_iterator();
        let mut move_results: Vec<(ChessMove,i32)> = Vec::new();

        // lets iterate over targets.
        let targets = self.board.color_combined(!self.board.side_to_move());
        iter.set_iterator_mask(*targets);

        for mov in &mut iter {
            // This move captures one of my opponents pieces (with the exception of en passant)
            let mut new_game = self.game.clone();
            new_game.make_move(mov);
            let result = Self::make_ai_move_r(&new_game, 4, i32::MIN+1, i32::MAX);
            move_results.push((mov,result));
        }

        // now, iterate over the rest of the moves
        iter.set_iterator_mask(!EMPTY);
        for mov in &mut iter {
            // This move does not capture anything
            let mut new_game = self.game.clone();
            new_game.make_move(mov);
            let result = Self::make_ai_move_r(&new_game, 4, i32::MIN+1, i32::MAX);
            move_results.push((mov,result));
        }

        if move_results.len() > 0 {
            move_results.sort_by(|(_, res1), (_, res2)| res2.cmp(res1));
            self.game.make_move(move_results[0].0);
            self.board = self.game.current_position();
            self.color = self.game.side_to_move();
        }
    }

    pub fn make_ai_move_r(game: &Game, depth: u8, mut a: i32, b:i32) -> i32 {
        let mut result1:i32 = i32::MIN+1;
        let mut result: i32;
        let board = game.current_position();
        let mut game_over: bool = false;
        if let Some(result) = game.result() {
            if result == GameResult::BlackCheckmates || result == GameResult::WhiteCheckmates || result == GameResult::Stalemate {
                game_over = true;
            }
        }
        // Terminal cases 
        if depth == 0  || game_over {
            return Self::evaluate_board(&board, game.side_to_move());
        } else {
            let mut iter = MoveGen::new_legal(&board);
            let targets = board.color_combined(!board.side_to_move());
            iter.set_iterator_mask(*targets);

            for mov in &mut iter {
                // This move captures one of the opponents pieces (no en passant)
                let mut new_game = Self::make_move_r(game.clone(), mov);
                result = -Self::make_ai_move_r(&new_game, depth-1, -b, -a);
                result1 = result1.max(result);
                a = a.max(result);
                if a >= b {
                    break;
                }
            }

            iter.set_iterator_mask(!EMPTY);
            for mov in &mut iter {
                // This move does not capture anything
                let mut new_game = Self::make_move_r(game.clone(), mov);
                result = -Self::make_ai_move_r(&new_game, depth-1, -b, -a);
                result1 = result1.max(result);
                a = a.max(result);
                if a >= b {
                    break;
                }
            }
        }
        result1
    }

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
            return white_pieces - black_pieces
        }
        black_pieces - white_pieces
    }

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

    pub fn is_game_over(&self) -> bool {
        let mut game_over: bool = false;
        if let Some(result) = self.game.result() {
            if result == GameResult::BlackCheckmates || result == GameResult::WhiteCheckmates || result == GameResult::Stalemate {
                game_over = true;
            }
        }
        game_over
    }
}