use macroquad::color::RED;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{draw_circle, draw_texture, Texture2D, WHITE};
use crate::maths::Vec2s;

pub struct Player {
    pub pos:Vec2s,
    pub texture2d: Texture2D
}

impl Player {
    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.pos.y -= 0.01
        } else if is_key_down(KeyCode::S) {
            self.pos.y += 0.01
        }
        if is_key_down(KeyCode::A) {
            self.pos.x -= 0.01
        } else if is_key_down(KeyCode::D) {
            self.pos.x += 0.01
        }
        self.pos.clamp();
        draw_texture(&self.texture2d, self.pos.x(), self.pos.y(), WHITE);
        draw_circle(self.pos.x(), self.pos.y(), 10.0, RED);
    }
}

pub struct Enemy {
    pub pos:Vec2s,
    pub texture2d: Texture2D
}


impl Enemy {
    pub fn update(&mut self, mut player_pos: &mut Vec2s) {
        let mut move_distance = player_pos - self.pos.clone();
        self.pos += move_distance.normalised()*0.001;
        //draw_line(self.pos.x(), self.pos.y(), move_distance.x(), move_distance.y(), 1.0, RED);
        draw_texture(&self.texture2d, self.pos.x(), self.pos.y(), WHITE);
    }
}