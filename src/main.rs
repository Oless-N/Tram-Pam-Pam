mod characters;
mod states;
mod weapons;
mod world;

use macroquad::prelude::*;
use crate::weapons::{Bullet, Weapon};  // Імпортуйте трейт Weapon
use crate::characters::character::Character;
use crate::world::physics::Physics;

enum GameState {
    Menu,
    Playing,
}

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

    let mut player = Character::new(50.0, 520.0, player_stand, walk_right.clone(), walk_left.clone(), 2.0);
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut bullet = Bullet::new(player.x, player.y, 8.0, bullet_texture.clone()); // Приклад кулі

    let mut moving_left = false;

    // Змінні для стрибків і гравітації
    let mut is_jumping = false;
    let mut vertical_speed = 0.0;
    let gravity = 0.5;
    let jump_strength = -10.0;
    let ground_level = 520.0; // рівень землі

    // Змінна для керування станом гри (меню або гра)
    let mut game_state = GameState::Menu;

    loop {
        match game_state {
            GameState::Menu => {
                // Малюємо меню
                clear_background(LIGHTGRAY);
                draw_text("TramPamPam", screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 50.0, 50.0, DARKGRAY);
                draw_text("Press ENTER to Start", screen_width() / 2.0 - 150.0, screen_height() / 2.0, 30.0, DARKGRAY);

                // Якщо натиснуто клавішу ENTER, переходимо до ігрового режиму
                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Оновлення фізики
                Physics::apply_physics(&mut player);

                // Перевірка клавіш для руху
                if is_key_down(KeyCode::Right) {
                    player.x += player.speed;
                    player.animations = walk_right.clone(); // Змінюємо анімацію на рух вправо
                    player.ani_count = (player.ani_count + 1) % player.animations.len(); // Оновлення анімації
                    moving_left = false;
                } else if is_key_down(KeyCode::Left) {
                    player.x -= player.speed;
                    player.animations = walk_left.clone(); // Змінюємо анімацію на рух вліво
                    player.ani_count = (player.ani_count + 1) % player.animations.len(); // Оновлення анімації
                    moving_left = true;
                } else {
                    player.ani_count = 0; // Відновлення анімації в стан очікування
                }

                // Перевірка на стрибок
                if is_key_pressed(KeyCode::Up) && !is_jumping {
                    is_jumping = true;
                    vertical_speed = jump_strength;
                }

                // Оновлення вертикальної швидкості та положення
                if is_jumping {
                    player.y += vertical_speed;
                    vertical_speed += gravity;

                    // Перевірка, чи персонаж на землі
                    if player.y >= ground_level {
                        player.y = ground_level;
                        is_jumping = false;
                        vertical_speed = 0.0;
                    }
                }

                // Стрільба
                if is_key_pressed(KeyCode::Space) {
                    // Виклик атаки (створення кулі)
                    bullet.attack(&player, &mut bullets, bullet_texture.clone());
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
            }
        }

        next_frame().await;
    }
}
