use crate::life_game::{CellData, Game};

pub(crate) trait Solver {
    fn compute_state(&self, game: &Game) -> CellData;

    fn get_living_neighbour_count(game: &Game, row: usize, column: usize) -> u8 where Self: Sized {
        let game_state = &game.game_state;
        let dimensions = &game.dimensions;

        let above = (row + dimensions.height - 1) % dimensions.height;
        let below = (row + 1) % dimensions.height;
        let left = (column + dimensions.width - 1) % dimensions.width;
        let right = (column + 1) % game.dimensions.width;

        (if game_state[above][left] { 1 } else { 0 }
            + if game_state[above][column] { 1 } else { 0 }
            + if game_state[above][right] { 1 } else { 0 }
            + if game_state[row][left] { 1 } else { 0 }
            + if game_state[row][right] { 1 } else { 0 }
            + if game_state[below][left] { 1 } else { 0 }
            + if game_state[below][column] { 1 } else { 0 }
            + if game_state[below][right] { 1 } else { 0 })
    }

    fn decide_state(is_alive: bool, living_neighbours: u8) -> bool where Self: Sized {
        if is_alive && !(2..=3).contains(&living_neighbours) {
            false
        } else if !is_alive && living_neighbours == 3 {
            true
        } else {
            is_alive
        }
    }
}

pub(crate) struct SingleThreadedSolver;

impl Solver for SingleThreadedSolver {
    fn compute_state(&self, game: &Game) -> CellData {
        let mut new_state = vec![vec![false; game.dimensions.width]; game.dimensions.height];
        let current_state = &game.game_state;
        let dimensions = &game.dimensions;

        for row in 0..dimensions.height {
            for column in 0..dimensions.width {
                let is_alive = current_state[row][column];
                let living_neighbours = Self::get_living_neighbour_count(game, row, column);
                new_state[row][column] = Self::decide_state(is_alive, living_neighbours);
            }
        }

        new_state
    }
}
