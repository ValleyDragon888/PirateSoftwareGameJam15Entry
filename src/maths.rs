use macroquad::math::{Vec2, vec2};
use macroquad::prelude::screen_width;
use macroquad::window::screen_height;

pub struct Vec2s {
    pub x:f32,
    pub y:f32,
}

impl Vec2s {
    pub fn get_screen_position_vec2(&mut self) -> Vec2 {
        return vec2(screen_width()*self.x, screen_height()*self.y)
    }
    pub fn get_raw_vec2(&mut self) -> Vec2 {
        return vec2(self.x, self.y);
    }

    pub fn x(&mut self) -> f32 { return self.get_screen_position_vec2().x }
    pub fn y(&mut self) -> f32 { return self.get_screen_position_vec2().y }

    pub fn clamp(&mut self) {
        self.x = self.x.clamp(0.0, 1.0);
        self.y = self.y.clamp(0.0, 1.0);
    }
    
    pub fn normalised(&mut self) -> Vec2s {
        return Vec2s::from_vec2(self.get_raw_vec2().normalize())
    }
    
    pub fn from_vec2(v: Vec2) -> Vec2s {
        return vec2s(v.x, v.y);
    }
}


pub fn vec2s(x:f32, y:f32) -> Vec2s {return Vec2s{x:x, y:y}}