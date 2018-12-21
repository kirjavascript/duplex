#[macro_use]
mod web;
mod parser;
mod cube;
mod alg;

fn main() {
     console!("solver loaded");
     use crate::cube::*;
     use crate::alg::*;

     // let alg = Alg::new("FR'F'R'F2LDRD'L'R'F2R2").unwrap();

     // let mut cube = Cube::new();
     // cube.do_transform(&alg.transform);
     // console!("{}", cube.is_ll_solved());
     // cube.do_transform(&alg.invert().transform);
     // console!("{}", cube.is_ll_solved());
     // cube.do_transform(&Move{order:Order::Prime,layer:Layer::U}.get_transform());
     // console!("{}", cube.is_ll_solved());


}
