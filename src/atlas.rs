use macroquad::prelude::*;

use crate::constants::*;
use crate::noise::rand01;

pub const TILE: u32 = 16;
pub const ATLAS_W: u32 = TILE * 4;
pub const ATLAS_H: u32 = TILE * 4;
pub const T_SKIN: u8 = 10;
pub const T_SLEEVE: u8 = 11;

pub fn make_atlas() -> Image {
    let mut bytes = vec![0u8; (ATLAS_W * ATLAS_H * 4) as usize];

    for tile in 0..12u32 {
        let tx = (tile % 4) * TILE;
        let ty = (tile / 4) * TILE;

        for py in 0..TILE {
            for px in 0..TILE {
                // Делаем шум КРУПНЫМ (пятна 2x2 пикселя), как в Minecraft
                let n_clump = rand01(px as i32 / 2, py as i32 / 2, 100 + tile);
                // Совсем слабый шум для детализации (1x1)
                let n_detail = rand01(px as i32, py as i32, 200 + tile);

                let (r, g, b): (f32, f32, f32) = match tile {
                    0 => {
                        // Трава (верх) - мягкий перелив
                        let v = 0.9 + 0.2 * n_clump + 0.05 * n_detail;
                        (0.25 * v, 0.60 * v, 0.20 * v)
                    }
                    1 => {
                        // Трава (бок)
                        let edge = 4 + (rand01(px as i32 / 2, 0, 55) * 2.0) as u32;
                        if py < edge {
                            let v = 0.9 + 0.2 * n_clump + 0.05 * n_detail;
                            (0.25 * v, 0.60 * v, 0.20 * v)
                        } else {
                            let v = 0.9 + 0.2 * n_clump + 0.05 * n_detail;
                            (0.40 * v, 0.28 * v, 0.18 * v)
                        }
                    }
                    2 => {
                        // Земля
                        let v = 0.9 + 0.2 * n_clump + 0.05 * n_detail;
                        (0.40 * v, 0.28 * v, 0.18 * v)
                    }
                    3 => {
                        // Камень
                        let v = 0.9 + 0.2 * n_clump + 0.05 * n_detail;
                        (0.45 * v, 0.45 * v, 0.48 * v)
                    }
                    4 => {
                        // Дерево (бок)
                        let stripe = if px % 4 == 0 { 0.8 } else { 1.0 };
                        let v = (0.9 + 0.2 * n_clump + 0.05 * n_detail) * stripe;
                        (0.38 * v, 0.26 * v, 0.14 * v)
                    }
                    5 => {
                        // Дерево (верх)
                        let d = ((px as f32 - 7.5).powi(2) + (py as f32 - 7.5).powi(2)).sqrt();
                        let ring = if (d as u32) % 3 == 0 { 0.8 } else { 1.0 };
                        let v = (0.9 + 0.2 * n_clump) * ring;
                        (0.48 * v, 0.36 * v, 0.20 * v)
                    }
                    6 => {
                        // Листва
                        let v = 0.85 + 0.3 * n_clump + 0.1 * n_detail;
                        (0.15 * v, 0.45 * v, 0.12 * v)
                    }
                    7 => {
                        // Доски
                        let board = if py % 4 == 0 { 0.7 } else { 1.0 };
                        let v = (0.9 + 0.2 * n_clump + 0.05 * n_detail) * board;
                        (0.65 * v, 0.50 * v, 0.30 * v)
                    }
                    8 => {
                        // Песок
                        let v = 0.9 + 0.2 * n_clump + 0.05 * n_detail;
                        (0.85 * v, 0.80 * v, 0.60 * v)
                    }
                    9 => {
                        // Кирпич
                        let mortar = py % 4 == 0 || (px + (py / 4) * 4) % 8 == 0;
                        if mortar {
                            (0.65, 0.65, 0.65)
                        } else {
                            let v = 0.9 + 0.2 * n_clump;
                            (0.60 * v, 0.25 * v, 0.20 * v)
                        }
                    }
                    10 => {
                        // Кожа руки
                        (0.95, 0.75, 0.65)
                    }
                    _ => {
                        // Рукав
                        (0.10, 0.40, 0.60)
                    }
                };

                let x = tx + px;
                let y = ty + py;
                let i = ((y * ATLAS_W + x) * 4) as usize;

                bytes[i] = (r.clamp(0.0, 1.0) * 255.0) as u8;
                bytes[i + 1] = (g.clamp(0.0, 1.0) * 255.0) as u8;
                bytes[i + 2] = (b.clamp(0.0, 1.0) * 255.0) as u8;
                bytes[i + 3] = 255;
            }
        }
    }

    Image {
        width: ATLAS_W as u16,
        height: ATLAS_H as u16,
        bytes,
    }
}

pub fn tile_uv(tile: u8) -> (f32, f32, f32, f32) {
    let col = (tile % 4) as f32;
    let row = (tile / 4) as f32;
    let inset = 0.5 / ATLAS_W as f32;
    let inset_v = 0.5 / ATLAS_H as f32;

    (
        col * 0.25 + inset,
        (col + 1.0) * 0.25 - inset,
        row * 0.25 + inset_v,
        (row + 1.0) * 0.25 - inset_v,
    )
}

pub fn face_tile(b: u8, face: usize) -> u8 {
    match b {
        GRASS => match face {
            2 => 0,
            3 => 2,
            _ => 1,
        },
        DIRT => 2,
        STONE => 3,
        WOOD => match face {
            2 | 3 => 5,
            _ => 4,
        },
        LEAVES => 6,
        PLANKS => 7,
        SAND => 8,
        BRICK => 9,
        _ => 3,
    }
}