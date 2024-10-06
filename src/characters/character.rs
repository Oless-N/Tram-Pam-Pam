
use macroquad::prelude::*;
use crate::states::{State, IdleState};
use crate::weapons::Weapon;

pub struct Character {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub animations: Vec<Texture2D>,
    pub current_state: Box<dyn State>,
    pub health: i32,
    pub weapon: Option<Box<dyn Weapon>>, // Зброя персонажа
    pub speed: f32,
    pub ani_count: usize,
}

impl Character {
    pub fn new(x: f32, y: f32, texture: Texture2D, animations: Vec<Texture2D>, animations_2: Vec<Texture2D>, speed: f32) -> Self {
        Character {
            x,
            y,
            texture,
            animations,
            current_state: Box::new(IdleState::new()), // Початковий стан - очікування
            health: 100,
            weapon: None,
            speed,
            ani_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.current_state.update();

        if let Some(mut weapon) = self.weapon.take() { // тимчасово забираємо зброю
            weapon.update(self);
            self.weapon = Some(weapon); // повертаємо зброю після оновлення
        }
    }

    pub fn draw(&self) {
        // Використовуємо анімацію для відображення гравця під час руху
        let current_texture = if self.ani_count < self.animations.len() {
            &self.animations[self.ani_count]
        } else {
            &self.texture
        };
        draw_texture(current_texture, self.x, self.y, WHITE);
    }

    pub fn change_state(&mut self, new_state: Box<dyn State>) {
        self.current_state = new_state;
    }
}
