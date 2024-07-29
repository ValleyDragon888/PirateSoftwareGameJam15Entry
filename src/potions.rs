use macroquad::color::{BLACK, WHITE};
use macroquad::input::{is_key_down, mouse_position, MouseButton};
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{draw_texture, is_mouse_button_down, KeyCode, RED, screen_height, Texture2D};
use macroquad::shapes::{draw_circle_lines, draw_rectangle, draw_rectangle_lines};
use macroquad::text::draw_text;
use crate::cooldown::Cooldown;
use crate::enemy::ZombieManager;

pub enum PotionType {
    Damage
}

pub enum PotionInstance {
    Damage(f32, f32) // Position
}

pub struct PotionInventorySlot {
    texture2d: Texture2D,
    key_code: KeyCode,
    key_code_string: String,
    cooldown: Cooldown,
    potion_type: PotionType,
    placing:bool
}

impl PotionInventorySlot {
    pub fn new(texture2d: Texture2D, potion_type: PotionType) -> Self {
        match potion_type {
            PotionType::Damage => {
                PotionInventorySlot {
                    texture2d: texture2d,
                    key_code: KeyCode::Key1,
                    key_code_string: "1".to_string(),
                    cooldown: Cooldown {timer: 10.0, cooldown:10.0},
                    potion_type,
                    placing: false
                }
            }
        }
    }

    pub fn update(&mut self, x:f32, y:f32, zombie_manager: &mut ZombieManager) {
        self.cooldown.update();
        if is_key_down(self.key_code) {
            if self.cooldown.attempt_use() {
                self.placing = true
            }
        }
        if self.placing && is_key_down(KeyCode::Escape) { self.placing = false }
        if self.placing && is_mouse_button_down(MouseButton::Left) {
            match self.potion_type {
                PotionType::Damage => {
                    zombie_manager.potion(PotionInstance::Damage(mouse_position().0, mouse_position().1));
                    self.placing = false
                }
            }
        }

        self.render(x, y)
    }

    pub fn render(&mut self, x:f32, y:f32) {
        // draw cooldown
        let width = 20.0/(self.cooldown.cooldown/self.cooldown.timer);
        draw_rectangle(x, y-30.0, width.clamp(0.0, 20.0), 40.0, RED);

        draw_text(&*self.key_code_string, x, y, 50.0, BLACK);

        draw_texture(&self.texture2d, x, y+20.0, WHITE);

        // draw box around
        draw_rectangle_lines(x, y-30.0, 20.0, y+40.0, 5.0, BLACK);
        
        // tooltip
        if Rect::new(x, y-30.0, 20.0, y+40.0).contains(vec2(mouse_position().0, mouse_position().1)) {
            draw_text("Damage Potion. Damages zombies. Duh.", 0.0, screen_height() - 40.0, 50.0, BLACK)
        }

        if self.placing {
            draw_texture(&self.texture2d, mouse_position().0, mouse_position().1, WHITE);
            draw_circle_lines(mouse_position().0, mouse_position().1, 100.0, 10.0, RED);
        }
    }
}

pub struct PotionManager {
}

impl PotionManager {
    pub fn update() { todo!() }
}