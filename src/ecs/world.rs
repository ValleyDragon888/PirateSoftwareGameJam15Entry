use crate::Entity;

pub struct World {
    pub entity_pool: Vec<Entity>
}

impl World {
    pub fn new() -> World {
        World{entity_pool: vec!()}
    }

    pub fn spawn_entity(&mut self, e: Entity) {
        self.entity_pool.push(e);
    }
}