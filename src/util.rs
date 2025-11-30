use crate::prelude::*;

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn stack<T, const N: usize>(mut f: impl FnMut(i32) -> T) -> [T; N] {
    use std::array;
    array::from_fn(|i| f(i as i32))
}

/// port_mappings follows the form [(input_index, output_index); N]
///
/// output is a tuple of options for convenience
pub fn chain_ports<I, V, const N: usize>(
    world: &mut World,
    machines: I,
    port_mappings: [(usize, usize); N],
) -> (Option<[PortIn; N]>, Option<[PortOut; N]>)
where
    I: IntoIterator<Item = V>,
    V: Machine,
{
    let input_indices = port_mappings.map(|(input, _output)| input);
    let output_indices = port_mappings.map(|(_input, output)| output);

    let mut cur: Option<([PortIn; N], [PortOut; N])> = None;
    for m in machines {
        let inputs = match cur.take() {
            None => input_indices.map(|i| m.input(i)),
            Some((prev_ins, prev_outs)) => {
                for (output, i) in prev_outs.into_iter().zip(input_indices) {
                    world.connect(output, m.input(i));
                }
                prev_ins
            }
        };
        let outputs = output_indices.map(|i| m.output(i));
        cur = Some((inputs, outputs));
    }
    (
        cur.map(|(inputs, _outputs)| inputs),
        cur.map(|(_inputs, outputs)| outputs),
    )
}

pub fn export<P: AsRef<Path>>(world: &World, path: P) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    world.export(&mut writer).and_then(|()| writer.flush())
}
