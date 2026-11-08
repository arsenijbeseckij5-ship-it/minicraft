use macroquad::prelude::*;
use crate::atlas::{face_tile, tile_uv};
use crate::world::World;

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub(crate) fn face_corners(face: usize) -> [Vec3; 4] {
    match face {
        0 => [vec3(1.0, 0.0, 0.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0), vec3(1.0, 0.0, 1.0)],
        1 => [vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0)],
        2 => [vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 1.0), vec3(1.0, 1.0, 1.0), vec3(1.0, 1.0, 0.0)],
        3 => [vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0)],
        4 => [vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 1.0), vec3(1.0, 1.0, 1.0), vec3(0.0, 1.0, 1.0)],
        _ => [vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0)],
    }
}

pub(crate) fn face_uvs(face: usize, uv: (f32, f32, f32, f32)) -> [Vec2; 4] {
    let (u0, u1, v0, v1) = uv;
    match face {
        0 | 5 => [vec2(u0, v1), vec2(u0, v0), vec2(u1, v0), vec2(u1, v1)],
        1 | 4 => [vec2(u0, v1), vec2(u1, v1), vec2(u1, v0), vec2(u0, v0)],
        _ => [vec2(u0, v0), vec2(u0, v1), vec2(u1, v1), vec2(u1, v0)],
    }
}

pub(crate) fn face_shade(face: usize) -> f32 {
    match face {
        2 => 1.0,
        3 => 0.4,
        4 | 5 => 0.8,
        _ => 0.6,
    }
}

// Вычисление Ambient Occlusion для вершины
fn compute_ao(w: &World, x: i32, y: i32, z: i32, face: usize, corner: usize) -> f32 {
    let (s1, s2, c) = match face {
        0 => match corner {
            0 => (w.occludes(x+1, y-1, z), w.occludes(x+1, y, z-1), w.occludes(x+1, y-1, z-1)),
            1 => (w.occludes(x+1, y+1, z), w.occludes(x+1, y, z-1), w.occludes(x+1, y+1, z-1)),
            2 => (w.occludes(x+1, y+1, z), w.occludes(x+1, y, z+1), w.occludes(x+1, y+1, z+1)),
            _ => (w.occludes(x+1, y-1, z), w.occludes(x+1, y, z+1), w.occludes(x+1, y-1, z+1)),
        },
        1 => match corner {
            0 => (w.occludes(x-1, y-1, z), w.occludes(x-1, y, z+1), w.occludes(x-1, y-1, z+1)),
            1 => (w.occludes(x-1, y-1, z), w.occludes(x-1, y, z-1), w.occludes(x-1, y-1, z-1)),
            2 => (w.occludes(x-1, y+1, z), w.occludes(x-1, y, z-1), w.occludes(x-1, y+1, z-1)),
            _ => (w.occludes(x-1, y+1, z), w.occludes(x-1, y, z+1), w.occludes(x-1, y+1, z+1)),
        },
        2 => match corner {
            0 => (w.occludes(x, y+1, z-1), w.occludes(x-1, y+1, z), w.occludes(x-1, y+1, z-1)),
            1 => (w.occludes(x, y+1, z+1), w.occludes(x-1, y+1, z), w.occludes(x-1, y+1, z+1)),
            2 => (w.occludes(x, y+1, z+1), w.occludes(x+1, y+1, z), w.occludes(x+1, y+1, z+1)),
            _ => (w.occludes(x, y+1, z-1), w.occludes(x+1, y+1, z), w.occludes(x+1, y+1, z-1)),
        },
        3 => match corner {
            0 => (w.occludes(x, y-1, z-1), w.occludes(x-1, y-1, z), w.occludes(x-1, y-1, z-1)),
            1 => (w.occludes(x, y-1, z-1), w.occludes(x+1, y-1, z), w.occludes(x+1, y-1, z-1)),
            2 => (w.occludes(x, y-1, z+1), w.occludes(x+1, y-1, z), w.occludes(x+1, y-1, z+1)),
            _ => (w.occludes(x, y-1, z+1), w.occludes(x-1, y-1, z), w.occludes(x-1, y-1, z+1)),
        },
        4 => match corner {
            0 => (w.occludes(x-1, y, z+1), w.occludes(x, y-1, z+1), w.occludes(x-1, y-1, z+1)),
            1 => (w.occludes(x+1, y, z+1), w.occludes(x, y-1, z+1), w.occludes(x+1, y-1, z+1)),
            2 => (w.occludes(x+1, y, z+1), w.occludes(x, y+1, z+1), w.occludes(x+1, y+1, z+1)),
            _ => (w.occludes(x-1, y, z+1), w.occludes(x, y+1, z+1), w.occludes(x-1, y+1, z+1)),
        },
        _ => match corner {
            0 => (w.occludes(x-1, y, z-1), w.occludes(x, y+1, z-1), w.occludes(x-1, y+1, z-1)),
            1 => (w.occludes(x-1, y, z-1), w.occludes(x, y-1, z-1), w.occludes(x-1, y-1, z-1)),
            2 => (w.occludes(x+1, y, z-1), w.occludes(x, y-1, z-1), w.occludes(x+1, y-1, z-1)),
            _ => (w.occludes(x+1, y, z-1), w.occludes(x, y+1, z-1), w.occludes(x+1, y+1, z-1)),
        },
    };
    
    // Исправленная логика: преобразуем bool в 0/1 перед сложением
    let side1_val = if s1 { 1 } else { 0 };
    let side2_val = if s2 { 1 } else { 0 };
    let corner_val = if c { 1 } else { 0 };
    
    // Если обе соседние грани твёрдые, угол полностью в тени (0)
    let ao = if side1_val == 1 && side2_val == 1 {
        0
    } else {
        side1_val + side2_val + corner_val
    };
    
    match ao {
        0 => 0.55,
        1 => 0.70,
        2 => 0.85,
        _ => 1.0,
    }
}

pub(crate) fn push_face(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    corners: [Vec3; 4],
    uvs: [Vec2; 4],
    shade: f32,
    ao_values: Option<[f32; 4]>, // <-- 6-й аргумент
) {
    let start = vertices.len() as u16;
    
    for (i, (c, uv)) in corners.iter().zip(uvs.iter()).enumerate() {
        let ao = ao_values.map(|ao| ao[i]).unwrap_or(1.0);
        let final_shade = shade * ao;
        let color = Color::new(final_shade, final_shade, final_shade, 1.0);
        
        vertices.push(Vertex {
            position: *c,
            uv: *uv,
            color: color.into(),
            normal: vec4(0.0, 0.0, 0.0, 0.0),
        });
    }
    
    indices.extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
}

pub fn build_chunk_mesh(w: &World, cx: i32, cz: i32, atlas: &Texture2D) -> Vec<Mesh> {
    const MAX_INDICES: usize = 4800;
    let mut meshes = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    
    let chunk = match w.chunks.get(&(cx, cz)) {
        Some(c) => c,
        None => return meshes,
    };
    
    for y in 0..crate::constants::SY {
        for lz in 0..crate::constants::CH {
            for lx in 0..crate::constants::CH {
                let b = chunk.blocks[World::idx(lx, y, lz)];
                if b == crate::constants::AIR {
                    continue;
                }
                
                let (wx, wz) = (cx * crate::constants::CH + lx, cz * crate::constants::CH + lz);
                let origin = vec3(wx as f32, y as f32, wz as f32);
                
                for face in 0..6 {
                    let (dx, dy, dz) = NEIGHBORS[face];
                    if w.occludes(wx + dx, y + dy, wz + dz) {
                        continue;
                    }
                    
                    if indices.len() + 6 > MAX_INDICES {
                        meshes.push(Mesh {
                            vertices: std::mem::take(&mut vertices),
                            indices: std::mem::take(&mut indices),
                            texture: Some(atlas.clone()),
                        });
                    }
                    
                    let corners = face_corners(face).map(|c| origin + c);
                    
                    let ao_values = [
                        compute_ao(w, wx, y, wz, face, 0),
                        compute_ao(w, wx, y, wz, face, 1),
                        compute_ao(w, wx, y, wz, face, 2),
                        compute_ao(w, wx, y, wz, face, 3),
                    ];
                    
                    push_face(
                        &mut vertices,
                        &mut indices,
                        corners,
                        face_uvs(face, tile_uv(face_tile(b, face))),
                        face_shade(face),
                        Some(ao_values), // <-- Передаём AO
                    );
                }
            }
        }
    }
    
    if !indices.is_empty() {
        meshes.push(Mesh {
            vertices,
            indices,
            texture: Some(atlas.clone()),
        });
    }
    
    meshes
}