#![allow(clippy::all)]

mod prelude {
    pub use crate::{
        disharmonizer_stack::disharmonizer_stack, gold_factory::gold_factory,
        pure_factory::pure_factory, spark_factory::spark_factory, storage::*, util::*,
    };

    pub use immortality_factory_laboratory::prelude::*;
}

mod chassis_factory;
mod disharmonizer_stack;
mod gold_factory;
mod pure_factory;
mod spark_factory;
mod storage;
mod stress_test;
mod util;

use crate::{prelude::*, util::export};

fn stuff() -> World {
    // let mut bp = World::new();
    // let pump = bp.place(AirPump, 0, 0);
    // let refine = bp.place(Refinery, 2, 0);
    // bp.connect(pump.output(0), refine.input(0));
    // let bp = Blueprint {
    //     contents: bp,
    //     size: Size {
    //         w: AirPump.width() + Refinery.width(),
    //         h: 2,
    //     },
    //     inputs: vec![],
    //     outputs: vec![refine.output(0)],
    // };
    // let mut world = World::new();
    // let mc0 = world.place(&bp, 0, 0);
    // let mc1 = world.place(&bp, 0, 2);
    // let merge = world.place(Merger, 0, 4);
    // world.connect(mc0.output(0), merge.input(0));
    // world.connect(mc1.output(0), merge.input(1));
    // world

    // World::new()
    // gold_factory::gold_demo()
    // spark_factory::spark_demo()
    // pure_factory::pure_demo()
    // spark_factory::pure_spark_demo()
    // chassis_factory::chassis_demo()
    chassis_factory::pure_chassis_demo()
    // stress_test::ideal()
    // stress_test::buildings()
    // stress_test::pumps()
    // stress_test::ports()
    // stress_test::connections()
}

fn get_pump_count(world: &World) -> usize {
    world
        .structures
        .iter()
        .filter(|s| s.structure.kind() == AirPump)
        .count()
}

fn main() {
    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    // world.place(&all_items(4), -100, -100);
    world.place(&stuff(), 0, 0);

    println!("{} Air Pumps.", get_pump_count(&world));
    println!("{} Structures.", world.structures.len());
    let structure_size = size_of_val(&world.structures[0]);
    let structure_bytes = world.structures.len() * structure_size;
    println!("{structure_bytes} bytes from structures ({structure_size} bytes/struct",);
    // dbg!(&world);
    export(&world, "../save_v1_6.ini").expect("Could not export world.");
}
