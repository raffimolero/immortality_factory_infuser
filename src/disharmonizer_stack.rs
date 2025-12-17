use immortality_factory_laboratory::prelude::*;

/// inputs: [copper] * 4
///
/// outputs: [dust, dust, silica] * 4 + [salt, blood] * 4
pub fn disharmonizer_stack() -> Blueprint {
    // outputs: [dust, dust, curse, silica] * 4
    let disharm_half = |merge_y| {
        let mut bp = World::new();

        // mana disharmonizers and refinery stacks
        let mut outputs = Vec::with_capacity(16);
        let Size { w, h } = Disharmonizer.size();
        for (i, (dx, dy)) in [(0, 0), (w, 0), (0, h), (w, h)].into_iter().enumerate() {
            let i = i as Coord;
            let merge_x = 24 + i;
            let merge = bp.place(BigMerger, merge_x, merge_y);

            let ref_y = i * 2;
            for j in 0..3 {
                let ref_x = j * 8;
                let pump = bp.place(AirPump, ref_x, ref_y);
                let refine = bp.place(Refinery, ref_x + 2, ref_y);
                bp.connect(pump.output(0), refine.input(0));
                bp.connect(refine.output(0), merge.input(j as usize));
            }

            let dh = bp.place(Disharmonizer, 28 + dx, dy);
            bp.connect(merge.output(0), dh.input(0));
            outputs.extend((0..4).map(|i| dh.output(i)));
        }

        Blueprint {
            contents: bp,
            size: Size { w: 36, h: 8 },
            inputs: vec![],
            outputs,
        }
    };
    // inputs: [copper] * 4
    // outputs: [dust, dust, silica] * 4 + [salt, blood] * 4
    let disharm_stack = {
        let mut bp = World::new();
        let mut inputs = vec![];
        let mut outputs = vec![];

        // track the current width of the machine
        let mut bp_w = 0;

        // 2 disharmonizer factories placed so that there are gaps in the middle for mergers
        let top = bp.place(&disharm_half(0), 0, 0);
        let bot = bp.place(&disharm_half(2), 0, top.height());
        bp_w += top.width();
        for dhs in [&top, &bot] {
            for i in 0..4 {
                outputs.push(dhs.output(i * 4 + 0)); // mana dust
                outputs.push(dhs.output(i * 4 + 1)); // mana dust
                outputs.push(dhs.output(i * 4 + 3)); // silica powder
            }
        }

        // place mergers in empty space
        let mergers = [24, 25, 26, 27].map(|x| bp.place(Merger, x, 6));
        // does not increase width

        // wire disharmonizers to mergers in a pattern
        // needs a bunch of math to compute
        for i in 0..8 {
            let merger = mergers[i / 2];
            let merger_input = i % 2;
            let half = [&top, &bot][i / 4];
            let output_per_disharm = 4;
            let curse_slot = 2;
            let disharm_output = (i % 4) * output_per_disharm + curse_slot;
            bp.connect(half.output(disharm_output), merger.input(merger_input));
        }

        // make curse disharmonizers and blood unifiers
        let Size { w, h } = Disharmonizer.size();
        for (i, ((dh_x, dh_y), (uf_x, uf_y))) in [
            ((0, h * 0), (w, 0)),
            ((0, h * 1), (w + 3, 0)),
            ((0, h * 2), (w, h * 3 - 1)),
            ((0, h * 3), (w + 3, h * 3 - 1)),
        ]
        .into_iter()
        .enumerate()
        {
            let dh = bp.place(Disharmonizer, bp_w + dh_x, dh_y);
            let uf = bp.place(Unifier, bp_w + uf_x, uf_y);
            bp.connect(mergers[i].output(0), dh.input(0));
            bp.connect(dh.output(1), uf.input(0));
            bp.connect(dh.output(2), uf.input(1));
            inputs.push(uf.input(2)); // copper coin
            outputs.push(dh.output(0)); // chaos salt
            outputs.push(uf.output(0)); // blood vial
        }
        bp_w += w * 3;

        Blueprint {
            contents: bp,
            size: Size {
                w: bp_w,
                h: top.height() * 2,
            },
            inputs,
            outputs,
        }
    };
    disharm_stack
}
