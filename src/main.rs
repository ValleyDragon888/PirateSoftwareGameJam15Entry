#![feature(tuple_trait)]

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
mod collisions;
mod maths;
mod app;
use app::App;

use collisions::line::Line;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct C {}

fn hello() {
    println!("Hello")
}

fn hi() {
    println!("Hi")
}

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut app = App::new();
    app.update_schedule.add_systems((hello, hi).chain());
    app.run().await
}