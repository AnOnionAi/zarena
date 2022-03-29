use super::piece::ManPiece;
use super::player::Player;
use super::tile::{EmptyTile, OccupiedTile, Tile};

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub struct BoardPosition {
    pub row: usize,
    pub column: usize,
}

impl BoardPosition {
    pub fn new(row: usize, column: usize) -> BoardPosition {
        BoardPosition {
            row: row,
            column: column,
        }
    }
}

pub struct Board {
    number_rows: usize,
    number_columns: usize,
    tiles: Vec<Box<dyn Tile>>,
}

const CHECKERBOARD_SIZE: usize = 8;
const CHECKERS_NUMBER_TILES: usize = CHECKERBOARD_SIZE * CHECKERBOARD_SIZE;

impl Board {
    // #[cfg(test)]
    pub fn new(number_rows: usize, number_columns: usize) -> Board {
        let number_tiles = number_rows * number_columns;
        let mut board = Board {
            number_rows: number_rows,
            number_columns: number_columns,
            tiles: Vec::with_capacity(number_tiles),
        };

        for _ in 0..number_tiles {
            board.tiles.push(Box::new(EmptyTile));
        }

        board
    }

    pub fn set_tiles(&mut self, tiles: Vec<Box<dyn Tile>>) {
        self.tiles = tiles;
    }

    pub fn new_checkerboard(player1: &Player, player2: &Player) -> Board {
        if player1.id == player2.id {
            panic!("Player 1 and Player 2 have the same ID: {}", player1.id)
        }

        let mut board = Board {
            number_rows: CHECKERBOARD_SIZE,
            number_columns: CHECKERBOARD_SIZE,
            tiles: Vec::with_capacity(CHECKERS_NUMBER_TILES),
        };

        Board::fill_even_row(&mut board, player1);
        Board::fill_odd_row(&mut board, player1);
        Board::fill_even_row(&mut board, player1);

        Board::fill_empty_row(&mut board);
        Board::fill_empty_row(&mut board);

        Board::fill_odd_row(&mut board, player2);
        Board::fill_even_row(&mut board, player2);
        Board::fill_odd_row(&mut board, player2);

        board
    }

    pub fn number_rows(&self) -> usize {
        self.number_rows
    }

    pub fn number_columns(&self) -> usize {
        self.number_columns
    }

    fn indices_to_index(&self, row: usize, column: usize) -> usize {
        self.number_columns * row + column
    }

    pub fn get_tile(&self, row: usize, column: usize) -> &dyn Tile {
        let idx = self.indices_to_index(row, column);
        &*self.tiles[idx]
    }

    pub fn set_tile(&mut self, row: usize, column: usize, tile: Box<dyn Tile>) {
        let idx = self.indices_to_index(row, column);
        self.tiles[idx] = tile;
    }

    pub fn clear_tile(&mut self, row: usize, column: usize) {
        self.set_tile(row, column, Box::new(EmptyTile));
    }

    pub fn swap_tiles(&mut self, row1: usize, column1: usize, row2: usize, column2: usize) {
        let idx1 = self.indices_to_index(row1, column1);
        let idx2 = self.indices_to_index(row2, column2);
        self.tiles.swap(idx1, idx2);
    }

    fn fill_even_row(board: &mut Board, player: &Player) {
        for t in 0..board.number_columns {
            let tile: Box<dyn Tile> = if t % 2 == 0 {
                let piece = ManPiece::new(player);
                Box::new(OccupiedTile::new(Box::new(piece)))
            } else {
                Box::new(EmptyTile)
            };
            board.tiles.push(tile);
        }
    }

    fn fill_odd_row(board: &mut Board, player: &Player) {
        for t in 0..board.number_columns {
            let tile: Box<dyn Tile> = if t % 2 == 1 {
                let piece = ManPiece::new(player);
                Box::new(OccupiedTile::new(Box::new(piece)))
            } else {
                Box::new(EmptyTile)
            };
            board.tiles.push(tile);
        }
    }

    fn fill_empty_row(board: &mut Board) {
        for _ in 0..board.number_columns {
            board.tiles.push(Box::new(EmptyTile));
        }
    }
}
