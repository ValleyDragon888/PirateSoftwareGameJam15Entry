use macroquad::prelude::{draw_texture, Texture2D};
use macroquad::color::WHITE;
use crate::cooldown::Cooldown;
use crate::maths::Vec2s;
use crate::player::Player;

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