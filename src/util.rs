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

pub fn export<P: AsRef<Path>>(world: &World, path: P) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    world.export(&mut writer).and_then(|()| writer.flush())
}
