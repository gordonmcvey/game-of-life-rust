use colored::Colorize;
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
                // println!("{} {}", row, column);
            }
        }

        self.cells = new_state;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for row in self.cells.iter() {
            for cell in row.iter() {
                let cell_output = {
                    if *cell {
                        String::from(" ").on_bright_white()
                    } else {
                        String::from(" ").on_bright_black()
                    }
                };

                output.push_str(format!("{}", cell_output).as_str());
            }
            output.push_str("\n");
        }

        write!(f, "{}", output)?;
        Ok(())
    }
}
