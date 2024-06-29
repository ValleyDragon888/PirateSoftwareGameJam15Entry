#![feature(tuple_trait)]

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
mod collisions;
mod maths;
mod app;

use collisions::line::Line;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct C {}

fn hello() {
    println!("Hello")
}

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut world = World::new();
    let mut update_schedule = Schedule::default();
    update_schedule.add_systems(hello);
    world.spawn(
        (
            Position {x:0.0, y:0.0},
            C {}
        )
    );
    loop {
        clear_background(RED);
        update_schedule.run(&mut world);
        next_frame().await
    }
}