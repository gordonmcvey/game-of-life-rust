use crate::life_game::CellData;

pub fn glider(state: &mut CellData, x: usize, y: usize) {
    let width = state.len();
    let height = state[0].len();

    let x = x % width;
    let y = y % height;
    
    state[x][(y + 1) % height] = true;
    state[(x + 1) % width][(y + 2) % height] = true;
    state[(x + 2) % width][y] = true;
    state[(x + 2) % width][(y + 1) % height] = true;
    state[(x + 2) % width][(y + 2) % height] = true;
}

pub fn pentadecathlon(state: &mut CellData, x: usize, y: usize) {
    let width = state.len();
    let height = state[0].len();
    
    let x = x % width;
    let y = y % height;

    state[x][(y + 2) % height] = true;
    state[x][(y + 7) % height] = true;
    
    state[(x + 1) % width][y] = true;
    state[(x + 1) % width][(y + 1) % height] = true;
    state[(x + 1) % width][(y + 3) % height] = true;
    state[(x + 1) % width][(y + 4) % height] = true;
    state[(x + 1) % width][(y + 5) % height] = true;
    state[(x + 1) % width][(y + 6) % height] = true;
    state[(x + 1) % width][(y + 8) % height] = true;
    state[(x + 1) % width][(y + 9) % height] = true;
    
    state[(x + 2) % width][(y + 2) % height] = true;
    state[(x + 2) % width][(y + 7) % height] = true;
}
