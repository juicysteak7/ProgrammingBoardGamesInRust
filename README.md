# ProgrammingBoardGamesInRust
This repo is for my class projects of both Rust and A.I.

A.I. has proven to be a good test of algorithm knowledge, and data structure knowledge. To quote Bart, Rust is all about good data design.

This application will iteratively print the gameboard to the player, ask the player for thier move, choose a move, until there is either a checkmate, or a stalemate. I want to add scalable difficulty when you are playing against the A.I. The max difficulty should still have reasonable speed. I want the player to be able to pick if they want to go first or second (choose their color). The A.I. should use negamax search of a specified depth depending on the difficulty set by the player. The negamax should also use a transposition table and alpha - beta pruning. There will be iterative deepening depending on the difficulty selected by the player.

**I want my A.I. on max difficulty to beat me, easily.**

Example Board Layout:
   
   a b c d e f g h

8 r n b q k b n r   (Black)

7 p p p p p p p p

6 . . . . . . . .   (Empty)

5 . . . . . . . .

4 . . . . . . . .

3 . . . . . . . .

2 P P P P P P P P

1 R N B Q K B N R   (White)
