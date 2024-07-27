use macroquad::prelude::{draw_circle, draw_rectangle, draw_text, draw_texture, screen_height, screen_width, Texture2D};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::color::{BLACK, BLUE, Color, DARKBLUE, MAROON, RED, WHITE};
use macroquad::time::get_frame_time;
use crate::cooldown::Cooldown;
use crate::maths::{Vec2s, vec2s};

pub struct Player {
    pub pos:Vec2s,
    pub texture2d: Vec<Texture2D>,
    pub health:i8,
    pub lantern_capacity: f32,
    pub animation_cooldown: Cooldown,
    pub current_frame: usize
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

        // do animation
        self.animation_cooldown.update();
        if self.animation_cooldown.attempt_use() {
            self.current_frame += 1;
            if self.current_frame > 16 {
                self.current_frame = 0;
            }
        }

        // Lantern updates
        self.lantern_capacity -= get_frame_time();



        draw_texture(&self.texture2d.get_mut(self.current_frame).unwrap(), self.pos.x(), self.pos.y(), WHITE);



        // This is debug 0,0 on image. remove @ some point
        draw_circle(self.pos.x(), self.pos.y(), 10.0, RED);
        self.draw_health_and_lantern_bars();
    }

    fn draw_health_and_lantern_bars(&mut self) {
        // Draw the player's health as a bar on the bottom of the screen
        let one_unit = (screen_width()/2.0)/100.0;
        // draw the dark red beckground
        draw_rectangle(0.0, screen_height()-20.0, screen_width()/2.0, 20.0, MAROON);
        // draw health rectangle
        draw_rectangle(0.0, screen_height()-20.0, f32::from(self.health)* one_unit, 20.0, RED);
        // I hate this hack. <===========================================> why? WHYYYYYy?
        draw_text(&("Health: ".to_owned()+&self.health.to_string()+"%")
        , 0.0, screen_height()-4.0, 28.0, BLACK);

        //Lantern bar - copied code, im too lazy to make a shared method (or function???)
        // draw the blue beckground
        draw_rectangle(screen_width()/2.0, screen_height()-20.0, screen_width()/2.0, 20.0, DARKBLUE);
        // draw health rectangle
        draw_rectangle(screen_width()/2.0, screen_height()-20.0, f32::from(self.lantern_capacity)* one_unit, 20.0, BLUE);
        // I still hate this hack, but this time it's copied code AS WELL!
        let lantern_capacity_rounded = self.lantern_capacity.clone().round();
        draw_text(&("Lantern Capacity: ".to_owned()+ &*lantern_capacity_rounded.to_string() +"%")
                  , screen_width()/2.0, screen_height()-4.0, 28.0, BLACK);
    }

    pub(crate) fn new(texture: Vec<Texture2D>) -> Player {
        return Player {
            pos: vec2s(0.1, 0.1),
            texture2d: texture,
            health: 100,
            lantern_capacity: 100.0,
            animation_cooldown: Cooldown {timer: 0.1, cooldown: 0.1},
            current_frame: 0
        };
    }

    pub fn texture_names() -> Vec<String> {
        let mut names = vec!();
        for i in 0..17 {
            names.push("assets/wizard/".to_owned() + &*i.to_string() + ".png")
        }
        println!("{:?}", names);
        return names
    }
    
    pub fn get_lantern_bgcol(&self) -> Color {
        let brightness = self.lantern_capacity.clone().clamp(0.0, 100.0)/100.0;
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0-brightness,
        }
    }
}