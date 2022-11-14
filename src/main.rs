mod game;
use std::{io, process};
use crate::game::game::Game;
use crate::game::board::Moves;

fn main() {
    let mut game = Game::start();

    loop {
        game.show_board();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        let res = match input.as_str() {
            "w" => game.process_move(Moves::Up),
            "s" => game.process_move(Moves::Down),
            "a" => game.process_move(Moves::Left),
            "d" => game.process_move(Moves::Right),
            "q" => {
                process::exit(1);
            }
            _ => {true}
        };
        if !res {
            game.show_board();
            println!("You lose");
            break;
        }
        game.put_new_value();
    }
}
