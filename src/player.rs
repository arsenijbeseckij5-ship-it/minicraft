use macroquad::prelude::*;

use crate::world::World;

pub const HALF: f32 = 0.3;
pub const HEIGHT: f32 = 1.8;
pub const EYE: f32 = 1.62;

pub struct Player {
    pub pos: Vec3,
    pub vel: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
    pub fly: bool,
}

impl Player {
    fn aabb_collides(&self, w: &World, p: Vec3) -> bool {
        let min = vec3(p.x - HALF, p.y, p.z - HALF);
        let max = vec3(p.x + HALF, p.y + HEIGHT, p.z + HALF);

        for x in min.x.floor() as i32..=max.x.floor() as i32 {
            for y in min.y.floor() as i32..=max.y.floor() as i32 {
                for z in min.z.floor() as i32..=max.z.floor() as i32 {
                    if w.solid(x, y, z) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn look_dir(&self) -> Vec3 {
        vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
    }

    pub fn update(&mut self, w: &World, dt: f32) {
        let fwd = vec3(self.yaw.cos(), 0.0, self.yaw.sin());
        let right = vec3(-self.yaw.sin(), 0.0, self.yaw.cos());

        let mut wish = vec3(0.0, 0.0, 0.0);

        if is_key_down(KeyCode::W) {
            wish += fwd;
        }
        if is_key_down(KeyCode::S) {
            wish -= fwd;
        }
        if is_key_down(KeyCode::D) {
            wish += right;
        }
        if is_key_down(KeyCode::A) {
            wish -= right;
        }

        if wish.length() > 0.0 {
            wish = wish.normalize();
        }

        let speed = if self.fly {
            14.0
        } else if is_key_down(KeyCode::LeftControl) {
            8.0
        } else {
            5.0
        };

        self.vel.x = wish.x * speed;
        self.vel.z = wish.z * speed;

        if self.fly {
            self.vel.y = 0.0;

            if is_key_down(KeyCode::Space) {
                self.vel.y = speed;
            }
            if is_key_down(KeyCode::LeftShift) {
                self.vel.y = -speed;
            }
        } else {
            self.vel.y -= 25.0 * dt;

            if self.vel.y < -40.0 {
                self.vel.y = -40.0;
            }

            if is_key_down(KeyCode::Space) && self.on_ground {
                self.vel.y = 8.5;
                self.on_ground = false;
            }
        }

        let mut p = self.pos;

        p.x += self.vel.x * dt;
        if self.aabb_collides(w, p) {
            p.x = self.pos.x;
            self.vel.x = 0.0;
        }

        p.z += self.vel.z * dt;
        if self.aabb_collides(w, p) {
            p.z = self.pos.z;
            self.vel.z = 0.0;
        }

        self.on_ground = false;

        p.y += self.vel.y * dt;
        if self.aabb_collides(w, p) {
            if self.vel.y < 0.0 {
                self.on_ground = true;
            }

            p.y = self.pos.y;
            self.vel.y = 0.0;
        }

        self.pos = p;
    }
}