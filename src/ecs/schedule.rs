use crate::World;

pub struct Schedule<'a> {
    pub systems: &'a Vec<&'a dyn Fn(&mut World) -> ()>
}

impl<'a> Schedule<'a> {
    pub fn run(&'a mut self, world: &mut World) {
        let systems = self.systems;
        for func in systems {
            func(world);
        }
    }
}