use crate::life_game::builder;
use crate::life_game::CellData;
use life_game::Game;
use std::cmp::min;
use std::time::Duration;
use std::{io, thread};

mod life_game;

fn main() {
    let (width, height) = space_for_game();
    let mut starting_state: CellData = vec![vec![false; width - 2]; height - 2];
    let mut input = String::new();

    println!("Select puzzle:");
    println!("1: Preset starting state");
    println!("2: Random population, 5%");
    println!("3: Random population, 10%");
    println!("4: Random population, 20%");
    println!();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(1) => {
            // gliders
            builder::glider(&mut starting_state, 0, 0);
            builder::glider(&mut starting_state, 73, 3);

            // pentadecathlons
            builder::pentadecathlon(&mut starting_state, 10, 19);
            builder::pentadecathlon(&mut starting_state, 45, 12);
            builder::pentadecathlon(&mut starting_state, 56, 34)
        },
        Ok(2) => builder::randomise(&mut starting_state, 5),
        Ok(3) => builder::randomise(&mut starting_state, 10),
        Ok(4) => builder::randomise(&mut starting_state, 20),
        Ok(_) | Err(_) => {
            println!("Invalid selection");
            return;
        },
    };

    let mut game = Game::from_data(starting_state);

    while !game.has_stabilised() {
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", game);
        thread::sleep(Duration::from_millis(100));
        game.step();
    }

    println!("Game over!  State stabilised after {} iterations", game.iteration());
}

fn space_for_game() -> (usize, usize) {
    let dimensions = term_size::dimensions();
    if dimensions.is_some() {
        let raw_dimensions = dimensions.unwrap();
        return (
            min(raw_dimensions.0, 200),
            min(raw_dimensions.1, 50),
        );
    }
    (80, 25)
}
