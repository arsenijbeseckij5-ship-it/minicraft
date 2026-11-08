use macroquad::prelude::*;
use crate::atlas::{face_tile, tile_uv};
use crate::meshing::push_face;

pub fn build_icon(b: u8, cx: f32, cy: f32, atlas: &Texture2D) -> Mesh {
    let w = 12.0;
    let h = 6.0;
    let v = 12.0;
    
    let t = vec2(cx, cy - v / 2.0 - h);
    let r = vec2(cx + w, cy - v / 2.0);
    let bmid = vec2(cx, cy - v / 2.0 + h);
    let l = vec2(cx - w, cy - v / 2.0);
    let b2 = vec2(cx, cy - v / 2.0 + h + v);
    let l2 = vec2(cx - w, cy - v / 2.0 + v);
    let r2 = vec2(cx + w, cy - v / 2.0 + v);
    
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let p = |v: Vec2| vec3(v.x, v.y, 0.0);
    
    let top_tile = face_tile(b, 2);
    let side_tile = face_tile(b, 0);
    
    // --- Верхняя грань ---
    let (u0, u1, v0, v1) = tile_uv(top_tile);
    let um = (u0 + u1) / 2.0;
    let vm = (v0 + v1) / 2.0;
    push_face(
        &mut vertices,
        &mut indices,
        [p(t), p(r), p(bmid), p(l)],
        [vec2(um, v0), vec2(u1, vm), vec2(um, v1), vec2(u0, vm)],
        1.0,
        None, // <-- Добавлено
    );
    
    // --- Левая грань ---
    let (u0, u1, v0, v1) = tile_uv(side_tile);
    push_face(
        &mut vertices,
        &mut indices,
        [p(l), p(bmid), p(b2), p(l2)],
        [vec2(u0, v0), vec2(u1, v0), vec2(u1, v1), vec2(u0, v1)],
        0.65,
        None, // <-- Добавлено
    );
    
    // --- Правая грань ---
    push_face(
        &mut vertices,
        &mut indices,
        [p(bmid), p(r), p(r2), p(b2)],
        [vec2(u0, v0), vec2(u1, v0), vec2(u1, v1), vec2(u0, v1)],
        0.85,
        None, // <-- Добавлено
    );
    
    Mesh {
        vertices,
        indices,
        texture: Some(atlas.clone()),
    }
}