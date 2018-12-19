mod parser;
mod cube;
mod alg;
mod web;

fn main() { web_main(); }

#[no_mangle]
extern "C" fn web_main() {
     crate::web::interop::panic_hook();

     use crate::cube::*;
     use crate::alg::*;

     let alg = Alg::new("RUR'U'R'FR2U'R'U'RUR'F'").unwrap();

     let mut cube = Cube::new();
     cube.do_transform(&alg.transform);

     console!("{:?}", alg.moves);
     console!("{:?}", alg.invert().moves);
}
