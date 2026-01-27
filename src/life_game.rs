use std::fmt::Display;

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

    pub fn from_data(cells: CellData) -> Self {
        Game {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }

    pub fn step(&mut self) {
        let mut new_state = vec![vec![false; self.width]; self.height];

        for row in 0..self.height {
            for column in 0..self.width {
                let is_alive = self.cells[row][column];
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

        self.cells = new_state;
    }

    fn get_living_neighbour_count(&self, row: usize, column: usize) -> u8 {
        let above = (row + self.height - 1) % self.height;
        let below = (row + 1) % self.height;
        let left = (column + self.width - 1) % self.width;
        let right = (column + 1) % self.width;

        let count = if self.cells[above][left] { 1 } else { 0 }
            + if self.cells[above][column] { 1 } else { 0 }
            + if self.cells[above][right] { 1 } else { 0 }
            + if self.cells[row][left] { 1 } else { 0 }
            + if self.cells[row][right] { 1 } else { 0 }
            + if self.cells[below][left] { 1 } else { 0 }
            + if self.cells[below][column] { 1 } else { 0 }
            + if self.cells[below][right] { 1 } else { 0 };

        count
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border = std::iter::repeat("-").take(self.width).collect::<String>();
        let mut output = String::with_capacity(((self.width + 2) * (self.height + 2)) * 2);

        output.push_str("+");
        output.push_str(&border);
        output.push_str("+\n");

        for row in self.cells.iter() {
            output.push_str("|");
            for cell in row.iter() {
                let cell_output = {
                    if *cell {
                        String::from("#")
                    } else {
                        String::from(" ")
                    }
                };

                output.push_str(format!("{}", cell_output).as_str());
            }
            output.push_str("|\n");
        }

        output.push_str("+");
        output.push_str(&border);
        output.push_str("+\n");

        write!(f, "{}", output)
    }
}
