use std::array;

use crate::disharmonizer_stack::disharmonizer_stack;

use immortality_factory_laboratory::prelude::*;

pub fn pure_factory() -> Blueprint {
    let mut bp = World::new();
    let mut inputs = vec![];
    let mut outputs = vec![];
    const ROWS: i32 = 5;

    let refine_w = AirPump.width() + Refinery.width();
    let refine_h = AirPump.height();
    let refines_w = refine_w * 3;

    // mana gems and disharms
    let mana_w = refines_w + ROWS;
    let dhs_mana_w = mana_w + Disharmonizer.width() * 2;
    let dhs_mana_h = ROWS * 2 * refine_h;
    let dhs_mana = array::from_fn::<_, { ROWS as usize * 2 }, _>(|i| {
        let i = i as i32;
        let merge_y = if i < ROWS {
            0
        } else {
            dhs_mana_h - BigMerger.height()
        };
        let merge = bp.place(BigMerger, refines_w + (i % ROWS), merge_y);
        let merge_port_last = BigMerger.connectors().inputs.len() - 1;

        let pump_y = i * refine_h;
        for j in 0..3 {
            let pump = bp.place(AirPump, j * refine_w, pump_y);
            let refine = bp.place(Refinery, j * refine_w + AirPump.width(), pump_y);
            bp.connect(pump.output(0), refine.input(0));
            bp.connect(refine.output(0), merge.input(merge_port_last - j as usize));
        }

        let dh_mana = bp.place(
            Disharmonizer,
            (i % 2) * Disharmonizer.width() + mana_w,
            (i / 2) * Disharmonizer.height(),
        );
        bp.connect(merge.output(0), dh_mana.input(0));
    });

    let dhs_curse = array::from_fn::<_, { ROWS as usize }, _>(|i| {
        let i = i as i32;
        let merge = bp.place(Merger, refines_w + i, BigMerger.height());
        let dh_curse = bp.place(Disharmonizer, dhs_mana_w, i * Disharmonizer.height());
    });

    Blueprint {
        contents: bp,
        size: Size { w: -1, h: -1 }, // TODO
        inputs,
        outputs,
    }
}
