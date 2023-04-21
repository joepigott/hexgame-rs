mod disjointset;
mod hexgame;

use disjointset::DisjointSet;
use hexgame::HexGame;
use std::io;

fn main() {
    println!("Welcome to HexGame! Please specify the board size.");

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
                if player_move > (game.size * game.size) ||
                   player_move == 0 {
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
                player_move = get_int_input();
                if player_move > (game.size * game.size) ||
                   player_move == 0{
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

fn get_int_input() -> usize {
    let value: usize;

    loop {
        println!("Please enter a positive integer: ");

        let input = &mut String::new();
        match io::stdin().read_line(input) {
            Ok(_) => {
                value = match input.trim().parse::<usize>() {
                    Ok(num) => { num },
                    Err(_) => { continue; }
                };

                break;
            },
            Err(_) => { continue; }
        }
    };

    return value;
}
