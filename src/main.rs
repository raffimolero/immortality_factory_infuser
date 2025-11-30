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

mod disharmonizer_stack;
mod gold_factory;
mod pure_factory;
mod spark_factory;
mod storage;
mod util;

use crate::{prelude::*, util::export};

fn pure_stuff() -> World {
    let mut world = World::new();
    let pf_bp = &pure_factory();
    let stack_count = 1;

    let ov_bp = &trash(3);
    let ovs = stack_vec(stack_count * 2 + 2, |i| {
        let ov = world.place(ov_bp, -ov_bp.width(), ov_bp.height() * i);
        ov.input(0)
    });

    let pure_factories = stack_vec(stack_count, |i| {
        let pf = world.place(pf_bp, 0, i * pf_bp.height());
        world.connect(pf.output(2), ovs[i as usize * 2]);
        world.connect(pf.output(3), ovs[i as usize * 2 + 1]);
        pf
    });
    let (inputs, outputs) = chain_ports(&mut world, &pure_factories, [(0, 0), (1, 1)]).unwrap();
    for (i, o) in outputs.into_iter().enumerate() {
        world.connect(o, ovs[i + stack_count * 2]);
    }

    world
}

fn spark_stuff() -> World {
    let mut world = World::new();
    let pure_vault_count = 16;
    let pure_vault = world.place(
        &storage(pure_vault_count, pure_vault_count, PureManaGem),
        -StorageVault.width(),
        0,
    );
    let sf = world.place(&spark_factory(), 0, 0);
    world.connect(pure_vault.output(0), sf.input(1));
    {
        let sv_gold = world.place(&storage(8 * 4, 4, Empty), 0, sf.height());
        world.connect(sf.output(0), sv_gold.input(0));
    }
    {
        let sv_spark = world.place(
            &storage(8 * 4, 4, Empty),
            0,
            sf.height() + StorageVault.height() * 4,
        );
        world.connect(sf.output(1), sv_spark.input(0));
    }
    world
}

fn spark_stuff_2() -> World {
    let mut world = World::new();
    let pf = world.place(&pure_factory(), 0, 0);
    let sf = world.place(&spark_factory(), 0, pf.height());
    let sv_gold = world.place(&storage(8, 2, Empty), 38, 36);
    let sv_spark = world.place(&storage(8, 2, Empty), 38, 36 + StorageVault.height() * 2);
    let sv_blood = world.place(&storage(8, 2, Empty), 38, 36 + StorageVault.height() * 4);
    world.connect(pf.output(0), sf.input(0));
    world.connect(pf.output(2), sf.input(1));
    world.connect(pf.output(3), sf.input(2));
    world.connect(sf.output(0), sv_gold.input(0));
    world.connect(sf.output(1), sv_spark.input(0));
    world.connect(pf.output(1), sv_blood.input(0));
    world
}

fn gold_stuff() -> World {
    let mut world = World::new();
    let sf = world.place(&gold_factory(), 0, 0);
    let sv = world.place(&storage(8 * 4, 4, Empty), 0, sf.height());
    world.connect(sf.output(0), sv.input(0));
    world
}

fn stuff() -> World {
    // gold_stuff()
    // spark_stuff()
    // pure_stuff()
    spark_stuff_2()
}

fn main() {
    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    world.place(&all_items(16), -100, -100);
    world.place(&stuff(), 0, 0);

    export(&world, "../save.ini");
}
