use crate::prelude::*;

/// input: [gold] + [pure] * 5
///
/// output: [gold, chassis] + [salt] * 3
pub fn chassis_factory() -> Blueprint {
    let mut bp = World::new();
    let mut inputs = vec![];
    let mut outputs = vec![];
    const ROWS: Coord = 5;
    const MARKETS: Coord = 14 * 2;

    // pre-calculate structure widths
    let refine_w = AirPump.width() + Refinery.width();
    let refines_w = refine_w * 3;
    let mana_w = refines_w + ROWS;
    let dhs_mana_x = mana_w;
    let dhs_curse_x = dhs_mana_x + Disharmonizer.width() * 2;
    let rfs_silica_x = dhs_curse_x + Disharmonizer.width();
    let dhs_plate_x = rfs_silica_x + Refinery.width();
    let ufs_x = dhs_plate_x + Disharmonizer.width();
    let factory_x = ufs_x + Unifier.width() * ROWS;

    let bss_x = SubdimensionalMarket.width() * MARKETS / 2 + ROWS;

    let refine_h = AirPump.height();
    let refines_h = ROWS * 2 * refine_h;
    let sells_y = refines_h;
    let rf_sheet_y = sells_y + BigSplitter.height();
    let factory_y = sells_y + SubdimensionalMarket.height() * 2;

    let merges_y = BigMerger.height() * 2;
    let big_merges_y = merges_y + Merger.height() * 4;

    let dust_i = 0;
    let salt_i = ROWS as usize * 4; // 20
    let blood_i = salt_i + 2;
    let gold_i = blood_i + 5;
    // let refs_plate_w = dhs_curse_w + Refinery.width();
    // let dhs_plate_w = refs_plate_w + Disharmonizer.width();
    // let ufs_w = dhs_plate_w + Unifier.width() * 5;
    // let ufs_gem_h = Unifier.height() * 2;
    // let ufs_blood_h = ufs_gem_h + Unifier.height();
    // let merges_curse_h = ufs_blood_h - Merger.height();
    // let factory_w = ufs_w + Refinery.width();
    // let factory_h = refines_h;

    let sells = stack::<_, { MARKETS as usize }>(|i| {
        let y = refines_h + i % 2 * SubdimensionalMarket.height();
        let x = i / 2 * SubdimensionalMarket.width() + (i >= 12) as i16 * ROWS;
        bp.place(SubdimensionalMarket, x, y)
    });

    let merges = stack::<_, { ROWS as usize * 4 }>(|i| {
        let y = merges_y + i / ROWS * Merger.height();
        let x = refines_w + i % ROWS;
        bp.place(Merger, x, y)
    });

    let mut big_merges = stack_vec(ROWS as usize, |i| {
        let y = big_merges_y;
        let x = refines_w + i;
        bp.place(BigMerger, x, y)
    });
    big_merges.push(bp.place(BigMerger, bss_x + 5, sells_y));

    let ([in_bms_gold], [out_bms_gold]) = chain_ports(&mut bp, &big_merges, [(0, 0)]).unwrap();
    inputs.push(in_bms_gold);
    for i in 0..blood_i {
        bp.connect(sells[i].output(0), big_merges[i / 4].input(1 + i % 4));
    }

    // mana gems and disharms
    let out_curse_silica = stack::<_, { ROWS as usize * 2 }>(|i| {
        let merge_y = i / ROWS * BigMerger.height();
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
        bp.connect(dh_mana.output(0), sells[i as usize * 2 + 0].input(0));
        bp.connect(dh_mana.output(1), sells[i as usize * 2 + 1].input(0));
        (dh_mana.output(2), dh_mana.output(3))
    });

    let bs_gold_ov = bp.place(BigSplitter, bss_x + 0, sells_y);
    let bs_gold = bp.place(BigSplitter, bss_x + 1, sells_y);
    let bs_silver = bp.place(BigSplitter, bss_x + 2, sells_y);
    let bs_copper = bp.place(BigSplitter, bss_x + 3, sells_y);
    let bm_sheet = bp.place(BigMerger, bss_x + 4, sells_y);
    let sm_gold = sells[gold_i];
    let rf_sheet = bp.place(Refinery, bss_x, rf_sheet_y);

    bp.connect(out_bms_gold, sm_gold.input(0));
    bp.connect(sm_gold.output(0), bs_gold_ov.input(0));
    bp.connect(bs_gold_ov.output(4), bs_gold.input(0));
    bp.connect(sm_gold.output(1), bs_silver.input(0));
    bp.connect(sm_gold.output(2), bs_copper.input(0));
    bp.connect(bm_sheet.output(0), rf_sheet.input(0));

    outputs.push(bs_gold_ov.output(0));
    outputs.push(rf_sheet.output(0));

    let in_curse_silica = stack::<_, { ROWS as usize }>(|i| {
        let dh_curse = bp.place(Disharmonizer, dhs_curse_x, i * Disharmonizer.height());
        let rf_silica_y = i * Disharmonizer.height();
        let rf_silica_0 = bp.place(Refinery, rfs_silica_x, rf_silica_y);
        let rf_silica_1 = bp.place(Refinery, rfs_silica_x, rf_silica_y + Refinery.height());
        let dh_plate = bp.place(Disharmonizer, dhs_plate_x, i * Disharmonizer.height());
        let uf_x = ufs_x + Unifier.width() * i;
        let uf_blood = bp.place(Unifier, uf_x, Unifier.height() * 0);
        let uf_bright = bp.place(Unifier, uf_x, Unifier.height() * 1);
        let uf_bar = bp.place(Unifier, uf_x, Unifier.height() * 2);
        let uf_sheet = bp.place(Unifier, uf_x, Unifier.height() * 3);

        let i = i as usize;

        let mg_curse = merges[ROWS as usize * 0 + i];
        let mg_silica_0 = merges[ROWS as usize * 1 + i];
        let mg_silica_1 = merges[ROWS as usize * 2 + i];
        let mg_plate = merges[ROWS as usize * 3 + i];
        let sm_blood = sells[blood_i + i];

        let out_salt = dh_curse.output(0);
        if i < 2 {
            bp.connect(out_salt, sells[salt_i + i].input(0));
        } else {
            outputs.push(out_salt);
        }

        // adamantine bar
        bp.connect(bs_copper.output(i), uf_bar.input(0));
        bp.connect(bs_silver.output(i), uf_bar.input(1));
        bp.connect(bs_gold.output(i), uf_bar.input(2));
        let out_bar = uf_bar.output(0);

        // silver coin
        bp.connect(out_curse_silica[i * 2 + 0].0, mg_curse.input(0));
        bp.connect(out_curse_silica[i * 2 + 1].0, mg_curse.input(1));
        bp.connect(mg_curse.output(0), dh_curse.input(0));
        bp.connect(dh_curse.output(1), uf_blood.input(0));
        bp.connect(dh_curse.output(2), uf_blood.input(1));
        bp.connect(sells[i].output(2), uf_blood.input(2));
        bp.connect(uf_blood.output(0), sm_blood.input(0));
        let out_silver = sm_blood.output(1);

        // gloom shard
        bp.connect(out_curse_silica[i * 2 + 0].1, mg_silica_0.input(0));
        bp.connect(out_curse_silica[i * 2 + 1].1, mg_silica_1.input(0));
        bp.connect(dh_plate.output(1), mg_silica_0.input(1));
        bp.connect(dh_plate.output(2), mg_silica_1.input(1));
        bp.connect(mg_silica_0.output(0), rf_silica_0.input(0));
        bp.connect(mg_silica_1.output(0), rf_silica_1.input(0));
        bp.connect(rf_silica_0.output(0), mg_plate.input(0));
        bp.connect(rf_silica_1.output(0), mg_plate.input(1));
        bp.connect(mg_plate.output(0), dh_plate.input(0));
        let out_gloom = dh_plate.output(0);

        // astral sheet
        bp.connect(out_silver, uf_bright.input(0));
        bp.connect(out_gloom, uf_bright.input(1));
        bp.connect(out_bar, uf_sheet.input(0));
        bp.connect(uf_bright.output(0), uf_sheet.input(1));
        let in_pure = uf_sheet.input(2);
        inputs.push(in_pure);
        bp.connect(uf_sheet.output(0), bm_sheet.input(i));
    });

    Blueprint {
        contents: bp,
        size: Size {
            w: factory_x,
            h: factory_y,
        },
        inputs,
        outputs,
    }
}

pub fn chassis_demo() -> World {
    let mut world = World::new();
    let pure_vault_count = 6;
    let cf = world.place(&chassis_factory(), 0, 0);

    let sv_g_bp = &storage(8 * 4, 4, GoldCoin);
    let sv_gold = world.place(sv_g_bp, -sv_g_bp.width(), 0);
    // the overflowing gold simply loops
    world.connect(sv_gold.output(0), cf.input(0));
    world.connect(cf.output(0), sv_gold.input(0));

    let sv_bp = &storage(pure_vault_count, 1, PureManaGem);
    let _ = stack::<_, 5>(|i| {
        let sv = world.place(sv_bp, -sv_bp.width(), sv_g_bp.height() + i * sv_bp.height());
        world.connect(sv.output(0), cf.input(i as usize + 1));
    });
    let ads = stack::<_, 3>(|i| {
        let ad_salt = world.place(
            AbysalDoor,
            Laboratory.width() + AbysalDoor.width() * i,
            -AbysalDoor.height(),
        );
        world.connect(cf.output(2 + i as usize), ad_salt.input(0));
    });
    let infuser = world.place(
        RitualInfuser,
        -RitualInfuser.width(),
        -RitualInfuser.height(),
    );
    world.connect(cf.output(1), infuser.input(0));
    world
}

pub fn pure_chassis_demo() -> World {
    let mut world = World::new();
    let pf = &pure_factory();
    let pfs = stack::<_, 16>(|i| world.place(pf, 0, i * pf.height()));
    let (inputs, outputs) = chain_ports(&mut world, &pfs, [(0, 0), (1, 1)]).unwrap();
    world
}
