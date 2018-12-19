mod parser;
mod cube;
mod alg;
mod web;

fn main() {
     use crate::cube::*;
     use crate::alg::*;

     let alg = Alg::new("rUR'URU2r'").unwrap();

     let mut cube = Cube::new();
     cube.do_transform(&alg.transform);
     console!("{}", cube.is_ll_solved());
     cube.do_transform(&alg.invert().transform);
     console!("{}", cube.is_ll_solved());
     cube.do_transform(&Move{order:Order::Normal,layer:Layer::U}.get_transform());
     console!("{}", cube.is_ll_solved());

     console!("{:?}", alg.moves);
}

// TODO: is_ll_solved, is_ll_transform, is_f2l_solved
