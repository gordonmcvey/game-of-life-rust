use crate::life_game::CellData;
use rand::RngExt;

pub fn glider(state: &mut CellData, x: usize, y: usize) {
    let build_list: Vec<(usize, usize)> = vec!(
        (0, 1), (1, 2), (2, 0), (2, 1), (2, 2),
    );

    build(state, x, y, build_list);
}

pub fn pentadecathlon(state: &mut CellData, x: usize, y: usize) {
    let build_list: Vec<(usize, usize)> = vec!(
        (0, 2), (0, 7),
        (1, 0), (1, 1), (1, 3), (1, 4), (1, 5), (1, 6), (1, 8), (1, 9),
        (2, 2), (2, 7),
    );

    build(state, x, y, build_list);
}

pub fn u(state: &mut CellData, x: usize, y: usize) {
    let build_list: Vec<(usize, usize)> = vec!(
        (0, 0), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2),
    );
    
    build(state, x, y, build_list);
}

pub fn randomise(state: &mut CellData, probability: u32) {
    match probability {
        0..=100 => (),
        _ => panic!("Probability must be between 0 and 100")
    }

    let mut rng = rand::rng();
    let mut random_number: u32;

    for row in state.iter_mut() {
        for cell in row.iter_mut() {
            random_number = rng.random_range(0..100);
            if random_number <= probability {
                *cell = true;
            }
        }
    }
}

fn build(state: &mut CellData, x: usize, y: usize, build_list: Vec<(usize, usize)>) {
    let width = state[0].len();
    let height = state.len();

    let x = x % width;
    let y = y % height;

    for coords in build_list.iter() {
        state[(y + coords.0) % height][(x + coords.1) % width] = true;
    }
}
