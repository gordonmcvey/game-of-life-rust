type CellData = Vec<Vec<bool>>;

pub struct Game {
    width: usize,
    height: usize,
    cells: CellData,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Game {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    pub fn from_data (cells: CellData) -> Self {
        Game {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }

    pub fn step(&mut self) {
        let mut new_state = vec![vec![false; self.width]; self.height];

        for row in 0 .. self.height {
            for column in 0 .. self.width {
                
            }
        }

        self.cells = new_state;
    }
}
