use crate::prelude::*;

/// inputs: [gold, blood]
///
/// outputs: [gold, blood, pure, pure]
pub fn pure_factory() -> Blueprint {
    let mut bp = World::new();
    let mut inputs = vec![];
    let mut outputs = vec![];
    const ROWS: i32 = 5;

    // pre-calculate structure widths
    let refine_w = AirPump.width() + Refinery.width();
    let refine_h = AirPump.height();
    let refines_w = refine_w * 3;
    let refines_h = ROWS * 2 * refine_h;
    let mana_w = refines_w + ROWS;
    let dhs_mana_w = mana_w + Disharmonizer.width() * 2;
    let dhs_curse_w = dhs_mana_w + Disharmonizer.width();
    let ufs_w = dhs_curse_w + Unifier.width() * 5;
    let ufs_gem_h = Unifier.height() * 2;
    let ufs_blood_h = ufs_gem_h + Unifier.height();
    let merges_curse_h = ufs_blood_h - Merger.height();
    let factory_w = ufs_w + Refinery.width();
    let factory_h = refines_h;

    // mana gems and disharms
    let out_curse_gem = stack::<_, { ROWS as usize * 2 }>(|i| {
        let merge_y = if i < ROWS {
            0
        } else {
            refines_h - BigMerger.height()
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

        let uf_gem = bp.place(
            Unifier,
            dhs_curse_w + (i / 2) * Unifier.width(),
            (i % 2) * Unifier.height(),
        );
        bp.connect(dh_mana.output(0), uf_gem.input(0));
        bp.connect(dh_mana.output(1), uf_gem.input(1));
        bp.connect(dh_mana.output(3), uf_gem.input(2));
        (dh_mana.output(2), uf_gem.output(0))
    });

    const MERGES: usize = 4;
    let [bm_gold, bm_blood, bms @ ..] = stack::<_, MERGES>(|i| {
        let merge = bp.place(BigMerger, refines_w + i, BigMerger.height());
        merge
    });
    let merge_gold = bp.place(
        Merger,
        refines_w + MERGES as i32,
        BigMerger.height(), //
    );
    let merge_blood = bp.place(
        Merger,
        refines_w + MERGES as i32,
        BigMerger.height() + Merger.height(),
    );
    bp.connect(bm_gold.output(0), merge_gold.input(1));
    bp.connect(bm_blood.output(0), merge_blood.input(1));
    inputs.push(merge_gold.input(0));
    inputs.push(merge_blood.input(0));
    outputs.push(merge_gold.output(0));
    outputs.push(merge_blood.output(0));

    let _ = stack::<_, { ROWS as usize }>(|i| {
        let merge = bp.place(Merger, ufs_w + (i % ROWS), merges_curse_h);
        let dh_curse = bp.place(Disharmonizer, dhs_mana_w, i * Disharmonizer.height());
        bp.connect(out_curse_gem[i as usize * 2].0, merge.input(0));
        bp.connect(out_curse_gem[i as usize * 2 + 1].0, merge.input(1));
        bp.connect(merge.output(0), dh_curse.input(0));

        let uf_blood = bp.place(Unifier, dhs_curse_w + i * Unifier.width(), ufs_gem_h);
        bp.connect(dh_curse.output(1), uf_blood.input(0));
        bp.connect(dh_curse.output(2), uf_blood.input(1));
        bp.connect(uf_blood.output(0), bm_blood.input(i as usize));

        let sell = bp.place(
            SubdimensionalMarket,
            dhs_curse_w + i * SubdimensionalMarket.width(),
            ufs_blood_h,
        );
        bp.connect(dh_curse.output(0), sell.input(0));
        bp.connect(sell.output(2), uf_blood.input(2));
        bp.connect(sell.output(0), bm_gold.input(i as usize));
    });

    let _ = stack::<_, 2>(|i| {
        let refine = bp.place(Refinery, ufs_w, merges_curse_h - refine_h * (1 + i));

        let merge = bms[i as usize];
        for j in 0..ROWS {
            bp.connect(
                out_curse_gem[(i * ROWS + j) as usize].1,
                merge.input(j as usize),
            );
        }
        bp.connect(merge.output(0), refine.input(0));
        outputs.push(refine.output(0));
    });

    Blueprint {
        contents: bp,
        size: Size {
            w: factory_w,
            h: factory_h,
        },
        inputs,
        outputs,
    }
}
