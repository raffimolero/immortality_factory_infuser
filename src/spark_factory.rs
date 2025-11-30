use crate::disharmonizer_stack::disharmonizer_stack;
use std::array;

use immortality_factory_laboratory::prelude::*;

/// inputs: [gold, pure]
///
/// outputs: [gold, spark]
pub fn spark_factory() -> Blueprint {
    let spark_factory = {
        let mut bp = World::new();
        let dhs = bp.place(&disharmonizer_stack(), 0, 0);

        // a lot of things
        let glooms: [PortOut; 4] = array::from_fn(|i| {
            let i = i as i32;
            let merge_x = (i % 2) * (Merger.width() * 3) + 40;
            let merge_y = (i / 2) * Merger.height() + 5;
            let merge0 = bp.place(Merger, merge_x, merge_y);
            let merge1 = bp.place(Merger, merge_x + 1, merge_y);
            let merge2 = bp.place(Merger, merge_x + 2, merge_y);
            let ref_x = 46;
            let ref_y = i * 4;
            let ref0 = bp.place(Refinery, ref_x, ref_y);
            let ref1 = bp.place(Refinery, ref_x, ref_y + 2);
            let dhs_silica_idx = i as usize * 6 + 2;
            let dhs_silica_out0 = dhs.output(dhs_silica_idx);
            let dhs_silica_out1 = dhs.output(dhs_silica_idx + 3);
            let dh_gloom = bp.place(
                StructureData::Disharmonizer {
                    input: ObsidianPlate,
                    outputs: [GloomShard, SilicaPowder, SilicaPowder, Empty],
                },
                52,
                i * 4,
            );
            bp.connect(dhs_silica_out0, merge0.input(0));
            bp.connect(dhs_silica_out1, merge1.input(0));
            bp.connect(merge0.output(0), ref0.input(0));
            bp.connect(merge1.output(0), ref1.input(0));
            bp.connect(ref0.output(0), merge2.input(0));
            bp.connect(ref1.output(0), merge2.input(1));
            bp.connect(merge2.output(0), dh_gloom.input(0));
            bp.connect(dh_gloom.output(1), merge0.input(1));
            bp.connect(dh_gloom.output(2), merge1.input(1));
            dh_gloom.output(0)
        });

        let merges_coin: [Structure; 5] = array::from_fn(|i| {
            let merge_x = 56 + i as i32;
            let merge_y = 5;
            bp.place(BigMerger, merge_x, merge_y)
        });
        for w in merges_coin[0..3].windows(2) {
            bp.connect(w[0].output(0), w[1].input(0));
        }
        let split_pure = bp.place(Splitter, 61, 5);
        let merge_spark = bp.place(Merger, 61, 8);

        // sell dust
        let sells: [Structure; 16] = array::from_fn(|i| {
            let i = i as i32;
            let sell_x = (i / 2) * SubdimensionalMarket.width();
            let sell_y = (i % 2) * SubdimensionalMarket.height() + dhs.height();
            let sell = bp.place(SubdimensionalMarket, sell_x, sell_y);
            let dhs_dust_idx = (i as usize / 2) * 3 + (i as usize % 2);
            bp.connect(dhs.output(dhs_dust_idx), sell.input(0));

            if i < 6 {
                bp.connect(
                    sell.output(1),
                    merges_coin[3 + (i as usize % 2)].input(4 - (i as usize / 2)),
                );
            } else {
                let i = i - 6;
                let merge_struct_idx = i as usize / 4;
                let merge_port_idx = i as usize % 4 + 1;
                bp.connect(
                    sell.output(0),
                    merges_coin[merge_struct_idx].input(merge_port_idx),
                );
            }

            sell
        });

        for i in 0..4 {
            bp.connect(sells[i].output(2), dhs.input(i));
        }

        // unify orb and spark
        for i in 0..2 {
            // unify orb
            let orb = {
                let uf_x = 56;
                let uf_y = i * 11;
                let uf_bright = bp.place(
                    StructureData::Unifier {
                        inputs: [Empty, SilverCoin, Empty],
                        output: BrightShard,
                    },
                    uf_x,
                    uf_y,
                );

                let uf_orb = bp.place(
                    StructureData::Unifier {
                        inputs: [GloomShard, BrightShard, Empty],
                        output: Empty,
                    },
                    uf_x + Unifier.width(),
                    uf_y,
                );
                let gloom_idx = i as usize * 2;
                let dhs_salt_idx = 3 * 8 + (2 * (i as usize * 2));
                bp.connect(glooms[gloom_idx + 1], uf_bright.input(0));
                bp.connect(merges_coin[3 + i as usize].output(0), uf_bright.input(1));
                bp.connect(glooms[gloom_idx], uf_orb.input(0));
                bp.connect(uf_bright.output(0), uf_orb.input(1));
                bp.connect(dhs.output(dhs_salt_idx), uf_orb.input(2));
                uf_orb.output(0)
            };

            // create life force
            let life = {
                let i = i as i32;
                let uf_x = (i / 2 + 8) * SubdimensionalMarket.width();
                let uf_y = (i % 2) * Unifier.height() + dhs.height();
                let uf_life = bp.place(
                    StructureData::Unifier {
                        inputs: [VialOfBlood, VialOfBlood, Empty],
                        output: Empty,
                    },
                    uf_x,
                    uf_y,
                );
                let dhs_salt_idx = 3 * 8 + (2 * (i as usize * 2 + 1));
                let dhs_blood_idx_0 = 3 * 8 + (1) + (i as usize * 4);
                let dhs_blood_idx_1 = 3 * 8 + (1) + (i as usize * 4 + 2);
                bp.connect(dhs.output(dhs_blood_idx_0), uf_life.input(0));
                bp.connect(dhs.output(dhs_blood_idx_1), uf_life.input(1));
                bp.connect(dhs.output(dhs_salt_idx), uf_life.input(2));
                uf_life.output(0)
            };

            let (pure, spark) = {
                let uf_x = (i / 2 + 8) * SubdimensionalMarket.width() + Unifier.width();
                let uf_y = (i % 2) * Unifier.height() + dhs.height();
                let uf_spark = bp.place(Unifier, uf_x, uf_y);
                bp.connect(orb, uf_spark.input(1));
                bp.connect(life, uf_spark.input(2));
                (uf_spark.input(0), uf_spark.output(0))
            };
            bp.connect(split_pure.output(i as usize), pure);
            bp.connect(spark, merge_spark.input(i as usize));
        }

        Blueprint {
            contents: bp,
            size: Size {
                w: 62,
                h: dhs.height() + SubdimensionalMarket.height() * 2,
            },
            inputs: vec![merges_coin[0].input(0), split_pure.input(0)],
            outputs: vec![merges_coin[2].output(0), merge_spark.output(0)],
        }
    };
    spark_factory
}
