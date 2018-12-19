#[macro_use]
mod web;
mod parser;
mod cube;
mod alg;

fn main() {
     use crate::cube::*;
     use crate::alg::*;
     use crate::parser::*;

         let moves = parse_moves("FR'F'R'F2LDRD'L'R'F2R2").unwrap();
         let transform = moves_to_transform(&moves);
         let mut cube = Cube::new();
         cube.do_transform(&transform);
         console!("{}", cube);

     // let alg = Alg::new("FR'F'R'F2LDRD'L'R'F2R2").unwrap();

     // let mut cube = Cube::new();
     // cube.do_transform(&alg.transform);
     // console!("{}", cube.is_ll_solved());
     // cube.do_transform(&alg.invert().transform);
     // console!("{}", cube.is_ll_solved());
     // cube.do_transform(&Move{order:Order::Prime,layer:Layer::U}.get_transform());
     // console!("{}", cube.is_ll_solved());

     // console!("{:?}", alg.moves);
}

// TODO: is_ll_transform, is_f2l_solved
