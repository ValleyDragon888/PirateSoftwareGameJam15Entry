use crate::enemy::Enemy;
use std::process::exit;
use macroquad::prelude::*;
use player::Player;
use crate::GameState::Playing;
use crate::enemy::ZombieManager;
use crate::potions::{PotionInventorySlot, PotionType};

mod maths;
mod cooldown;
mod player;
mod enemy;
mod potions;
mod constants;

pub enum GameState {
    Playing,
    Dead(DeathReason)
}

pub enum DeathReason {
    LanternRanOut,
    HealthRanOut
}

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut game_state = GameState::Playing;
    let mut player_textures = vec!();
    for texture in Player::texture_names() {
        player_textures.push(load_texture(&*texture).await.unwrap());
    }
    let mut player = Player::new(player_textures);

    let mut zombie_textures = vec!();
    for texture in Enemy::texture_names() {
        zombie_textures.push(load_texture(&*texture).await.unwrap());
    }
    let mut enemy = ZombieManager::new(zombie_textures);

    let mut potionslot = PotionInventorySlot::new(load_texture("assets/potions/0.png").await.unwrap(), PotionType::Damage);
    loop {
        // // Set camera
        // let view_size = vec2(720.0f32, 240.0f32);
        // let window_size = vec2(screen_size().0, screen_size().1);
        //
        // let scale = (window_size / view_size).min_element().floor().max(1.0);
        // let remaining_size = (window_size / scale) - view_size;
        // let camera_size = view_size + remaining_size;
        //
        // set_camera(&Camera2D::from_display_rect(Rect::new( 0.0, camera_size.y, camera_size.x, -camera_size.y)));
        match game_state {
            GameState::Playing => {
                clear_background(WHITE);
                player.update();
                enemy.update(&mut player);

                // lantern
                draw_rectangle(0.0, 0.0, screen_width(), screen_height()-20.0, player.get_lantern_bgcol());
                // let bgcol = player.get_lantern_bgcol();
                // for x in 0..screen_width() as i32 {
                //     for y in 0..screen_height() as i32 {
                //         draw_rectangle(x as f32, y as f32, 1.0, 1.0, bgcol)
                //     }
                // }

                potionslot.update(100.0, 50.0,&mut enemy);
                next_frame().await;
                if player.health <= 0  { game_state = GameState::Dead(DeathReason::HealthRanOut) }
                else if player.lantern_capacity <= 0.0  { game_state = GameState::Dead(DeathReason::LanternRanOut) }
            }
            GameState::Dead(ref reason) => {
                do_death_screen(reason).await;
                game_state = Playing;
                let mut player_textures = vec!();
                for texture in Player::texture_names() {
                    player_textures.push(load_texture(&*texture).await.unwrap());
                }
                player = Player::new(player_textures);

                let mut zombie_textures = vec!();
                for texture in Enemy::texture_names() {
                    zombie_textures.push(load_texture(&*texture).await.unwrap());
                }
                enemy = ZombieManager::new(zombie_textures);
                potionslot = PotionInventorySlot::new(load_texture("assets/potions/0.png").await.unwrap(), PotionType::Damage);
            }
        }

    }
}

async fn do_death_screen(reason: &DeathReason) {
    loop {
        clear_background(BLACK);

        // You Died
        draw_text_center("You Died.", 100.0, 100, WHITE);

        // Death specific messasge
        let message = match reason {
            DeathReason::LanternRanOut => {"You let your lantern run out of juice."}
            DeathReason::HealthRanOut => {"Your health ran out."}
        };
        draw_text_center(message, 250.0, 50, WHITE);
        draw_text_center("\"Maybe stay alive next time.\"", 270.0, 30, WHITE);

        // Buttons
        let respawn_size = draw_text_center("Re-Spawn", 350.0, 100, BLUE);
        let rage_quit_size = draw_text_center("Rage Quit", 420.0, 100, BLUE);

        let respawn_rect = Rect::new(respawn_size.1, 300.0, respawn_size.0.width, respawn_size.0.height);
        let rage_quit_rect = Rect::new(rage_quit_size.1, 370.0, rage_quit_size.0.width, rage_quit_size.0.height);

        if respawn_rect.contains(Vec2::from(mouse_position())) && is_mouse_button_down(MouseButton::Left) { break }
        if rage_quit_rect.contains(Vec2::from(mouse_position())) && is_mouse_button_down(MouseButton::Left) { exit(0); }

        next_frame().await;
    }
}

fn center_pos(width:f32) -> f32 {
    return (screen_width() / 2.0) - (width / 2.0)
}

fn draw_text_center(text: &str, y:f32, font_size:i8, color: Color) -> (TextDimensions, f32) {
    // f32 return is the center_pos of text
    let text_size = measure_text(
        text, Option::None, font_size as u16, 1.0
    );
    draw_text(text,
              center_pos(text_size.width),
              y,
              font_size as f32,
              color);
    return (text_size, center_pos(text_size.width))
}