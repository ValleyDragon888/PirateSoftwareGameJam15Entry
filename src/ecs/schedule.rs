pub struct Schedule<'a> {
    pub systems: &'a Vec<&'a dyn Fn() -> ()>
}

impl<'a> Schedule<'a> {
    pub fn run(&'a mut self) {
        let systems = self.systems;
        for func in systems {
            func();
        }
    }
}