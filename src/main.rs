mod atlas;
mod constants;
mod hand;
mod icon;
mod meshing;
mod noise;
mod player;
mod raycast;
mod world;

use macroquad::prelude::*;
use atlas::make_atlas;
use constants::*;
use hand::build_hand;
use icon::build_icon;
use meshing::build_chunk_mesh;
use noise::height_at;
use player::{Player, EYE, HALF, HEIGHT};
use raycast::raycast;
use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "MiniCraft (Rust MVP)".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let seed = 12345; // Seed для мира
    let atlas_img = make_atlas();
    let atlas = Texture2D::from_image(&atlas_img);
    atlas.set_filter(FilterMode::Nearest);
    
    let mut world = World::new(seed);
    
    for dx in -R..=R {
        for dz in -R..=R {
            world.generate_chunk(dx, dz);
        }
    }
    
    let spawn_x = 8;
    let spawn_z = 8;
    let spawn_y = height_at(spawn_x, spawn_z, seed) + 1;
    
    let mut player = Player {
        pos: vec3(
            spawn_x as f32 + 0.5,
            spawn_y as f32 + 0.1,
            spawn_z as f32 + 0.5,
        ),
        vel: vec3(0.0, 0.0, 0.0),
        yaw: 0.8,
        pitch: -0.2,
        on_ground: false,
        fly: false,
    };
    
    let mut selected: usize = 0;
    let mut grabbed = false;
    let mut swing = 0.0f32;
    let mut bob = 0.0f32;
    let mut bob_amp = 0.0f32;
    let mut name_timer = 0.0f32;
    let mut icons: Vec<Option<Mesh>> = Vec::new();
    let mut last_screen = (0u32, 0u32);
    
    loop {
        let dt = get_frame_time().min(0.05);
        let sw = screen_width();
        let sh = screen_height();
        
        // Обработка захвата курсора
        if is_mouse_button_pressed(MouseButton::Left) && !grabbed {
            grabbed = true;
            let _ = set_cursor_grab(true);
            show_mouse(false);
        }
        
        if is_key_pressed(KeyCode::Escape) {
            grabbed = false;
            let _ = set_cursor_grab(false);
            show_mouse(true);
        }
        
        if grabbed {
            let d = mouse_delta_position();
            player.yaw -= d.x * 1.8;
            player.pitch = (player.pitch + d.y * 1.8).clamp(-1.5, 1.5);
            
            if is_key_pressed(KeyCode::F) {
                player.fly = !player.fly;
                player.vel.y = 0.0;
            }
            
            for (i, key) in [
                KeyCode::Key1, KeyCode::Key2, KeyCode::Key3,
                KeyCode::Key4, KeyCode::Key5, KeyCode::Key6,
                KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
            ].iter().enumerate() {
                if is_key_pressed(*key) && selected != i {
                    selected = i;
                    name_timer = 2.0;
                }
            }
            
            let wy = mouse_wheel().1;
            if wy > 0.0 {
                selected = (selected + 8) % 9;
                name_timer = 2.0;
            } else if wy < 0.0 {
                selected = (selected + 1) % 9;
                name_timer = 2.0;
            }
        }
        
        player.update(&world, dt);
        
        // Walk bobbing
        let hspeed = (player.vel.x * player.vel.x + player.vel.z * player.vel.z).sqrt();
        let bob_target = if player.on_ground && hspeed > 0.1 && !player.fly {
            1.0
        } else {
            0.0
        };
        bob_amp += (bob_target - bob_amp) * (dt * 10.0).min(1.0);
        if hspeed > 0.1 {
            bob += dt * hspeed * 1.6;
        }
        
        // Генерация чанков вокруг игрока
        let pcx = (player.pos.x.floor() as i32).div_euclid(CH);
        let pcz = (player.pos.z.floor() as i32).div_euclid(CH);
        
        let mut generated = 0;
        for dx in -R..=R {
            for dz in -R..=R {
                let key = (pcx + dx, pcz + dz);
                if !world.chunks.contains_key(&key) && generated < 4 {
                    world.generate_chunk(key.0, key.1);
                    generated += 1;
                }
            }
        }
        
        // Удаление далёких чанков (но модификации сохраняются)
        world.chunks.retain(|&(cx, cz), _| {
            (cx - pcx).abs() <= R + 1 && (cz - pcz).abs() <= R + 1
        });
        
        // Перестройка мешей с лимитом
        let dirty_keys: Vec<(i32, i32)> = world
            .chunks
            .iter()
            .filter(|(_, c)| c.dirty)
            .map(|(k, _)| *k)
            .take(MAX_CHUNKS_PER_FRAME)
            .collect();
        
        for key in dirty_keys {
            let meshes = build_chunk_mesh(&world, key.0, key.1, &atlas);
            if let Some(c) = world.chunks.get_mut(&key) {
                c.meshes = meshes;
                c.dirty = false;
            }
        }
        
        // Взаимодействие с блоками
        let look = player.look_dir();
        let right_h = vec3(-player.yaw.sin(), 0.0, player.yaw.cos());
        let horiz = bob.cos() * 0.04 * bob_amp;
        let vert = (bob * 2.0).sin() * 0.05 * bob_amp;
        let roll = bob.cos() * 0.015 * bob_amp;
        let eye = player.pos + vec3(0.0, EYE, 0.0) + right_h * horiz + vec3(0.0, vert, 0.0);
        let cam_up = (vec3(0.0, 1.0, 0.0) + right_h * roll).normalize();
        let target = raycast(&world, eye, look, 6.0);
        
        if grabbed {
            // Анимация удара
            if is_mouse_button_pressed(MouseButton::Left)
                || is_mouse_button_pressed(MouseButton::Right)
            {
                swing = 0.25;
            }
            if is_mouse_button_down(MouseButton::Left) && swing <= 0.0 {
                swing = 0.25;
            }
            
            // Разрушение/установка блоков (только при нажатии, не удержании)
            if let Some((hit, prev)) = target {
                if is_mouse_button_pressed(MouseButton::Left) {
                    world.set(hit.x, hit.y, hit.z, AIR);
                }
                if is_mouse_button_pressed(MouseButton::Right) && HOTBAR[selected] != AIR {
                    let bmin = vec3(prev.x as f32, prev.y as f32, prev.z as f32);
                    let bmax = bmin + vec3(1.0, 1.0, 1.0);
                    let pmin = player.pos - vec3(HALF, 0.0, HALF);
                    let pmax = player.pos + vec3(HALF, HEIGHT, HALF);
                    
                    let overlap = bmin.x < pmax.x
                        && bmax.x > pmin.x
                        && bmin.y < pmax.y
                        && bmax.y > pmin.y
                        && bmin.z < pmax.z
                        && bmax.z > pmin.z;
                    
                    if !overlap {
                        world.set(prev.x, prev.y, prev.z, HOTBAR[selected]);
                    }
                }
            }
        }
        
        swing -= dt;
        name_timer -= dt;
        
        // ---- Рендеринг мира ----
        clear_background(Color::new(0.53, 0.75, 0.95, 1.0));
        
        let cam = Camera3D {
            position: eye,
            target: eye + look,
            up: cam_up,
            fovy: 78.0f32.to_radians(),
            ..Default::default()
        };
        set_camera(&cam);
        
        for (_, c) in world.chunks.iter() {
            for m in &c.meshes {
                draw_mesh(m);
            }
        }
        
        if let Some((hit, _)) = target {
            draw_cube_wires(
                vec3(hit.x as f32 + 0.5, hit.y as f32 + 0.5, hit.z as f32 + 0.5),
                vec3(1.002, 1.002, 1.002),
                BLACK,
            );
        }
        
        // Рука от первого лица
        let hand = build_hand(eye, look, swing.max(0.0), bob, bob_amp, &atlas);
        draw_mesh(&hand);
        
        // ---- HUD ----
        set_default_camera();
        
        // Hotbar
        if (sw as u32, sh as u32) != last_screen {
            last_screen = (sw as u32, sh as u32);
            let slot = 44.0;
            let x0 = (sw - slot * 9.0) / 2.0;
            let y0 = sh - slot - 10.0;
            icons = HOTBAR
                .iter()
                .enumerate()
                .map(|(i, &b)| {
                    if b == AIR {
                        None
                    } else {
                        Some(build_icon(
                            b,
                            x0 + i as f32 * slot + slot / 2.0,
                            y0 + slot / 2.0 + 2.0,
                            &atlas,
                        ))
                    }
                })
                .collect();
        }
        
        let slot = 44.0;
        let total = slot * 9.0;
        let x0 = (sw - total) / 2.0;
        let y0 = sh - slot - 10.0;
        
        draw_rectangle(
            x0 - 3.0,
            y0 - 3.0,
            total + 6.0,
            slot + 6.0,
            Color::new(0.0, 0.0, 0.0, 0.55),
        );
        
        for i in 0..9 {
            let x = x0 + i as f32 * slot;
            draw_rectangle(x, y0, slot, slot, Color::new(0.15, 0.15, 0.15, 0.6));
            draw_rectangle_lines(x, y0, slot, slot, 1.0, Color::new(0.5, 0.5, 0.5, 0.8));
            
            if let Some(Some(m)) = icons.get(i) {
                draw_mesh(m);
            }
        }
        
        draw_rectangle_lines(
            x0 + selected as f32 * slot - 2.0,
            y0 - 2.0,
            slot + 4.0,
            slot + 4.0,
            3.0,
            WHITE,
        );
        
        if name_timer > 0.0 && HOTBAR[selected] != AIR {
            let name = BLOCK_NAMES[HOTBAR[selected] as usize];
            let tw = measure_text(name, None, 24, 1.0).width;
            draw_text(name, (sw - tw) / 2.0, y0 - 14.0, 24.0, WHITE);
        }
        
        // Crosshair + info
        let cx = sw / 2.0;
        let cy = sh / 2.0;
        draw_line(cx - 8.0, cy, cx + 8.0, cy, 2.0, WHITE);
        draw_line(cx, cy - 8.0, cx, cy + 8.0, 2.0, WHITE);
        
        draw_text(
            &format!(
                "FPS: {}   {}   pos: {:.0} {:.0} {:.0}   chunks: {}   seed: {}",
                get_fps(),
                if player.fly { "FLY" } else { "WALK" },
                player.pos.x,
                player.pos.y,
                player.pos.z,
                world.chunks.len(),
                world.seed
            ),
            12.0,
            28.0,
            20.0,
            WHITE,
        );
        
        if !grabbed {
            draw_text("CLICK TO PLAY", cx - 110.0, cy - 30.0, 40.0, WHITE);
            draw_text(
                "WASD move | Space jump | F fly | LMB break | RMB place | 1-9/wheel select | Esc menu",
                cx - 330.0,
                cy + 10.0,
                20.0,
                WHITE,
            );
        }
        
        next_frame().await;
    }
}