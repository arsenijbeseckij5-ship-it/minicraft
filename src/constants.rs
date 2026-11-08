pub const CH: i32 = 16;
pub const SY: i32 = 48;
pub const R: i32 = 4;
pub const AIR: u8 = 0;
pub const GRASS: u8 = 1;
pub const DIRT: u8 = 2;
pub const STONE: u8 = 3;
pub const WOOD: u8 = 4;
pub const LEAVES: u8 = 5;
pub const PLANKS: u8 = 6;
pub const SAND: u8 = 7;
pub const BRICK: u8 = 8;

pub const BLOCK_NAMES: [&str; 9] = [
    "Air", "Grass", "Dirt", "Stone", "Wood", "Leaves", "Planks", "Sand", "Brick",
];

pub const HOTBAR: [u8; 9] = [GRASS, DIRT, STONE, WOOD, LEAVES, PLANKS, SAND, BRICK, AIR];

// Максимум чанков для перестройки за кадр (предотвращает фризы)
pub const MAX_CHUNKS_PER_FRAME: usize = 2;