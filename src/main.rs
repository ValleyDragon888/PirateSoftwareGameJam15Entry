#![feature(tuple_trait)]

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
mod collisions;
mod maths;
mod app;
use app::App;

#[derive(Component, Debug)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Circle {}

fn hello() {
    println!("Hello")
}

fn hi() {
    println!("Hi")
}

fn create_circle(
    mut world: &mut World
) {
    world.spawn((
        Position {x: 0.0, y:0.0},
        Circle {}
    ));
}

fn print_circles(
    mut query: Query<&mut Position, With<Circle>>
) {
    for i in query.iter_mut() {
        println!("Found circle at {:?}", i)
    }
}

fn render_circles(
    query: Query<&Position, With<Circle>>
) {
    for i in query.iter() {
        draw_circle(i.x, i.y, 200.0, RED);
    }
}

fn move_circles(
    mut query: Query<&mut Position, With<Circle>>
) {
    for mut pos in query.iter_mut() {
        pos.x += 20.0 * get_frame_time()
    }
}

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut app = App::new();
    app.startup_schedule.add_systems(create_circle);
    app.update_schedule.add_systems((print_circles, move_circles, (hello, hi).chain()));
    app.render_schedule.add_systems(render_circles);
    app.run().await
}