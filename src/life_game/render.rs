use crate::life_game::{CellData, Coordinates, Dimensions, Game};

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

    fn map_chunk_to_character_index(&self, cells: &CellData) -> usize {
        // Work out the character index that this chunk of cells will map to.  We treat the chunk
        // as a group of bits, starting with the LSB in the top-left, then working right and down
        // until we get to the MSB in the bottom-right of the chunk.
        let mut result:usize = 0;

        for row_index in 0 .. cells.len() {
            for column_index in 0 .. cells[row_index].len() {
                let cell_index = (row_index * self.columns_per_symbol) + column_index;
                let cell_is_alive = cells[row_index][column_index];

                if cell_is_alive {
                    let bit_value = usize::pow(2, cell_index as u32);
                    result |= bit_value;
                }
            }
        }

        result
    }

    fn render_cells(&self, game: &Game, output: &mut String) {
        let cell_chunk_dimensions = Dimensions { width: self.columns_per_symbol, height: self.rows_per_symbol };
        let mut coordinates = Coordinates { x: 0, y: 0 };

        for row in (0 .. game.dimensions.height).step_by(self.rows_per_symbol) {
            output.push('┃');
            for column in (0 .. game.dimensions.width).step_by(self.columns_per_symbol) {
                coordinates.x = column;
                coordinates.y = row;
                let char_to_use = self.map_chunk_to_character_index(
                    &game.get_cell_chunk_at(&coordinates, &cell_chunk_dimensions)
                );
                output.push(*self.symbol_map.get(char_to_use).unwrap_or(&'?'));
            }
            output.push_str("┃\n");
        }
    }
}

impl Renderer for CharacterMapRenderer {
    fn render(&self, game: &Game) -> String {
        // Rendering dimensions should be divided by the game grid dimensions but rounded up
        let render_width = game.dimensions.width.div_ceil(self.columns_per_symbol);
        let render_height = game.dimensions.height.div_ceil(self.rows_per_symbol);

        let border = std::iter::repeat_n("━", render_width).collect::<String>();

        // Output buffer should allocate 4 bytes for each cell in the grid plus 2 extra rows and columns
        let mut output = String::with_capacity(
            ((render_width + 2) * (render_height + 2)) * 4
        );

        output.push('┏');
        output.push_str(&border);
        output.push_str("┓\n");

        self.render_cells(game, &mut output);

        output.push('┗');
        output.push_str(&border);
        output.push_str("┛\n");

        output
    }
}
