#[macro_use]
mod web;
mod parser;
mod cube;
mod alg;
mod enumerate;

fn main() {
     console!("wasm says hi");

     // use crate::cube::*;
     // use crate::alg::*;
     // use crate::enumerate::*;

     // let alg = Alg::new("M'U'MU2M'U'M", "name").unwrap();

     // let mut cube = Cube::new();
     // cube.do_transform(&alg.transform);
     // console!("\n{}", cube);
}
