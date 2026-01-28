use crate::life_game::CellData;

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

fn build(state: &mut CellData, x: usize, y: usize, build_list: Vec<(usize, usize)>) {
    let width = state[0].len();
    let height = state.len();

    let x = x % width;
    let y = y % height;

    for coords in build_list.iter() {
        state[(y + coords.0) % height][(x + coords.1) % width] = true;
    }
}
