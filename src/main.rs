extern crate nom;

mod parser;
mod cube;

use cube::*;
use parser::parse_moves;

fn main() {
    println!("{:?}", parse_moves("R' F' R U R' U' R' F R2 U' R' U2 R "));
    // mirror / inverse / minverse
    let mut cube = Cube::new();
    cube.edges[0].flip();
    cube.corners[0].twist(false);
    cube.corners[0].twist(true);
    println!("{}", cube);
}
