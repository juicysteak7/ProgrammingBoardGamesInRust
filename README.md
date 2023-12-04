# ProgrammingBoardGamesInRust

Pedro Gonzalez

Professor Bart Massey

CS 410: Rust Programming

# Description
For licensing information please look at the LICENSE file at the root of the project.

This repo is for my class projects of both Rust and A.I.

A.I. has proven to be a good test of algorithm knowledge, and data structure knowledge. 

To quote Bart, Rust is all about good data design.

This application will start the game loop of: 

    print the gameboard to the player, 

    ask the player for thier move, 

    apply move,

    print the gameboard to the player,

    choose a A.I. move, 

    apply move,

    print the gameboard to the player,

    until there is either a checkmate, or a stalemate. 

I want to add scalable difficulty when you are playing against the A.I. 

The max difficulty should still have reasonable speed. 

I want the player to be able to pick if they want to go first or second (choose their color). 

The A.I. should use negamax search of a specified depth depending on the difficulty set by the player. 

The negamax should also use a transposition table and alpha - beta pruning. There will be iterative deepening depending on the difficulty selected by the player.

# Project Notes

This project leverages the pre-existing [chess](https://crates.io/crates/chess) crate for the game logic I didn't have time to implement.

In order to run this project you must have Rust and Crate installed.

You can install them [here](https://www.rust-lang.org/)

Then the command to start the program is cargo run.

Once the programm starts the chess game will begin.

It will promt you for your move, generate an A.I. move, print the board, then loop again.

All in all I enjoyed this project, I spent a lot of time on my other A.I. project and wish I had more time to fine tune this A.I. 

Testing this project involved a lot of by hand testing. 

I struggled to develop meaningful tests because I leverage an outside crate so frequently.

This is something I can improve on from this project.

It might be a project I return to.

If I do return to it, I would implement a tansposition table so the program can store a look up positions it has visited.

I might then write that transposition table to a file to ingest when the program starts again so it can save positions between invokations.