pub mod builder;

use std::fmt::Display;
use std::hash::{Hash, Hasher};

/// The vector that represents the game state.  The outer vector represents the rows (so its index
/// is the Y axis coordinate), and the inner vector represents the cells for each column in that row
/// (so its index is the X axis coordinate).
pub type CellData = Vec<Vec<bool>>;

pub struct Game {
    width: usize,
    height: usize,
    iteration: usize,
    game_state: CellData,
    previous_states: [u64; Self::ITERATION_HISTORY],
}

impl Game {
    const ITERATION_HISTORY: usize = 32;

    pub fn new(width: usize, height: usize) -> Self {
        Game {
            width,
            height,
            iteration: 0,
            game_state: vec![vec![false; width]; height],
            previous_states: [0; Self::ITERATION_HISTORY],
        }
    }

    pub fn from_data(cells: CellData) -> Self {
        Game {
            width: cells[0].len(),
            height: cells.len(),
            iteration: 0,
            game_state: cells,
            previous_states: [0; Self::ITERATION_HISTORY],
        }
    }

    pub fn step(&mut self) {
        let mut new_state = vec![vec![false; self.width]; self.height];

        for row in 0..self.height {
            for column in 0..self.width {
                let is_alive = self.game_state[row][column];
                let living_neighbours = self.get_living_neighbour_count(row, column);
                // println!("{}, {} has {} live neighbours", row, column, living_neighbours);

                if is_alive && (living_neighbours < 2 || living_neighbours > 3) {
                    new_state[row][column] = false;
                } else if !is_alive && living_neighbours == 3 {
                    new_state[row][column] = true;
                } else {
                    new_state[row][column] = is_alive;
                }
            }
        }

        self.previous_states[self.iteration % Self::ITERATION_HISTORY] = self.hash();
        self.iteration += 1;

        self.game_state = new_state;
    }

    pub fn iteration(&self) -> usize {
        self.iteration
    }

    pub fn has_stabilised(&self) -> bool {
        self.previous_states.contains(&self.hash())
    }

    fn hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.game_state.hash(&mut hasher);
        hasher.finish()
    }

    fn get_living_neighbour_count(&self, row: usize, column: usize) -> u8 {
        let above = (row + self.height - 1) % self.height;
        let below = (row + 1) % self.height;
        let left = (column + self.width - 1) % self.width;
        let right = (column + 1) % self.width;

        let count = if self.game_state[above][left] { 1 } else { 0 }
            + if self.game_state[above][column] { 1 } else { 0 }
            + if self.game_state[above][right] { 1 } else { 0 }
            + if self.game_state[row][left] { 1 } else { 0 }
            + if self.game_state[row][right] { 1 } else { 0 }
            + if self.game_state[below][left] { 1 } else { 0 }
            + if self.game_state[below][column] { 1 } else { 0 }
            + if self.game_state[below][right] { 1 } else { 0 };

        count
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border = std::iter::repeat("━").take(self.width).collect::<String>();
        // Output buffer should allocate 4 bytes for each cell in the grid plus 2 extra rows and columns
        let mut output = String::with_capacity(((self.width + 2) * (self.height + 2)) * 4);

        output.push_str("┏");
        output.push_str(&border);
        output.push_str("┓\n");

        for row in self.game_state.iter() {
            output.push_str("┃");
            for cell in row.iter() {
                let cell_output = {
                    if *cell {
                        String::from("█")
                    } else {
                        String::from(" ")
                    }
                };

                output.push_str(format!("{}", cell_output).as_str());
            }
            output.push_str("┃\n");
        }

        output.push_str("┗");
        output.push_str(&border);
        output.push_str("┛\n");

        write!(f, "{}", output)
    }
}
