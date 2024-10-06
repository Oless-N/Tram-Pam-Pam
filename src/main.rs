
mod characters;
mod states;
mod weapons;
mod world;

use macroquad::prelude::*;
use crate::weapons::Bullet;
use crate::world::Physics;
use crate::characters::character::Character;
use crate::states::IdleState;

#[macroquad::main("TramPamPam")]
async fn main() {
    let walk_right = vec![
        load_texture("src/assets/img/pygame_right_1.png").await.unwrap(),
        load_texture("src/assets/img/pygame_right_2.png").await.unwrap(),
        load_texture("src/assets/img/pygame_right_3.png").await.unwrap(),
        load_texture("src/assets/img/pygame_right_4.png").await.unwrap(),
        load_texture("src/assets/img/pygame_right_5.png").await.unwrap(),
        load_texture("src/assets/img/pygame_right_6.png").await.unwrap(),
    ];

    let walk_left = vec![
        load_texture("src/assets/img/pygame_left_1.png").await.unwrap(),
        load_texture("src/assets/img/pygame_left_2.png").await.unwrap(),
        load_texture("src/assets/img/pygame_left_3.png").await.unwrap(),
        load_texture("src/assets/img/pygame_left_4.png").await.unwrap(),
        load_texture("src/assets/img/pygame_left_5.png").await.unwrap(),
        load_texture("src/assets/img/pygame_left_6.png").await.unwrap(),
    ];

    let player_stand = load_texture("src/assets/img/pygame_idle.png").await.unwrap();
    let bg = load_texture("src/assets/img/pygame_bg_3.png").await.unwrap();
    let bullet_texture = load_texture("src/assets/img/bullet.png").await.unwrap();

    let mut player = Character::new(50.0, 520.0, player_stand, walk_right, 5.0);
    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        // Оновлення фізики
        Physics::apply_physics();

        // Перевірка клавіш для руху
        if is_key_down(KeyCode::Right) {
            player.x += player.speed;
            player.ani_count = (player.ani_count + 1) % player.animations.len(); // Оновлення анімації
        } else if is_key_down(KeyCode::Left) {
            player.x -= player.speed;
            player.ani_count = (player.ani_count + 1) % player.animations.len(); // Оновлення анімації
        } else {
            player.ani_count = 0; // Відновлення анімації в стан очікування
        }

        // Стрільба
        if is_key_pressed(KeyCode::F) {
            bullets.push(Bullet::new(player.x + 20.0, player.y, 8.0, bullet_texture.clone()));
        }

        // Оновлення і малювання куль
        for bullet in &mut bullets {
            bullet.update();
            bullet.draw();
        }

        // Малювання гравця та фону
        clear_background(WHITE);
        draw_texture(&bg, 0.0, 0.0, WHITE);
        player.draw();

        next_frame().await;
    }
}
