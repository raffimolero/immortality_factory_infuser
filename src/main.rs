mod disharmonizer_stack;
mod gold_factory;
mod pure_factory;
mod spark_factory;
mod storage;

use crate::{pure_factory::pure_factory, spark_factory::spark_factory, storage::*};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use immortality_factory_laboratory::prelude::*;

fn pure_stuff() -> World {
    let mut world = World::new();
    let pure_factory = world.place(&pure_factory(), 0, 0);
    world
}

fn spark_stuff() -> World {
    let mut world = World::new();
    let pure_vault_count = 16;
    let pure_vault = world.place(
        &storage_vault(pure_vault_count, pure_vault_count, PureManaGem),
        -StorageVault.width(),
        0,
    );
    let sf = world.place(&spark_factory(), 0, 0);
    world.connect(pure_vault.output(0), sf.input(1));
    {
        let sv_gold = world.place(&storage_vault(8 * 4, 4, Empty), 0, sf.height());
        world.connect(sf.output(0), sv_gold.input(0));
    }
    {
        let sv_spark = world.place(
            &storage_vault(8 * 4, 4, Empty),
            0,
            sf.height() + StorageVault.height() * 4,
        );
        world.connect(sf.output(1), sv_spark.input(0));
    }
    world
}

fn stuff() -> World {
    // spark_stuff()
    pure_stuff()
}

fn main() {
    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    world.place(&all_items(16), -100, -100);
    world.place(&stuff(), 0, 0);

    let file = File::create("../save.ini").expect("Failed to create file.");
    let mut writer = BufWriter::new(file);
    world
        .export(&mut writer)
        .and_then(|()| writer.flush()) // docs told me to flush it manually instead of relying on Drop
        .expect("Failed to write to file.");
}
