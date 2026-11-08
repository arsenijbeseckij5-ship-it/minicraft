use std::collections::HashMap;
use macroquad::prelude::*;
use crate::constants::*;
use crate::noise::{hash2, height_at};

pub struct Chunk {
    pub blocks: Vec<u8>,
    pub meshes: Vec<Mesh>,
    pub dirty: bool,
}

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub modifications: HashMap<(i32, i32, i32), u8>, // Сохраняем изменения блоков
    pub seed: u32,
}

impl World {
    pub fn new(seed: u32) -> World {
        World {
            chunks: HashMap::new(),
            modifications: HashMap::new(),
            seed,
        }
    }
    
    pub(crate) fn idx(lx: i32, y: i32, lz: i32) -> usize {
        ((y * CH + lz) * CH + lx) as usize
    }
    
    pub fn get(&self, x: i32, y: i32, z: i32) -> u8 {
        if y < 0 || y >= SY {
            return AIR;
        }
        
        // Сначала проверяем модификации
        if let Some(&b) = self.modifications.get(&(x, y, z)) {
            return b;
        }
        
        let (cx, cz) = (x.div_euclid(CH), z.div_euclid(CH));
        match self.chunks.get(&(cx, cz)) {
            Some(c) => c.blocks[Self::idx(x.rem_euclid(CH), y, z.rem_euclid(CH))],
            None => AIR,
        }
    }
    
    pub fn occludes(&self, x: i32, y: i32, z: i32) -> bool {
        if y < 0 {
            return true;
        }
        self.get(x, y, z) != AIR
    }
    
    pub fn solid(&self, x: i32, y: i32, z: i32) -> bool {
        if y < 0 {
            return true;
        }
        self.get(x, y, z) != AIR
    }
    
    pub fn set(&mut self, x: i32, y: i32, z: i32, b: u8) {
        if y < 0 || y >= SY {
            return;
        }
        
        // Сохраняем модификацию
        self.modifications.insert((x, y, z), b);
        
        let (cx, cz) = (x.div_euclid(CH), z.div_euclid(CH));
        let (lx, lz) = (x.rem_euclid(CH), z.rem_euclid(CH));
        
        if let Some(c) = self.chunks.get_mut(&(cx, cz)) {
            c.blocks[Self::idx(lx, y, lz)] = b;
            c.dirty = true;
        }
        
        // Помечаем соседние чанки как dirty
        let mut neighbors = Vec::new();
        if lx == 0 {
            neighbors.push((cx - 1, cz));
        }
        if lx == CH - 1 {
            neighbors.push((cx + 1, cz));
        }
        if lz == 0 {
            neighbors.push((cx, cz - 1));
        }
        if lz == CH - 1 {
            neighbors.push((cx, cz + 1));
        }
        
        for k in neighbors {
            if let Some(c) = self.chunks.get_mut(&k) {
                c.dirty = true;
            }
        }
    }
    
    pub fn generate_chunk(&mut self, cx: i32, cz: i32) {
        let mut blocks = vec![AIR; (CH * CH * SY) as usize];
        
        for lx in 0..CH {
            for lz in 0..CH {
                let (wx, wz) = (cx * CH + lx, cz * CH + lz);
                let h = height_at(wx, wz, self.seed);
                
                for y in 0..=h {
                    let b = if y == h {
                        if h <= 6 { SAND } else { GRASS }
                    } else if y >= h - 2 {
                        DIRT
                    } else {
                        STONE
                    };
                    blocks[Self::idx(lx, y, lz)] = b;
                }
            }
        }
        
        // Генерация деревьев
        for gx in cx * CH - 2..cx * CH + CH + 2 {
            for gz in cz * CH - 2..cz * CH + CH + 2 {
                if hash2(gx, gz, 777 + self.seed) % 89 != 0 {
                    continue;
                }
                
                let h = height_at(gx, gz, self.seed);
                if h <= 6 {
                    continue;
                }
                
                let th = 4 + (hash2(gx, gz, 778 + self.seed) % 2) as i32;
                
                let mut put = |x: i32, y: i32, z: i32, b: u8| {
                    let (lx, lz) = (x - cx * CH, z - cz * CH);
                    if lx >= 0 && lx < CH && lz >= 0 && lz < CH && y >= 0 && y < SY {
                        let i = Self::idx(lx, y, lz);
                        if blocks[i] == AIR {
                            blocks[i] = b;
                        }
                    }
                };
                
                // Ствол
                for y in h + 1..=h + th {
                    put(gx, y, gz, WOOD);
                }
                
                // Листва
                for dx in -2..=2i32 {
                    for dz in -2..=2i32 {
                        for dy in th - 1..=th + 1 {
                            if dx == 0 && dz == 0 && dy <= th {
                                continue;
                            }
                            if dx.abs() == 2 && dz.abs() == 2 && dy == th + 1 {
                                continue;
                            }
                            put(gx + dx, h + dy, gz + dz, LEAVES);
                        }
                    }
                }
                put(gx, h + th + 2, gz, LEAVES);
            }
        }
        
        // Применяем сохранённые модификации к новому чанку
        for lx in 0..CH {
            for lz in 0..CH {
                let wx = cx * CH + lx;
                let wz = cz * CH + lz;
                for y in 0..SY {
                    if let Some(&b) = self.modifications.get(&(wx, y, wz)) {
                        blocks[Self::idx(lx, y, lz)] = b;
                    }
                }
            }
        }
        
        self.chunks.insert(
            (cx, cz),
            Chunk {
                blocks,
                meshes: Vec::new(),
                dirty: true,
            },
        );
        
        // Помечаем соседние чанки как dirty
        for k in [(cx - 1, cz), (cx + 1, cz), (cx, cz - 1), (cx, cz + 1)] {
            if let Some(c) = self.chunks.get_mut(&k) {
                c.dirty = true;
            }
        }
    }
}