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

pub fn lightweight_spaceship(state: &mut CellData, x: usize, y: usize) {
    let build_list: Vec<(usize, usize)> = vec!(
        (0, 1),
        (1, 0),
        (2, 0), (2, 4),
        (3, 0), (3, 1), (3, 2), (3, 3),
    );

    build(state, x, y, build_list)
}

pub fn achim_p144(state: &mut CellData, x: usize, y: usize) {
    let build_list: Vec<(usize, usize)> = vec!(
        (0, 0), (0, 1), (0, 26), (0, 27),
        (1, 0), (1, 1), (1, 26), (1, 27),

        (2, 18), (2, 19),
        (3, 17), (3, 20),
        (4, 18), (4, 19),

        (5, 14),
        (6, 13), (6, 15),
        (7, 12), (7, 16),
        (8, 12), (8, 15),

        (10, 12), (10, 15),
        (11, 11), (11, 15),
        (12, 12), (12, 14),
        (13, 13),

        (14, 8), (14, 9),
        (15, 7), (15, 10),
        (16, 8), (16, 9),

        (17, 0), (17, 1), (17, 26), (17, 27),
        (18, 0), (18, 1), (18, 26), (18, 27),
    );

    build(state, x, y, build_list)
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
