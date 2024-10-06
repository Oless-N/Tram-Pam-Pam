use macroquad::prelude::*;
use crate::characters::character::Character;

#[derive(Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
    pub texture: Texture2D,
}

impl Bullet {
    pub fn new(x: f32, y: f32, velocity: f32, texture: Texture2D) -> Self {
        Bullet {
            x,
            y,
            velocity,
            texture,
        }
    }

    pub fn update(&mut self) {
        self.x += self.velocity;
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.x, self.y, WHITE);
    }
}

pub trait Weapon {
    fn attack(&self, owner: &Character, bullets: &mut Vec<Bullet>, bullet_texture: Texture2D);
    fn update(&mut self, owner: &mut Character);
}

impl Weapon for Bullet {
    fn attack(&self, owner: &Character, bullets: &mut Vec<Bullet>, bullet_texture: Texture2D) {
        let bullet_x = owner.x + 20.0;
        let bullet_y = owner.y;
        bullets.push(Bullet::new(bullet_x, bullet_y, self.velocity, bullet_texture));
    }

    fn update(&mut self, owner: &mut Character) {
        // Логіка оновлення
    }
}