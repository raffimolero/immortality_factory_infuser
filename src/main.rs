#![allow(clippy::all)]

mod prelude {
    pub use crate::{
        disharmonizer_stack::disharmonizer_stack,
        gold_factory::gold_factory,
        pure_factory::pure_factory,
        spark_factory::spark_factory,
        storage::{all_items, storage, trash},
        util::{chain_ports, split_inputs_outputs, stack, stack_vec},
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
    // gold_factory::gold_demo()
    // spark_factory::spark_demo()
    // pure_factory::pure_demo()
    // spark_factory::pure_spark_demo()
    chassis_factory::chassis_demo()
    // chassis_factory::pure_chassis_demo()
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
    world.place(&all_items(16), -100, -100);
    world.place(&stuff(), 0, 0);

    println!("{} Air Pumps.", get_pump_count(&world));
    println!("{} Structures.", world.structures.len());
    println!("{} Connections.", world.connections.len());
    let structure_size = size_of_val(&world.structures[0]);
    let connection_size = size_of_val(&world.connections[0]);
    let structure_bytes = world.structures.len() * structure_size;
    let connection_bytes = world.connections.len() * connection_size;
    println!(
        "{} bytes total:\n\
        > {structure_bytes} bytes from structures ({structure_size} bytes/struct)\n\
        > {connection_bytes} bytes from connections ({connection_size} bytes/connect)",
        structure_bytes + connection_bytes,
    );
    export(&world, "../save.ini").expect("Could not export world.");
}
