use crate::constants::SY;

pub fn hash2(x: i32, z: i32, seed: u32) -> u32 {
    let mut h = (x as u32)
        .wrapping_mul(374761393)
        .wrapping_add((z as u32).wrapping_mul(668265263))
        .wrapping_add(seed.wrapping_mul(144665));
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^ (h >> 16)
}

pub fn rand01(x: i32, z: i32, seed: u32) -> f32 {
    ((hash2(x, z, seed) >> 8) & 0xffff) as f32 / 65535.0
}

pub fn vnoise(x: f32, z: f32, seed: u32) -> f32 {
    let xi = x.floor() as i32;
    let zi = z.floor() as i32;
    let fx = x - xi as f32;
    let fz = z - zi as f32;
    
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    
    let a = rand01(xi, zi, seed);
    let b = rand01(xi + 1, zi, seed);
    let c = rand01(xi, zi + 1, seed);
    let d = rand01(xi + 1, zi + 1, seed);
    
    a + (b - a) * sx + (c - a) * sz + (a - b - c + d) * sx * sz
}

pub fn height_at(x: i32, z: i32, seed: u32) -> i32 {
    let h = 12.0
        + 7.0 * vnoise(x as f32 / 28.0, z as f32 / 28.0, seed)
        + 3.0 * vnoise(x as f32 / 9.0, z as f32 / 9.0, seed + 1);
    (h.round() as i32).clamp(3, SY - 10)
}