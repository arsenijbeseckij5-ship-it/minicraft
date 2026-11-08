use macroquad::prelude::*;
use crate::constants::AIR;
use crate::world::World;

pub fn raycast(
    w: &World,
    origin: Vec3,
    dir: Vec3,
    max_dist: f32,
) -> Option<(IVec3, IVec3)> {
    // DDA voxel traversal (Amanatides & Woo)
    let mut x = origin.x.floor() as i32;
    let mut y = origin.y.floor() as i32;
    let mut z = origin.z.floor() as i32;
    
    let step_x = if dir.x > 0.0 { 1 } else if dir.x < 0.0 { -1 } else { 0 };
    let step_y = if dir.y > 0.0 { 1 } else if dir.y < 0.0 { -1 } else { 0 };
    let step_z = if dir.z > 0.0 { 1 } else if dir.z < 0.0 { -1 } else { 0 };
    
    let t_delta_x = if dir.x != 0.0 { (1.0 / dir.x).abs() } else { f32::MAX };
    let t_delta_y = if dir.y != 0.0 { (1.0 / dir.y).abs() } else { f32::MAX };
    let t_delta_z = if dir.z != 0.0 { (1.0 / dir.z).abs() } else { f32::MAX };
    
    let mut t_max_x = if dir.x > 0.0 {
        (x as f32 + 1.0 - origin.x) / dir.x
    } else if dir.x < 0.0 {
        (x as f32 - origin.x) / dir.x
    } else {
        f32::MAX
    };
    
    let mut t_max_y = if dir.y > 0.0 {
        (y as f32 + 1.0 - origin.y) / dir.y
    } else if dir.y < 0.0 {
        (y as f32 - origin.y) / dir.y
    } else {
        f32::MAX
    };
    
    let mut t_max_z = if dir.z > 0.0 {
        (z as f32 + 1.0 - origin.z) / dir.z
    } else if dir.z < 0.0 {
        (z as f32 - origin.z) / dir.z
    } else {
        f32::MAX
    };
    
    let mut prev = ivec3(x, y, z);
    let mut t = 0.0;
    
    // Проверяем стартовую позицию
    if w.get(x, y, z) != AIR {
        return None;
    }
    
    while t < max_dist {
        // Находим ближайшую границу
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                t = t_max_x;
                t_max_x += t_delta_x;
            } else {
                z += step_z;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        } else {
            if t_max_y < t_max_z {
                y += step_y;
                t = t_max_y;
                t_max_y += t_delta_y;
            } else {
                z += step_z;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        }
        
        // Проверяем блок
        if w.get(x, y, z) != AIR {
            return Some((ivec3(x, y, z), prev));
        }
        
        prev = ivec3(x, y, z);
    }
    
    None
}