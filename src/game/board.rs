use super::piece::{Piece, TetrominoType};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    cells: [[Option<TetrominoType>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    pub fn fits(&self, piece: &Piece) -> bool {
        for (x, y) in piece.blocks() {
            if x < 0 || x >= BOARD_WIDTH as i32 {
                return false;
            }
            if y >= BOARD_HEIGHT as i32 {
                return false;
            }
            if y < 0 {
                continue;
            }
            if self.cells[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }

    pub fn lock_piece(&mut self, piece: &Piece) {
        for (x, y) in piece.blocks() {
            if y < 0 || y >= BOARD_HEIGHT as i32 {
                continue;
            }
            if x < 0 || x >= BOARD_WIDTH as i32 {
                continue;
            }
            self.cells[y as usize][x as usize] = Some(piece.kind);
        }
    }

    pub fn clear_full_lines(&mut self) -> u32 {
        let mut cleared = 0;
        let mut new_cells = [[None; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut write_row = BOARD_HEIGHT;

        for y in (0..BOARD_HEIGHT).rev() {
            if self.cells[y].iter().all(|cell| cell.is_some()) {
                cleared += 1;
            } else {
                write_row -= 1;
                new_cells[write_row] = self.cells[y];
            }
        }

        for y in 0..write_row {
            new_cells[y] = [None; BOARD_WIDTH];
        }

        self.cells = new_cells;
        cleared
    }

    pub fn merged_with_piece(
        &self,
        piece: &Piece,
    ) -> [[Option<TetrominoType>; BOARD_WIDTH]; BOARD_HEIGHT] {
        let mut merged = self.cells;
        for (x, y) in piece.blocks() {
            if y < 0 || y >= BOARD_HEIGHT as i32 {
                continue;
            }
            if x < 0 || x >= BOARD_WIDTH as i32 {
                continue;
            }
            merged[y as usize][x as usize] = Some(piece.kind);
        }
        merged
    }
}
