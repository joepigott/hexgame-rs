mod disjointset;
mod hexgame;
mod termcolor;

use disjointset::DisjointSet;
use hexgame::HexGame;
use std::io;

fn main() {
    println!(
        "Welcome to HexGame! The goal of the game is to connect the two\n\
              sides: the red player wins if they connect the top and bottom,\n\
              and the blue player wins if they connect the left and right.\n"
    );

    println!(
        "At any time, input 'q' to exit the game. To get started, specify\n\
              the size of the board (must be greater than 1).\n"
    );

    let mut size: usize;

    loop {
        size = match get_int_input() {
            Some(num) => num,
            None => {
                println!("Exiting...");
                return;
            }
        };

        if size >= 2 {
            break;
        }
    }

    play_game(&mut HexGame::new(size));
}

fn play_game(game: &mut HexGame) {
    let mut move_counter = 0;

    loop {
        game.print();

        // blue's move
        if move_counter % 2 == 0 {
            println!("Blue's move: ");

            let mut player_move: usize;
            loop {
                player_move = match get_int_input() {
                    Some(num) => num,
                    None => {
                        println!("Exiting...");
                        return;
                    }
                };
                if !game.is_valid_move(player_move) {
                    println!("Move is off of the board!");
                    continue;
                }
                break;
            }

            if game.play_blue(player_move) {
                game.print();
                println!("Blue wins with move at {}!!", player_move);
                break;
            }
        }
        // red's move
        else {
            println!("Red's move: ");

            let mut player_move: usize;
            loop {
                player_move = match get_int_input() {
                    Some(num) => num,
                    None => {
                        println!("Exiting...");
                        return;
                    }
                };
                if !game.is_valid_move(player_move) {
                    println!("Move is off of the board!");
                    continue;
                }
                break;
            }

            if game.play_red(player_move) {
                game.print();
                println!("Red wins with move at {}!!", player_move);
                break;
            }
        }

        move_counter += 1;
    }
}

fn get_int_input() -> Option<usize> {
    loop {
        println!("Please enter a positive integer: ");

        let input = &mut String::new();
        match io::stdin().read_line(input) {
            Ok(_) => {
                if input.trim() == "q" || input.trim() == "Q" {
                    return None;
                }

                match input.trim().parse::<usize>() {
                    Ok(num) => {
                        return Some(num);
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            Err(_) => {
                continue;
            }
        }
    }
}
