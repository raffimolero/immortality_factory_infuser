//! stuff that i think doesnt belong in the main immortality factory library but probably does

use crate::prelude::*;

use std::{
    array,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

/// convenience function to make array::from_fn() cleaner to call
pub fn stack<T, const N: usize>(mut f: impl FnMut(i32) -> T) -> [T; N] {
    array::from_fn(|i| f(i as i32))
}

/// convenience function to make Vec::from_iter() cleaner to call
pub fn stack_vec<T>(n: usize, f: impl FnMut(i32) -> T) -> Vec<T> {
    Vec::from_iter((0..n as i32).map(f))
}

/// chains the outputs of each machine to the inputs of the next machine in an iterator,
/// based on input/output port indices specified in port_mappings.
/// currently does not support chaining different machines with different port indices together.
///
/// port_mappings follows the form [(input_index, output_index); N]
///
/// output is the first set of inputs and the last set of outputs
pub fn chain_ports<'a, I, V, const N: usize>(
    world: &mut World,
    machines: I,
    port_mappings: [(usize, usize); N],
) -> Option<([PortIn; N], [PortOut; N])>
where
    I: IntoIterator<Item = &'a V>,
    V: Machine + 'a,
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
    cur
}

/// convenience function that converts chain_ports output into 2 separate vecs.
pub fn split_inputs_outputs<const N: usize>(
    ports: Option<([PortIn; N], [PortOut; N])>,
) -> (Vec<PortIn>, Vec<PortOut>) {
    (
        ports
            .into_iter()
            .map(|(inputs, _outputs)| inputs)
            .flatten()
            .collect(),
        ports
            .into_iter()
            .map(|(_inputs, outputs)| outputs)
            .flatten()
            .collect(),
    )
}

/// creates or overwrites a file with the contents of the world.
pub fn export<P: AsRef<Path>>(world: &World, path: P) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    world.export(&mut writer).and_then(|()| writer.flush())
}
