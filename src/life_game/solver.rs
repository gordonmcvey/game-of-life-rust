use crate::life_game::{CellData, Game};
use std::sync::Arc;
use std::thread;

pub(crate) trait Solver {
    fn compute_state(&self, game: &Game) -> CellData;

    fn get_living_neighbour_count(game_state: &CellData, row: usize, column: usize) -> u8 where Self: Sized {
        let width = game_state[0].len();
        let height = game_state.len();

        let above = (row + height - 1) % height;
        let below = (row + 1) % height;
        let left = (column + width - 1) % width;
        let right = (column + 1) % width;

        game_state[above][left] as u8
            + game_state[above][column] as u8
            + game_state[above][right] as u8
            + game_state[row][left] as u8
            + game_state[row][right] as u8
            + game_state[below][left] as u8
            + game_state[below][column] as u8
            + game_state[below][right] as u8
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

pub(crate) struct ThreadedSolver {
    thread_count: usize,
}

impl Solver for SingleThreadedSolver {
    fn compute_state(&self, game: &Game) -> CellData {
        let mut new_state = vec![vec![false; game.dimensions.width]; game.dimensions.height];
        let current_state = &game.game_state;

        for row in 0..game.dimensions.height {
            for column in 0..game.dimensions.width {
                let is_alive = current_state[row][column];
                let living_neighbours = Self::get_living_neighbour_count(current_state, row, column);
                new_state[row][column] = Self::decide_state(is_alive, living_neighbours);
            }
        }

        new_state
    }
}

impl ThreadedSolver {
    pub(crate) fn new(thread_count: usize) -> Self {
        Self { thread_count }
    }
}

impl Solver for ThreadedSolver {

    fn compute_state(&self, game: &Game) -> CellData {
        let current = Arc::new(game.game_state.clone());

        let handles: Vec<_> = (0..self.thread_count)
            .map(|thread_id| {
                let thread_current = Arc::clone(&current);
                let width = game.dimensions.width;
                let height = game.dimensions.height;
                let chunk_size = height.div_ceil(self.thread_count);

                thread::spawn(move || {
                    let mut new_chunk: CellData = Vec::new();
                    let row_start = thread_id * chunk_size;
                    let row_end = (row_start + chunk_size).min(height);

                    for row in row_start..row_end {
                        let mut new_row = vec![false; width];
                        for column in 0..width {
                            let is_alive = thread_current[row][column];
                            let living_neighbours = Self::get_living_neighbour_count(&thread_current, row, column);

                            new_row[column] = Self::decide_state(is_alive, living_neighbours);
                        }
                        new_chunk.push(new_row);
                    }

                    new_chunk
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|h: std::thread::JoinHandle<std::vec::Vec<_>>| h.join().unwrap())
            .collect()
    }
}
