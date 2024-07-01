use std::marker::Tuple;
use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use macroquad::miniquad::start;
use macroquad::window::next_frame;
use crate::hello;
    pub(crate) struct App {
    pub world: World,
    pub update_schedule: Schedule,
    pub render_schedule: Schedule,
    pub startup_schedule: Schedule
}


impl App {
    pub(crate) fn new() -> App {
        App {
            world: World::new(),
            update_schedule: Schedule::default(),
            render_schedule: Schedule::default(),
            startup_schedule: Schedule::default()
        }
    }
    pub async fn run(&mut self) {
        self.startup_schedule.run(&mut self.world);
        loop {
            next_frame().await;
            self.update_schedule.run(&mut self.world);
            // init render schedule with chained activites using ().chain()
            self.render_schedule.run(&mut self.world);
        }
    }
}