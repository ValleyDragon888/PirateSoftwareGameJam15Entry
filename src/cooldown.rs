use macroquad::time::get_frame_time;

pub struct Cooldown {
    pub timer: f32,
    pub cooldown: f32
}

impl Cooldown {
    pub fn attempt_use(&mut self) -> bool {
        if self.timer <= 0.0 {
            self.timer += self.cooldown;
            return true;
        } else {
            return false;
        }
    }

    pub fn update(&mut self) {
        self.timer -= get_frame_time();
    }

    pub fn new(initial_value:f32, cooldown: f32) -> Self {
        return Self {timer:initial_value, cooldown: cooldown}
    }
}