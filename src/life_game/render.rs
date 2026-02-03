use crate::life_game::{CellData, Game};

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

    fn get_cell_chunk_at(&self, game: &Game, x: usize, y: usize) -> CellData {
        // get a chunk of cells from the game state at the given co-ordinates.  The chunk will be a
        // group of cells that can be mapped onto a display character (eg if one character can
        // represent a group of 2x4 cells, then take 8 cells total, as two columns of 4 rows each
        game.game_state.iter()
            .skip(y)
            .take(self.rows_per_symbol)
            .map(|row|
                    row.iter()
                        .skip(x)
                        .take(self.columns_per_symbol)
                        .copied()  // Dereference the bool reference
                        .collect::<Vec<bool>>())
            .collect()
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
                let char_to_use = self.map_chunk_to_character_index(
                    &self.get_cell_chunk_at(game, column, row)
                );
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
