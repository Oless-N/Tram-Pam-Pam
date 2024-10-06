mod weapon;
mod bullet;

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
    fn attack(&self, owner: &Character); // Метод атаки, викликається, коли персонаж атакує
    fn update(&mut self, owner: &mut Character); // Оновлення стану зброї
}

impl Weapon for Bullet {
    fn attack(&self, owner: &Character) {
        // Реалізація атаки
    }

    fn update(&mut self, owner: &mut Character) {
        // Реалізація оновлення
    }
}
