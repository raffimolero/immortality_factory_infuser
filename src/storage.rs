use immortality_factory_laboratory::prelude::*;

pub fn storage_vault(count: usize, rows: usize, item: Item) -> Blueprint {
    let mut bp = World::new();
    let mut input = None;
    let mut output = None;
    for i in 0..count {
        let cur = bp.place(
            StructureData::StorageVault {
                input: item,
                storage: [item; 16],
                output: item,
            },
            (i / rows) as i32 * StorageVault.width(),
            (i % rows) as i32 * StorageVault.height(),
        );
        if let Some(prev) = output.take() {
            bp.connect(prev, cur.input(0));
        } else {
            input = Some(cur.input(0));
        }
        output = Some(cur.output(0));
    }
    Blueprint {
        contents: bp,
        size: Size {
            w: StorageVault.width(),
            h: StorageVault.height() * count as i32,
        },
        inputs: Vec::from_iter(input),
        outputs: Vec::from_iter(output),
    }
}

pub fn all_items(rows: usize) -> Blueprint {
    let mut bp = World::new();
    let mut inputs = vec![];
    let mut outputs = vec![];
    for (i, item) in Item::ITEMS.iter().copied().enumerate() {
        let i = i as i32;
        let sv = bp.place(
            &storage_vault(rows, rows, item),
            i * StorageVault.width(),
            0,
        );
        inputs.push(sv.input(0));
        outputs.push(sv.output(0));
    }
    Blueprint {
        contents: bp,
        size: StorageVault.size()
            * Size {
                w: Item::ITEMS.len() as i32,
                h: rows as i32,
            },
        inputs,
        outputs,
    }
}
