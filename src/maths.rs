use macroquad::window::{screen_height, screen_width};

pub enum ScalingScalarReferences {
    WIDTH,
    HEIGHT
}

struct ScalingScalar {
    pub scale:f32,
    pub reference: ScalingScalarReferences
}
impl ScalingScalar {
    pub fn re_scale(&mut self) -> f32 {
        let mut target: Option<f32> = None;
        match self.reference {
            ScalingScalarReferences::WIDTH => { target = Option::from(screen_width()) }
            ScalingScalarReferences::HEIGHT => { target = Option::from(screen_height()) }
        }
        // I'm not sure if f32 has enough presicion. maybe use https://crates.io/crates/fraction
        // Actually I think its fine :)
        target.unwrap() / self.scale
    }
}

pub struct Vec2s {
    pub x: ScalingScalar,
    pub y: ScalingScalar,
}

// fn vec2s(x:f32, y:f32) -> Vec2s {
//     Vec2s::new(x, y)
// }

impl Vec2s {
    // fn new(x:f32, y:f32) -> Vec2s {
    //     Vec2s {
    //         x: ScalingScalar {}
    //     }
    // }
}