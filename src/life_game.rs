pub mod builder;
pub mod render;
pub(crate) mod solver;

use crate::life_game::solver::SolverBox;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

/// The vector that represents the game state.  The outer vector represents the rows (so its index
/// is the Y axis coordinate), and the inner vector represents the cells for each column in that row
/// (so its index is the X axis coordinate).
pub type CellData = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Game {
    dimensions: Dimensions,
    iteration: usize,
    game_state: CellData,
    previous_states: [u64; Self::ITERATION_HISTORY],
    solver: SolverBox,
}

impl Game {
    const ITERATION_HISTORY: usize = 32;

    pub fn new(dimensions: Dimensions, solver: SolverBox) -> Self {
        let width = dimensions.width;
        let height = dimensions.height;
        Game {
            dimensions,
            iteration: 0,
            game_state: vec![vec![false; width]; height],
            previous_states: [0; Self::ITERATION_HISTORY],
            solver,
        }
    }

    pub fn from_data(cells: CellData, solver: SolverBox) -> Self {
        let mut game = Self::new(
            Dimensions {
                width: cells[0].len(),
                height: cells.len(),
            },
            solver,
        );
        game.game_state = cells;
        game
    }

    pub fn step(&mut self) {
        let new_state = self.solver.compute_state(&self.game_state);

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

    pub fn get_cell_chunk_at(
        &self,
        coordinates: &Coordinates,
        dimensions: &Dimensions,
    ) -> CellData {
        // get a chunk of cells from the game state at the given co-ordinates.  The chunk will be a
        // group of cells that can be mapped onto a display character (eg if one character can
        // represent a group of 2x4 cells, then take 8 cells total, as two columns of 4 rows each
        self.game_state
            .iter()
            .skip(coordinates.y)
            .take(dimensions.height)
            .map(|row| {
                row.iter()
                    .skip(coordinates.x)
                    .take(dimensions.width)
                    .copied() // Dereference the bool reference
                    .collect::<Vec<bool>>()
            })
            .collect()
    }

    fn hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.game_state.hash(&mut hasher);
        hasher.finish()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border = std::iter::repeat_n("━", self.dimensions.width).collect::<String>();
        // Output buffer should allocate 4 bytes for each cell in the grid plus 2 extra rows and columns
        let mut output =
            String::with_capacity(((self.dimensions.width + 2) * (self.dimensions.height + 2)) * 4);

        output.push('┏');
        output.push_str(&border);
        output.push_str("┓\n");

        for row in self.game_state.iter() {
            output.push('┃');
            for cell in row.iter() {
                let cell_output = {
                    if *cell {
                        String::from("█")
                    } else {
                        String::from(" ")
                    }
                };

                output.push_str(cell_output.to_string().as_str());
            }
            output.push_str("┃\n");
        }

        output.push('┗');
        output.push_str(&border);
        output.push_str("┛\n");

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::{CellData, Coordinates, Dimensions, Game};
    use crate::life_game::solver::SingleThreadedSolver;

    #[test]
    fn it_counts_iterations() {
        let expected = 100;
        // @todo The solver should be a mock
        let mut game = Game::new(
            Dimensions {
                width: 10,
                height: 10,
            },
            Box::from(SingleThreadedSolver),
        );
        assert_eq!(0, game.iteration());

        for _ in 0..expected {
            game.step();
        }

        assert_eq!(
            expected,
            game.iteration(),
            "Game should have counted {} iterations, actually counted {}",
            expected,
            game.iteration(),
        );
    }

    #[test]
    fn it_detects_stability() {
        let grid: CellData = vec![
            vec![false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false],
            vec![false, false, false, true, false, false, false],
            vec![false, false, false, true, false, false, false],
            vec![false, false, false, true, false, false, false],
            vec![false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false],
        ];
        // @todo The solver should be a mock
        let mut game = Game::from_data(grid, Box::from(SingleThreadedSolver));

        // We're dealing with a blinker with a period of 2, so iterations 0 and 1 should not be
        // considered stable, but iteration 2 is a repeat of iteration 0, so we can consider the
        // game stable even if it won't become a still-life.
        assert!(
            !game.has_stabilised(),
            "Game should not be stable at iteration 0"
        );
        game.step();
        assert!(
            !game.has_stabilised(),
            "Game should not be stable at iteration 1"
        );
        game.step();
        assert!(
            game.has_stabilised(),
            "Game should be stable at iteration 2"
        );
    }

    #[test]
    fn it_returns_chunk_of_game_state() {
        // @todo The solver should be a mock
        let game = Game::from_data(
            vec![
                vec![false, false, false, false],
                vec![false, false, false, true],
                vec![false, false, true, false],
                vec![false, false, true, true],
                vec![false, true, false, false],
                vec![false, true, false, true],
                vec![false, true, true, false],
                vec![false, true, true, true],
                vec![true, false, false, false],
                vec![true, false, false, true],
                vec![true, false, true, false],
                vec![true, false, true, true],
                vec![true, true, false, false],
                vec![true, true, false, true],
                vec![true, true, true, false],
                vec![true, true, true, true],
            ],
            Box::from(SingleThreadedSolver),
        );

        assert_eq!(
            vec![vec![false],],
            game.get_cell_chunk_at(
                &Coordinates { x: 0, y: 0 },
                &Dimensions {
                    width: 1,
                    height: 1
                }
            ),
            "Returned chunk doesn't match expected chunk value",
        );

        assert_eq!(
            vec![
                vec![false, true, true, false],
                vec![false, true, true, true],
                vec![true, false, false, false],
                vec![true, false, false, true],
            ],
            game.get_cell_chunk_at(
                &Coordinates { x: 0, y: 6 },
                &Dimensions {
                    width: 4,
                    height: 4
                }
            ),
            "Returned chunk doesn't match expected chunk value",
        );

        assert_eq!(
            vec![
                vec![true, false, true],
                vec![true, true, false],
                vec![true, true, true],
            ],
            game.get_cell_chunk_at(
                &Coordinates { x: 1, y: 13 },
                &Dimensions {
                    width: 4,
                    height: 4
                }
            ),
            "Returned chunk doesn't match expected chunk value",
        );

        assert_eq!(
            vec![vec![true],],
            game.get_cell_chunk_at(
                &Coordinates { x: 3, y: 15 },
                &Dimensions {
                    width: 4,
                    height: 4
                }
            ),
            "Returned chunk doesn't match expected chunk value",
        );
    }

    // Commenting this out because it's tested in the unit tests for the solvers, we can use this
    // as the basis for an integration test later.
    //
    // // Check that a glider evolves as expected for a game of life
    // #[test]
    // fn it_handles_a_glider_evolution() {
    //     let start_state: CellData = vec![
    //         vec![false, true, false, false, false, false],
    //         vec![false, false, true, false, false, false],
    //         vec![true, true, true, false, false, false],
    //         vec![false, false, false, false, false, false],
    //         vec![false, false, false, false, false, false],
    //         vec![false, false, false, false, false, false],
    //     ];
    //
    //     // @todo The solver should be a mock
    //     let mut game = Game::from_data(start_state, Box::from(SingleThreadedSolver));
    //     assert_eq!(
    //         vec![
    //             vec![false, true, false, false, false, false],
    //             vec![false, false, true, false, false, false],
    //             vec![true, true, true, false, false, false],
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //         ],
    //         game.game_state,
    //         "State is not the specified start state",
    //     );
    //
    //     game.step();
    //     assert_eq!(
    //         vec![
    //             vec![false, false, false, false, false, false],
    //             vec![true, false, true, false, false, false],
    //             vec![false, true, true, false, false, false],
    //             vec![false, true, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //         ],
    //         game.game_state,
    //         "State no longer resembles a glider after one iteration",
    //     );
    //
    //     game.step();
    //     assert_eq!(
    //         vec![
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, true, false, false, false],
    //             vec![true, false, true, false, false, false],
    //             vec![false, true, true, false, false, false],
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //         ],
    //         game.game_state,
    //         "State no longer resembles a glider after two iterations",
    //     );
    //
    //     game.step();
    //     assert_eq!(
    //         vec![
    //             vec![false, false, false, false, false, false],
    //             vec![false, true, false, false, false, false],
    //             vec![false, false, true, true, false, false],
    //             vec![false, true, true, false, false, false],
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //         ],
    //         game.game_state,
    //         "State no longer resembles a glider after three iterations",
    //     );
    //
    //     game.step();
    //     assert_eq!(
    //         vec![
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, true, false, false, false],
    //             vec![false, false, false, true, false, false],
    //             vec![false, true, true, true, false, false],
    //             vec![false, false, false, false, false, false],
    //             vec![false, false, false, false, false, false],
    //         ],
    //         game.game_state,
    //         "State no longer resembles a glider after four iterations",
    //     );
    // }
}
