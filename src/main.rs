use crate::life_game::render::CharacterMapRenderer;
use crate::life_game::CellData;
use crate::life_game::{builder, Game};
use std::cmp::min;
use std::num::ParseIntError;
use std::time::Duration;
use std::{io, process, thread};

mod life_game;

fn main() {
    let (display_width, display_height) = space_for_game();

    let (
        game_width_multiplier,
        game_height_multiplier,
        renderer
    ) = configure_rendering(prompt_rendering()).unwrap_or_else(|| process::exit(1));

    let game_width = (display_width - 2) * game_width_multiplier;
    let game_height = (display_height - 2) * game_height_multiplier;

    let starting_state: CellData = configure_game(prompt_game(), game_width, game_height).unwrap_or_else(|| process::exit(1));
    let mut game = Game::from_data(starting_state);

    while !game.has_stabilised() {
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", renderer.render(&game));
        thread::sleep(Duration::from_millis(100));
        game.step();
    }

    println!("Game over!  State stabilised after {} iterations", game.iteration());
}

fn prompt_rendering() -> Result<i32, ParseIntError> {
    let mut input = String::new();

    println!("Select rendering mode:");
    println!("1: One cell per character  (' ', '█'),           simplest, lowest resolution");
    println!("2: Two cells per character (' ', '▀', '▄', '█'), double horizontal resolution");
    println!("3: 4 cells per character   ('▘', '▚', '▞', '▙'), double horizontal and vertical resolution");
    println!();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<i32>()
}

fn configure_rendering(response: Result<i32, ParseIntError>) -> Option<(usize, usize, CharacterMapRenderer)> {
    match response {
        Ok(1) => Some((
            1,
            1,
            CharacterMapRenderer::single_cell_per_char()
        )),
        Ok(2) => Some((
            1,
            2,
            CharacterMapRenderer::two_cells_per_char()
        )),
        Ok(3) => Some((
            2,
            2,
            CharacterMapRenderer::four_cells_per_char()
        )),
        Ok(_) | Err(_) => {
            println!("Invalid selection");
            None
        },
    }
}

fn prompt_game() -> Result<i32, ParseIntError> {
    let mut input = String::new();

    println!("Select puzzle:");
    println!("1: Preset starting state");
    println!("2: A flock of seagulls (gliders)");
    println!("3: Lightweight spaceship invasion!");
    println!("4: Achim's P144 long period oscillator");
    println!("5: Random population, 5%");
    println!("6: Random population, 10%");
    println!("7: Random population, 20%");
    println!();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<i32>()
}

fn configure_game(response: Result<i32, ParseIntError>, game_width: usize, game_height: usize) -> Option<CellData> {
    let mut starting_state: CellData = vec![vec![false; game_width]; game_height];

    match response {
        Ok(1) => {
            // gliders
            builder::glider(&mut starting_state, 0, 0);
            builder::glider(&mut starting_state, 73, 3);

            // pentadecathlons
            builder::pentadecathlon(&mut starting_state, 10, 19);
            builder::pentadecathlon(&mut starting_state, 45, 12);
            builder::pentadecathlon(&mut starting_state, 56, 34)
        },
        Ok(2) => {
            for col in (0 .. game_width - 12).step_by(10) {
                builder::glider(&mut starting_state, col, 0);
            }
        },
        Ok(3) => {
            for row in (0 .. game_height - 6).step_by(6) {
                builder::lightweight_spaceship(&mut starting_state, game_width - 10, row);
            }
        }
        Ok(4) => {
            for row in (1 .. game_height - 23).step_by(21) {
                for column in (1 .. game_width - 33).step_by(31) {
                    builder::achim_p144(&mut starting_state, column, row);
                }
            }
        },
        Ok(5) => builder::randomise(&mut starting_state, 5),
        Ok(6) => builder::randomise(&mut starting_state, 10),
        Ok(7) => builder::randomise(&mut starting_state, 20),
        Ok(_) | Err(_) => {
            println!("Invalid selection");
            return None;
        },
    };

    Some(starting_state)
}

fn space_for_game() -> (usize, usize) {
    match term_size::dimensions() {
        Some(dimensions) => (
            min(dimensions.0, 200),
            min(dimensions.1, 50),
        ),
        None => (80, 25),
    }
}
