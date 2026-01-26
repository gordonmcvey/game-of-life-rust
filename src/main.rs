use life_game::Game;
use std::thread;
use std::time::Duration;

mod life_game;

fn main() {
    let mut game = Game::from_data(
        vec![
            vec![true, false, false, true, false, false, true, false, false],
            vec![true, false, false, true, false, false, true, false, false],
            vec![false, false, true, false, false, true, false, false, true],
            vec![false, false, false, false, false, false, false, false, false],
            vec![false, false, true, false, false, true, false, false, true],
        ]
    );

    loop {
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", game);
        thread::sleep(Duration::from_millis(500));
        game.step();
    }
}
