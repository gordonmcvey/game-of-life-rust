use crate::life_game::{CellData, Game};
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub type SolverBox = Box<dyn Solver>;
type Job = Box<dyn FnOnce() -> CellData + Send + 'static>;

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
    result_receiver: Mutex<mpsc::Receiver<CellData>>,
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

impl ThreadPoolSolver {

    pub fn new(pool_size: usize) -> Self {
        // Channel initialisation
        let (job_tx, job_rx) = mpsc::channel::<Option<Job>>();
        let (result_tx, result_rx) = mpsc::channel::<CellData>();

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
                    let job = job_receiver
                        .lock()
                        .unwrap()
                        .recv()
                        .unwrap()
                    ;

                    match job {
                        Some(job) => {
                            let job_result = job();
                            // NOTE: Use of unwrap() here may require some thought.
                            result_sender.send(job_result).unwrap();
                        },
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

impl Solver for ThreadPoolSolver {
    fn compute_state(&self, game: &Game) -> CellData {
        todo!()
    }
}
