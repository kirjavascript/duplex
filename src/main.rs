#[macro_use]
extern crate lazy_static;
extern crate nom;

mod parser;
mod cube;

use cube::*;
use parser::parse_moves;

fn main() {
    let moves = parse_moves("MUM'U'MUM'U'").unwrap();

    let moves_transform = combine_transforms(
        moves.iter().map(|s|s.get_transform()).collect()
    );

    let mut cube = Cube::new();

    cube.do_transform(&moves_transform);
    println!("{}", cube);
}
