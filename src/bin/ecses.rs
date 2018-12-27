extern crate specs;

use specs::prelude::*;

struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Vel>();
    let e1 = world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    let e2 = world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    let e3 = world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();
    let e4 = world.create_entity().with(Pos(2.0)).build();

    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();
    dispatcher.setup(&mut world.res);
    dispatcher.dispatch(&mut world.res);

    world.maintain();
}