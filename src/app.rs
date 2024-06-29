use std::marker::Tuple;
use bevy_ecs::prelude::Schedule;
use bevy_ecs::world::World;
use macroquad::window::next_frame;
use crate::hello;

struct App {
    world: World,
    update_schedule: Schedule,
    render_schedule: Schedule,
}


impl App {
    fn new() -> App {
        App {
            world: World::new(),
            update_schedule: Schedule::default(),
            render_schedule: Schedule::default()
        }
    }
    async fn run(&mut self) {
        next_frame().await;
        self.update_schedule.run(&mut self.world);
        // init render schedule with chained activites using ().chain()
        self.render_schedule.run(&mut self.world);
    }
}