use macroquad::prelude::{draw_texture, Texture2D};
use macroquad::color::WHITE;
use quad_rand;
use crate::cooldown::Cooldown;
use crate::maths::{Vec2s, vec2s};
use crate::player::Player;

pub struct Enemy {
    pub pos:Vec2s,
    pub texture2d: Vec<Texture2D>,
    pub attack_cooldown: Cooldown,
    pub animation_cooldown: Cooldown,
    pub current_frame: usize
}

impl Enemy {
    pub fn update(&mut self, player: &mut Player) {
        let player_pos = player.pos.clone();

        // update cooldowns
        self.attack_cooldown.update();
        self.animation_cooldown.update();

        // attempt to attack player
        if player_pos.clone().get_screen_position_vec2().distance(self.pos.clone().get_screen_position_vec2()) < 100.0 {
            if self.attack_cooldown.attempt_use() {
                player.health -= 1
            }
        }

        if self.animation_cooldown.attempt_use() {
            self.current_frame += 1;
            if self.current_frame > 7 {
                self.current_frame = 0
            }
        }

        // move towards player
        let mut adjusted_player_pos = vec2s(player_pos.x+quad_rand::gen_range(-0.1, 0.1),
                                            player_pos.y+quad_rand::gen_range(-0.1, 0.1));
        let mut move_distance = adjusted_player_pos - &mut self.pos.clone();
        let speed = quad_rand::gen_range(-0.001, 0.003);
        self.pos += move_distance.normalised()*speed;
        //draw_line(self.pos.x(), self.pos.y(), move_distance.x(), move_distance.y(), 1.0, RED);
        draw_texture(&self.texture2d.get_mut(self.current_frame).unwrap(), self.pos.x(), self.pos.y(), WHITE);
    }

    pub fn texture_names() -> Vec<String> {
        let mut names = vec!();
        for i in 0..8 {
            names.push("assets/zombie/".to_owned() + &*i.to_string() + ".png")
        }
        println!("{:?}", names);
        return names
    }
}

pub struct ZombieManager {
    pub zombies: Vec<Enemy>
}

impl ZombieManager {
    pub fn update(&mut self, player: &mut Player) {
        for zombie in self.zombies.iter_mut() {
            zombie.update(player);
        }
    }

    pub fn new(texture: Vec<Texture2D>) -> Self {
        let mut enemies = vec!();
        for _ in 0..10 {
            enemies.push(Enemy {
                pos: vec2s(quad_rand::gen_range(0.0, 1.0), quad_rand::gen_range(0.0, 1.0)),
                texture2d: texture.clone(),
                attack_cooldown: Cooldown { timer: 2.0, cooldown: 2.0 },
                animation_cooldown: Cooldown { timer: 0.25, cooldown: 0.25 },
                current_frame: 0
            });
        }
        return ZombieManager {
            zombies: enemies
        }
    }
}