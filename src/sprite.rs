use macroquad::color::{BLACK, BLUE, RED};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{draw_circle, draw_texture, Texture2D, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use macroquad::window::{screen_height, screen_width};
use crate::cooldown::Cooldown;
use crate::maths::Vec2s;

pub struct Player {
    pub pos:Vec2s,
    pub texture2d: Texture2D,
    pub health:i8
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
        // This is debug 0,0 on image. remove @ some point
        draw_circle(self.pos.x(), self.pos.y(), 10.0, RED);
        self.draw_health();
    }

    fn draw_health(&mut self) {
        // Draw the player's health as a bar on the bottom of the screen
        let one_health_unit = screen_width()/100.0;
        draw_rectangle(0.0, screen_height()-20.0, (f32::from(self.health)*one_health_unit), 20.0, RED);
        // I hate this hack. <===========================================> why? WHYYYYYy?
        draw_text(&("Health: ".to_owned()+&self.health.to_string()+"%") 
        , 0.0, screen_height()-4.0, 28.0, BLACK);
    }
} 

pub struct Enemy {
    pub pos:Vec2s,
    pub texture2d: Texture2D,
    pub attack_cooldown: Cooldown
}


impl Enemy {
    pub fn update(&mut self, player: &mut Player) {
        let player_pos = player.pos.clone();

        // update attack cooldown
        self.attack_cooldown.update();
        println!("{:?}", self.attack_cooldown.timer);

        // attempt to attack player
        if player_pos.clone().get_screen_position_vec2().distance(self.pos.clone().get_screen_position_vec2()) < 100.0 {
            if self.attack_cooldown.attempt_use() {
                player.health -= 1
            }
        }

        // move towards player
        let mut move_distance = player_pos - &mut self.pos.clone();
        self.pos += move_distance.normalised()*0.001;
        //draw_line(self.pos.x(), self.pos.y(), move_distance.x(), move_distance.y(), 1.0, RED);
        draw_texture(&self.texture2d, self.pos.x(), self.pos.y(), WHITE);
    }
}