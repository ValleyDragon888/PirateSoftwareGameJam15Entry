use macroquad::prelude::*;

struct Schedule<'a> {
    systems: &'a Vec<&'a dyn Fn() -> ()>
}

impl<'a> Schedule<'a> {
    fn run(&'a mut self) {
        let systems = self.systems;
        for func in systems {
            func();
        }
    }
}

fn sys_1() {
    println!("sys 1 run");
}

fn sys_2() {
    println!("sys 2 run");
}

#[macroquad::main("Desktop Build of Entry")]
async fn main() {
    let mut w = Schedule { systems: &vec!(&sys_1, &sys_2) };
    w.run();
    loop {
        draw_circle(0.0, 0.0, 200.0, RED);
        next_frame().await;
    }
}