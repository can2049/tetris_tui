use super::{board::Board, piece::Piece};

pub struct GameState {
    pub board: Board,
    pub current: Piece,
    pub next: Piece,
    pub score: u32,
    pub lines: u32,
    pub game_over: bool,
    pub paused: bool,
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        let current = Piece::random();
        let next = Piece::random();
        let mut state = Self {
            board,
            current,
            next,
            score: 0,
            lines: 0,
            game_over: false,
            paused: false,
        };

        if !state.board.fits(&state.current) {
            state.game_over = true;
        }

        state
    }

    pub fn reset(&mut self) {
        *self = GameState::new();
    }

    pub fn tick(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        if !self.try_shift(0, 1) {
            self.lock_current();
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.game_over {
            return;
        }
        self.paused = !self.paused;
    }

    pub fn move_horizontal(&mut self, delta: i32) {
        if self.game_over || self.paused {
            return;
        }
        self.try_shift(delta, 0);
    }

    pub fn soft_drop(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        if self.try_shift(0, 1) {
            self.score = self.score.saturating_add(1);
        }
    }

    pub fn hard_drop(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        let mut distance = 0;
        while self.try_shift(0, 1) {
            distance += 1;
        }

        if distance > 0 {
            self.score = self.score.saturating_add(distance * 2);
        }

        self.lock_current();
    }

    pub fn rotate(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        let rotated = self.current.rotated();
        let kicks = [0, -1, 1, -2, 2];
        for dx in kicks {
            let candidate = rotated.shifted(dx, 0);
            if self.board.fits(&candidate) {
                self.current = candidate;
                break;
            }
        }
    }

    fn try_shift(&mut self, dx: i32, dy: i32) -> bool {
        let candidate = self.current.shifted(dx, dy);
        if self.board.fits(&candidate) {
            self.current = candidate;
            true
        } else {
            false
        }
    }

    fn lock_current(&mut self) {
        self.board.lock_piece(&self.current);
        let cleared = self.board.clear_full_lines();
        if cleared > 0 {
            self.lines = self.lines.saturating_add(cleared);
            self.score = self.score.saturating_add(match cleared {
                1 => 100,
                2 => 250,
                3 => 500,
                4 => 800,
                _ => cleared * 200,
            });
        }
        self.spawn_piece();
    }

    fn spawn_piece(&mut self) {
        self.current = self.next;
        self.current.reset_position();
        self.next = Piece::random();

        if !self.board.fits(&self.current) {
            self.game_over = true;
        }
    }
}
