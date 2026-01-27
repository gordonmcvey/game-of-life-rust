use life_game::Game;
use std::cmp::min;
use std::time::Duration;
use std::thread;

mod life_game;

fn main() {
    let (width, height) = space_for_game();
    let mut starting_state = vec![vec![false; width - 2]; height - 2];

    // gliders
    glider(&mut starting_state, 0, 0);
    glider(&mut starting_state, 3, 73);

    // pentadecathlons
    pentadecathlon(&mut starting_state, 19, 10);
    pentadecathlon(&mut starting_state, 12, 45);
    pentadecathlon(&mut starting_state, 34, 56);

    let mut game = Game::from_data(starting_state);

    loop {
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", game);
        thread::sleep(Duration::from_millis(100));
        // let mut a = String::new();
        // io::stdin().read_line(&mut a);
        game.step();
    }
}

fn glider(state: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    state[x][y + 1] = true;
    state[x + 1][y + 2] = true;
    state[x + 2][y] = true;
    state[x + 2][y + 1] = true;
    state[x + 2][y + 2] = true;
}

fn pentadecathlon(state: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    state[x][y + 2] = true;
    state[x][y + 7] = true;

    state[x + 1][y] = true;
    state[x + 1][y + 1] = true;
    state[x + 1][y + 3] = true;
    state[x + 1][y + 4] = true;
    state[x + 1][y + 5] = true;
    state[x + 1][y + 6] = true;
    state[x + 1][y + 8] = true;
    state[x + 1][y + 9] = true;

    state[x + 2][y + 2] = true;
    state[x + 2][y + 7] = true;
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
