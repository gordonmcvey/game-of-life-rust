use crate::life_game::Game;

pub trait Renderer {
    fn render(&self, game: &Game) -> String;
}

pub struct CharacterMapRenderer {
    symbol_map: Vec<char>,
    rows_per_symbol: usize,
    columns_per_symbol: usize,
}

impl CharacterMapRenderer {
    pub fn new(symbol_map: Vec<char>, rows_per_symbol: usize, columns_per_symbol: usize) -> Self {
        CharacterMapRenderer {
            symbol_map,
            rows_per_symbol,
            columns_per_symbol,
        }
    }

    pub fn single_cell_per_char() -> Self {
        Self::new(vec![' ', '█'], 1,1, )
    }

    pub fn two_cells_per_char() -> Self {
        Self::new(
            vec![' ','▀', '▄', '█'],
            2,
            1,
        )
    }

    pub fn four_cells_per_char() -> Self {
        Self::new(
            vec![
                ' ', '▘', '▝', '▀',
                '▖', '▌', '▞', '▛',
                '▗', '▚', '▐', '▜',
                '▄', '▙', '▟', '█'
            ],
            2,
            2,
        )
    }

    pub fn eight_cells_per_char() -> Self {
        Self::new(
            vec![
                '⠀', '⠁', '⠈', '⠉',
                '⠂', '⠃', '⠊', '⠋',
                '⠐', '⠑', '⠘', '⠙',
                '⠒', '⠓', '⠚', '⠛',

                '⠄', '⠅', '⠌', '⠍',
                '⠆', '⠇', '⠎', '⠏',
                '⠔', '⠕', '⠜', '⠝',
                '⠖', '⠗', '⠞', '⠟',

                '⠠', '⠡', '⠨', '⠩',
                '⠢', '⠣', '⠪', '⠫',
                '⠰', '⠱', '⠸', '⠹',
                '⠲', '⠳', '⠺', '⠻',

                '⠤', '⠥', '⠬', '⠭',
                '⠦', '⠧', '⠮', '⠯',
                '⠴', '⠵', '⠼', '⠽',
                '⠶', '⠷', '⠾', '⠿',

                '⡀', '⡁', '⡈', '⡉',
                '⡂', '⡃', '⡊', '⡋',
                '⡐', '⡑', '⡘', '⡙',
                '⡒', '⡓', '⡚', '⡛',

                '⡄', '⡅', '⡌', '⡍',
                '⡆', '⡇', '⡎', '⡏',
                '⡔', '⡕', '⡜', '⡝',
                '⡖', '⡗', '⡞', '⡟',

                '⡠', '⡡', '⡨', '⡩',
                '⡢', '⡣', '⡪', '⡫',
                '⡰', '⡱', '⡸', '⡹',
                '⡲', '⡳', '⡺', '⡻',

                '⡤', '⡥', '⡬', '⡭',
                '⡦', '⡧', '⡮', '⡯',
                '⡴', '⡵', '⡼', '⡽',
                '⡶', '⡷', '⡾', '⡿',

                '⢀', '⢁', '⢈', '⢉',
                '⢂', '⢃', '⢊', '⢋',
                '⢐', '⢑', '⢘', '⢙',
                '⢒', '⢓', '⢚', '⢛',

                '⢄', '⢅', '⢌', '⢍',
                '⢆', '⢇', '⢎', '⢏',
                '⢔', '⢕', '⢜', '⢝',
                '⢖', '⢗', '⢞', '⢟',

                '⢠', '⢡', '⢨', '⢩',
                '⢢', '⢣', '⢪', '⢫',
                '⢰', '⢱', '⢸', '⢹',
                '⢲', '⢳', '⢺', '⢻',

                '⢤', '⢥', '⢬', '⢭',
                '⢦', '⢧', '⢮', '⢯',
                '⢴', '⢵', '⢼', '⢽',
                '⢶', '⢷', '⢾', '⢿',

                '⣀', '⣁', '⣈', '⣉',
                '⣂', '⣃', '⣊', '⣋',
                '⣐', '⣑', '⣘', '⣙',
                '⣒', '⣓', '⣚', '⣛',

                '⣄', '⣅', '⣌', '⣍',
                '⣆', '⣇', '⣎', '⣏',
                '⣔', '⣕', '⣜', '⣝',
                '⣖', '⣗', '⣞', '⣟',

                '⣠', '⣡', '⣨', '⣩',
                '⣢', '⣣', '⣪', '⣫',
                '⣰', '⣱', '⣸', '⣹',
                '⣲', '⣳', '⣺', '⣻',

                '⣤', '⣥', '⣬', '⣭',
                '⣦', '⣧', '⣮', '⣯',
                '⣴', '⣵', '⣼', '⣽',
                '⣶', '⣷', '⣾', '⣿',
            ],
            4,
            2,
        )
    }

    // @todo This should be 2 methods (one for finding the relevant cells and another to map the live cells to the appropriate symbol)
    // Will take a bit of figuring out some stuff regarding lifetimes and other oddities to get it to actually work like that.
    fn cells_at(&self, game: &Game, x: usize, y: usize) -> usize {
        // Get a group of cells starting at the x-y coordinates given
        let cells: Vec<Vec<&bool>> = game.game_state.iter()
            .skip(y)
            .take(self.rows_per_symbol)
            .map(
                |row|
                    row.iter()
                    .skip(x)
                    .take(self.columns_per_symbol)
                    .collect()
                )
            .collect();

        let mut result:usize = 0;

        // Determine the vector index value to use for the given block of cell state.  Treat the
        // top left cell as the LSB, then work right across the columns and down across the rows and
        // add the relevant bit value to the output
        for row_index in 0 .. cells.len() {
            for column_index in 0 .. cells[row_index].len() {
                let cell_index = (row_index * self.columns_per_symbol) + column_index;
                let cell_is_alive = *cells[row_index][column_index];

                if cell_is_alive {
                    let bit_value = usize::pow(2, cell_index as u32);
                    result |= bit_value;
                }
            }
        }

        result
    }
}

impl Renderer for CharacterMapRenderer {
    fn render(&self, game: &Game) -> String {
        // Rendering dimensions should be divided by the game grid dimensions but rounded up
        let render_width = game.width.div_ceil(self.columns_per_symbol);
        let render_height = game.height.div_ceil(self.rows_per_symbol);

        let border = std::iter::repeat_n("━", render_width).collect::<String>();

        // Output buffer should allocate 4 bytes for each cell in the grid plus 2 extra rows and columns
        let mut output = String::with_capacity(
            ((render_width + 2) * (render_height + 2)) * 4
        );

        output.push('┏');
        output.push_str(&border);
        output.push_str("┓\n");

        for row in (0 .. game.height).step_by(self.rows_per_symbol) {
            output.push('┃');
            for column in (0 .. game.width).step_by(self.columns_per_symbol) {
                let char_to_use = self.cells_at(game, column, row);
                let cell_output = self.symbol_map.get(char_to_use).unwrap_or(&'?');
                output.push(*cell_output);
            }
            output.push_str("┃\n");
        }

        output.push('┗');
        output.push_str(&border);
        output.push_str("┛\n");

        output
    }
}
