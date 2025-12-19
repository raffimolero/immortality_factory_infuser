use crate::prelude::*;

pub fn trash(columns: usize) -> Blueprint {
    let mut bp = World::new();
    let col_w = 1 + AbysalDoor.width();
    let sps = stack_vec(columns, |i| {
        let x = col_w * i;
        let bs = bp.place(BigSplitter, x, 0);
        for j in 0..4 {
            let ad = bp.place(AbysalDoor, x + 1, j);
            bp.connect(bs.output(j as usize), ad.input(0));
        }
        bs
    });
    let (inputs, outputs) = split_inputs_outputs(chain_ports(&mut bp, &sps, [(0, 4)]));
    Blueprint {
        contents: bp,
        size: Size {
            w: col_w * columns as Coord,
            h: BigSplitter.height(),
        },
        inputs,
        outputs,
    }
}

pub fn storage(count: usize, rows: usize, item: Item) -> Blueprint {
    let rows = rows as Coord;
    let mut bp = World::new();
    let svs = stack_vec(count, |i| {
        let sv = bp.place(
            StructureData::StorageVault {
                input: item,
                storage: [item; 16],
                output: item,
            },
            (i / rows) * StorageVault.width(),
            (i % rows) * StorageVault.height(),
        );
        sv
    });
    let (inputs, outputs) = split_inputs_outputs(chain_ports(&mut bp, &svs, [(0, 0)]));
    Blueprint {
        contents: bp,
        size: Size {
            w: StorageVault.width() * count as i16 / rows,
            h: StorageVault.height() * rows,
        },
        inputs,
        outputs,
    }
}

/// literally only pipes the input into the first input of primary
/// and only gives you the first output of primary
///
/// if you want customizability, go place them manually
pub fn overflow_buffer(trash_columns: usize, primary_bp: &Blueprint) -> Blueprint {
    let mut bp = World::new();
    let overflow_bp = &trash(trash_columns);
    let overflow = bp.place(overflow_bp, 0, 0);
    let primary = bp.place(primary_bp, overflow.width(), 0);
    bp.connect(overflow.output(0), primary.input(0));
    Blueprint {
        contents: bp,
        size: Size {
            w: overflow_bp.width() + primary_bp.width(),
            h: overflow_bp.height().max(primary_bp.height()),
        },
        inputs: vec![overflow.input(0)],
        // outputs are instead accessed through primary and overflow
        outputs: vec![primary.output(0)],
    }
}

pub fn all_items(rows: usize) -> Blueprint {
    let mut bp = World::new();
    let mut inputs = vec![];
    let mut outputs = vec![];
    for (i, item) in Item::ITEMS.iter().copied().enumerate() {
        let i = i as Coord;
        let sv = bp.place(&storage(rows, rows, item), i * StorageVault.width(), 0);
        inputs.push(sv.input(0));
        outputs.push(sv.output(0));
    }
    Blueprint {
        contents: bp,
        size: StorageVault.size()
            * Size {
                w: Item::ITEMS.len() as Coord,
                h: rows as Coord,
            },
        inputs,
        outputs,
    }
}
