mod disjointset;
mod hexgame;

use disjointset::DisjointSet;
use hexgame::HexGame;
use std::io;

fn main() {
    println!("Welcome to HexGame! Please specify the size of the board as \
              an integer: ");

    let size = get_int_input();
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
                player_move = get_int_input();
                if player_move > (game.size * game.size) - 1 {
                    println!("Move is off of the board!");
                    continue;
                }
                break;
            }

            if game.play_blue(player_move, false) {
                println!("Blue wins with move at {}!!", player_move);
                break;
            }
        }
        // red's move
        else {
            println!("Red's move: ");

            let mut player_move: usize;
            loop {
                player_move = get_int_input();
                if player_move > (game.size * game.size) - 1 {
                    println!("Move is off of the board!");
                    continue;
                }
                break;
            }

            if game.play_red(player_move, false) {
                println!("Red wins with move at {}!!", player_move);
            }
        }

        move_counter += 1;
    }

    game.print();
}

fn get_int_input() -> usize {
    let stdin = io::stdin();
    let input = &mut String::new();
    let value;

    loop {
        match stdin.read_line(input) {
            Ok(_) => {},
            Err(_) => {
                println!("Please input a positive integer: ");
            }
        }

        value = match input.trim().parse::<usize>() {
            Ok(num) => { num },
            Err(_) => {
                println!("Please input a positive integer: ");
                continue;
            }
        };

        break;
    };

    return value;
}