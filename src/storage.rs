use immortality_factory_laboratory::prelude::*;

pub fn overflow(columns: usize) -> Blueprint {
    let columns = columns as i32;
    let mut bp = World::new();
    let mut input = None;
    let mut output = None;
    let col_w = 1 + AbysalDoor.width();
    for i in 0..columns {
        let x = col_w * i;
        let bs = bp.place(BigSplitter, x, 0);
        for j in 0..4 {
            let ad = bp.place(AbysalDoor, x + 1, j);
            bp.connect(bs.output(j as usize), ad.input(0));
        }
        match output.take() {
            Some(prev) => bp.connect(prev, bs.input(0)),
            None => input = Some(bs.input(0)),
        }
        output = Some(bs.output(4));
    }
    Blueprint {
        contents: bp,
        size: Size {
            w: col_w * columns,
            h: BigSplitter.height(),
        },
        inputs: Vec::from_iter(input),
        outputs: Vec::from_iter(output),
    }
}

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
        match output.take() {
            Some(prev) => bp.connect(prev, cur.input(0)),
            None => input = Some(cur.input(0)),
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
