use crate::life_game::CellData;
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub type SolverBox = Box<dyn Solver>;
type Job = Box<dyn FnOnce() -> (usize, CellData) + Send + 'static>;

pub(crate) trait Solver {
    fn compute_state(&self, current_state: &CellData) -> CellData;

    fn get_living_neighbour_count(game_state: &CellData, row: usize, column: usize) -> u8
    where
        Self: Sized,
    {
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

    fn decide_state(is_alive: bool, living_neighbours: u8) -> bool
    where
        Self: Sized,
    {
        if is_alive && !(2..=3).contains(&living_neighbours) {
            false
        } else if !is_alive && living_neighbours == 3 {
            true
        } else {
            is_alive
        }
    }

    fn compute_chunk(
        thread_id: usize,
        chunk_size: usize,
        width: usize,
        height: usize,
        thread_current: &CellData,
    ) -> CellData
    where
        Self: Sized,
    {
        let row_start = thread_id * chunk_size;
        let row_end = (row_start + chunk_size).min(height);
        let mut new_chunk: CellData = Vec::with_capacity(row_end - row_start);

        for row in row_start..row_end {
            let mut new_row = vec![false; width];
            for column in 0..width {
                let is_alive = thread_current[row][column];
                let living_neighbours =
                    Self::get_living_neighbour_count(&thread_current, row, column);

                new_row[column] = Self::decide_state(is_alive, living_neighbours);
            }
            new_chunk.push(new_row);
        }

        new_chunk
    }
}

impl Debug for dyn Solver {
    // @todo Sensible output for debug
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

pub(crate) struct SingleThreadedSolver;

pub(crate) struct ThreadedSolver {
    thread_count: usize,
}

pub(crate) struct ThreadPoolSolver {
    pool_size: usize,
    job_sender: Sender<Option<Job>>,
    result_receiver: Mutex<mpsc::Receiver<(usize, CellData)>>,
}

impl Solver for SingleThreadedSolver {
    fn compute_state(&self, current_state: &CellData) -> CellData {
        let mut new_state = current_state.clone();
        let height = current_state.len();
        let width = {
            if height > 0 {
                current_state[0].len()
            } else {
                0
            }
        };

        for row in 0..height {
            for column in 0..width {
                let is_alive = current_state[row][column];
                let living_neighbours =
                    Self::get_living_neighbour_count(current_state, row, column);
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
    fn compute_state(&self, current_state: &CellData) -> CellData {
        let current = Arc::new(current_state.clone());
        let height = current_state.len();
        let width = {
            if height > 0 {
                current_state[0].len()
            } else {
                0
            }
        };

        let handles: Vec<_> = (0..self.thread_count)
            .map(|thread_id| {
                let thread_current = Arc::clone(&current);
                // let width = game.dimensions.width;
                // let height = game.dimensions.height;
                let chunk_size = height.div_ceil(self.thread_count);

                thread::spawn(move || {
                    Self::compute_chunk(thread_id, chunk_size, width, height, &thread_current)
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|h: std::thread::JoinHandle<std::vec::Vec<_>>| h.join().unwrap())
            .collect()
    }
}

impl ThreadPoolSolver {
    pub fn new(pool_size: usize) -> Self {
        // The job channel is used to send jobs from the main thread to the workers.
        let (job_tx, job_rx) = mpsc::channel::<Option<Job>>();

        // The result channel is used to collect job results from the workers in the main thread.
        let (result_tx, result_rx) = mpsc::channel::<(usize, CellData)>();

        // We want to share the receiving end of the job channel with all the threads in the pool.
        // This is how work will be enqueued for the workers to pick up, and the mutex will prevent
        // races between workers looking for new units of work
        let job_receiver = Arc::new(Mutex::new(job_rx));

        // Spin up the thread pool
        for _ in 0..pool_size {
            // There is only ever one instance of job_receiver, we're only cloning the reference to
            // it.  However, we are cloning the result sender.  This should not violate the single
            // consumer constraint of the mpsc channel.
            let job_receiver = Arc::clone(&job_receiver);
            let result_sender = result_tx.clone();

            thread::spawn(move || {
                loop {
                    // Get a new unit of work (the mutex will lock other workers from getting the
                    // same job, but we only need to lock it for this line so it should not cause
                    // significant thread contention)
                    //
                    // NOTE: Use of unwrap() here may require some thought.
                    let job = job_receiver.lock().unwrap().recv().unwrap();

                    match job {
                        Some(job) => {
                            // Run the job and send the result back to the main thread.
                            //
                            // NOTE: Use of unwrap() here may require some thought.
                            let job_result = job();
                            result_sender.send(job_result).unwrap();
                        }
                        // If we receive a None, then shut down the worker
                        _ => break,
                    }
                }
            });
        }

        Self {
            pool_size,
            job_sender: job_tx,
            result_receiver: Mutex::new(result_rx),
        }
    }
}

impl Drop for ThreadPoolSolver {
    fn drop(&mut self) {
        // Send a None shutdown signal to each worker
        for _ in 0..self.pool_size {
            self.job_sender.send(None).unwrap();
        }
    }
}

impl Solver for ThreadPoolSolver {
    fn compute_state(&self, current_state: &CellData) -> CellData {
        let current = Arc::new(current_state.clone());
        let height = current_state.len();
        let width = {
            if height > 0 {
                current_state[0].len()
            } else {
                0
            }
        };
        let chunk_size = height.div_ceil(self.pool_size);

        for thread_id in 0..self.pool_size {
            let thread_current = Arc::clone(&current);
            let job = Box::new(move || {
                let new_chunk: CellData =
                    Self::compute_chunk(thread_id, chunk_size, width, height, &thread_current);
                (thread_id, new_chunk)
            });

            self.job_sender.send(Some(job)).unwrap();
        }

        // Collect exactly thread_count results, then sort by starting row
        // to reassemble in the correct order
        let receiver = self.result_receiver.lock().unwrap();
        let mut chunks: Vec<(usize, CellData)> = (0..self.pool_size)
            .map(|_| receiver.recv().unwrap())
            .collect();

        // Results may arrive out of order — sort by chunk index
        chunks.sort_by_key(|(i, _)| *i);
        chunks.into_iter().flat_map(|(_, chunk)| chunk).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_a_glider_evolution_for_single_threaded_solver() {
        assert_glider_evolution(SingleThreadedSolver);
    }

    #[test]
    fn it_handles_a_dead_grid_for_single_threaded_solver() {
        assert_dead_grid_stays_dead(SingleThreadedSolver);
    }

    #[test]
    fn it_handles_an_alive_grid_for_single_threaded_solver() {
        assert_alive_grid_dies(SingleThreadedSolver);
    }

    #[test]
    fn it_handles_a_four_block_still_life_for_single_threaded_solver() {
        assert_four_block_still_life(SingleThreadedSolver);
    }

    #[test]
    fn it_handles_a_glider_evolution_for_multi_threaded_solver() {
        assert_glider_evolution(ThreadedSolver::new(2));
    }

    #[test]
    fn it_handles_a_dead_grid_for_multi_threaded_solver() {
        assert_dead_grid_stays_dead(ThreadedSolver::new(2));
    }

    #[test]
    fn it_handles_an_alive_grid_for_multi_threaded_solver() {
        assert_alive_grid_dies(ThreadedSolver::new(2));
    }

    #[test]
    fn it_handles_a_four_block_still_life_for_multi_threaded_solver() {
        assert_four_block_still_life(ThreadedSolver::new(2));
    }

    #[test]
    fn it_handles_a_glider_evolution_for_thread_pool_solver() {
        assert_glider_evolution(ThreadPoolSolver::new(2));
    }

    #[test]
    fn it_handles_a_dead_grid_for_thread_pool_solver() {
        assert_dead_grid_stays_dead(ThreadPoolSolver::new(2));
    }

    #[test]
    fn it_handles_an_alive_grid_for_thread_pool_solver() {
        assert_alive_grid_dies(ThreadPoolSolver::new(2));
    }

    #[test]
    fn it_handles_a_four_block_still_life_for_thread_pool_solver() {
        assert_four_block_still_life(ThreadPoolSolver::new(2));
    }

    // Check that a glider evolves as expected over the course of a few steps
    fn assert_glider_evolution<S: Solver>(solver: S) {
        let glider_states: [CellData; 4] = [
            vec![
                vec![false, false, false, false, false, false],
                vec![true, false, true, false, false, false],
                vec![false, true, true, false, false, false],
                vec![false, true, false, false, false, false],
                vec![false, false, false, false, false, false],
                vec![false, false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false, false],
                vec![false, false, true, false, false, false],
                vec![true, false, true, false, false, false],
                vec![false, true, true, false, false, false],
                vec![false, false, false, false, false, false],
                vec![false, false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false, false],
                vec![false, true, false, false, false, false],
                vec![false, false, true, true, false, false],
                vec![false, true, true, false, false, false],
                vec![false, false, false, false, false, false],
                vec![false, false, false, false, false, false],
            ],
            vec![
                vec![false, false, false, false, false, false],
                vec![false, false, true, false, false, false],
                vec![false, false, false, true, false, false],
                vec![false, true, true, true, false, false],
                vec![false, false, false, false, false, false],
                vec![false, false, false, false, false, false],
            ]];

        let mut current_state: CellData = vec![
            vec![false, true, false, false, false, false],
            vec![false, false, true, false, false, false],
            vec![true, true, true, false, false, false],
            vec![false, false, false, false, false, false],
            vec![false, false, false, false, false, false],
            vec![false, false, false, false, false, false],
        ];

        for (step, expected) in glider_states.iter().enumerate() {
            current_state = solver.compute_state(&current_state);
            assert_eq!(
                *expected,
                current_state,
                "State no longer resembles a glider after {} steps(s)",
                step + 1,
            );
        }
    }

    fn assert_dead_grid_stays_dead<S: Solver>(solver: S) {
        let current_state: CellData = vec![vec![false; 10]; 10];
        assert_eq!(
            current_state,
            solver.compute_state(&current_state),
            "Dead grid should stay dead"
        );
    }

    fn assert_alive_grid_dies<S: Solver>(solver: S) {
        let current_state: CellData = vec![vec![true; 10]; 10];
        assert_eq!(
            vec![vec![false; 10]; 10],
            solver.compute_state(&current_state),
            "Not all cells died"
        );
    }

    fn assert_four_block_still_life<S: Solver>(solver: S) {
        let current_state: CellData = vec![
            vec![false, false, false, false],
            vec![false, true, true, false],
            vec![false, true, true, false],
            vec![false, false, false, false],
        ];
        assert_eq!(
            current_state,
            solver.compute_state(&current_state),
            "Four block still shouldn't change"
        );
    }
}
