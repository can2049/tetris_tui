use rand::{seq::SliceRandom, thread_rng};

pub const SPAWN_X: i32 = 3;
pub const SPAWN_Y: i32 = 0;
pub const ROTATIONS: u8 = 4;

pub type BlockOffset = (i32, i32);

const SHAPES: [[[BlockOffset; 4]; ROTATIONS as usize]; 7] = [
    // I
    [
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],
    // O
    [
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (2, 1)],
    ],
    // T
    [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (0, 1), (1, 1), (1, 2)],
    ],
    // S
    [
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(1, 1), (2, 1), (0, 2), (1, 2)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ],
    // Z
    [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(2, 0), (1, 1), (2, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 0), (0, 1), (1, 1), (0, 2)],
    ],
    // J
    [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (2, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (1, 2), (0, 2)],
    ],
    // L
    [
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TetrominoType {
    I = 0,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    pub const ALL: [TetrominoType; 7] = [
        TetrominoType::I,
        TetrominoType::O,
        TetrominoType::T,
        TetrominoType::S,
        TetrominoType::Z,
        TetrominoType::J,
        TetrominoType::L,
    ];
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: TetrominoType,
    pub rotation: u8,
    pub x: i32,
    pub y: i32,
}

impl Piece {
    pub fn new(kind: TetrominoType) -> Self {
        Self {
            kind,
            rotation: 0,
            x: SPAWN_X,
            y: SPAWN_Y,
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let kind = *TetrominoType::ALL
            .choose(&mut rng)
            .unwrap_or(&TetrominoType::I);
        Self::new(kind)
    }

    pub fn offsets(&self) -> [BlockOffset; 4] {
        SHAPES[self.kind as usize][self.rotation as usize]
    }

    pub fn blocks(&self) -> [BlockOffset; 4] {
        self.offsets().map(|(dx, dy)| (self.x + dx, self.y + dy))
    }

    pub fn rotated(&self) -> Self {
        let mut clone = *self;
        clone.rotation = (clone.rotation + 1) % ROTATIONS;
        clone
    }

    pub fn shifted(&self, dx: i32, dy: i32) -> Self {
        let mut clone = *self;
        clone.x += dx;
        clone.y += dy;
        clone
    }

    pub fn reset_position(&mut self) {
        self.x = SPAWN_X;
        self.y = SPAWN_Y;
        self.rotation = 0;
    }
}
