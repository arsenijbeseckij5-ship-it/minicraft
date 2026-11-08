use macroquad::prelude::*;
use crate::atlas::{tile_uv, T_SKIN, T_SLEEVE};
use crate::meshing::{face_corners, face_shade, face_uvs, push_face};

fn push_box<F: Fn(Vec3) -> Vec3>(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    transform: &F,
    min: Vec3,
    max: Vec3,
    tile: u8,
) {
    let size = max - min;
    for face in 0..6 {
        let corners = face_corners(face).map(|c| transform(min + c * size));
        push_face(
            vertices,
            indices,
            corners,
            face_uvs(face, tile_uv(tile)),
            face_shade(face),
            None, // <-- Добавлено: для руки AO не нужен, используем базовый shade
        );
    }
}

pub fn build_hand(
    eye: Vec3,
    look: Vec3,
    swing: f32,
    bob: f32,
    amp: f32,
    atlas: &Texture2D,
) -> Mesh {
    let right = look.cross(vec3(0.0, 1.0, 0.0)).normalize();
    let upv = right.cross(look);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    let swing = swing.clamp(0.0, 0.25);
    let s = if swing > 0.0 {
        ((1.0 - swing / 0.25) * std::f32::consts::PI).sin()
    } else {
        0.0
    };
    
    let yaw_a = -0.38 - s * 0.15;
    let pitch_p = -0.12 - s * 0.22;
    let (sa, ca) = yaw_a.sin_cos();
    let (sp, cp) = pitch_p.sin_cos();
    
    let bx = bob.cos() * 0.006 * amp;
    let by = bob.sin() * 0.008 * amp - s * 0.03;
    
    let transform = |l: Vec3| -> Vec3 {
        let y1 = l.y * cp - l.z * sp;
        let z1 = l.y * sp + l.z * cp;
        let x2 = l.x * ca + z1 * sa;
        let z2 = -l.x * sa + z1 * ca;
        
        let lx = x2 + 0.68 + bx;
        let ly = y1 - 0.42 + by;
        let lz = z2 + 0.08;
        
        eye + right * lx + upv * ly + look * lz
    };
    
    push_box(
        &mut vertices,
        &mut indices,
        &transform,
        vec3(-0.11, -0.11, -0.45),
        vec3(0.11, 0.11, 0.55),
        T_SLEEVE,
    );
    
    push_box(
        &mut vertices,
        &mut indices,
        &transform,
        vec3(-0.11, -0.11, 0.55),
        vec3(0.11, 0.11, 0.82),
        T_SKIN,
    );
    
    Mesh {
        vertices,
        indices,
        texture: Some(atlas.clone()),
    }
}