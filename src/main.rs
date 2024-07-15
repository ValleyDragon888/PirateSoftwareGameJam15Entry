use macroquad::prelude::*;
pub mod ecs;
use ecs::schedule::Schedule;
use ecs::entity::{self, Component, Entity};
use ecs::world::{self, World};
use ecs::query::Query::With;

struct Enemy;
impl Component for Enemy {
    fn id(&self) -> String {"Enemy".to_string()}
}
struct Position(f32, f32);
impl Component for Position {
    fn id(&self) -> String {"Position".to_string()}
}

fn sys_1(world :&mut World) {
    println!("sys 1 run");
    for entity in world.entity_pool.iter() {
        if entity.contains_component("Ene") {println!("found")}
        if entity.satisfies_query(vec!(With("En"))) { println!("whoop") }
    }    
    // println!("{:?}", world.entity_pool.len());
    // println!("{:?}", world.entity_pool[0].components.len());
}

fn sys_2(_:&mut World) {
    println!("sys 2 run");
}

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut s = Schedule { systems: &vec!(&sys_1, &sys_2) };
    let e = Entity { components: vec![Box::new(Enemy {}), Box::new(Position(0.0, 0.0))]};
    let mut w = World::new();
    w.spawn_entity(e);
    s.run(&mut w);
    loop {
        draw_circle(0.0, 0.0, 200.0, RED);
        next_frame().await;
    }
}