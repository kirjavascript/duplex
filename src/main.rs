#[macro_use]
extern crate lazy_static;
extern crate nom;

mod parser;
mod cube;

use cube::*;
use parser::parse_moves;

fn main() {
    let moves = parse_moves("RUR'U'R'FR2U'R'U'RUR'F'").unwrap();

    let moves_transform = combine_transforms(
        moves.iter().map(|s|s.get_transform()).collect()
    );

    let mut cube = Cube::new();
    // println!("{:#?}", moves_transform);

    cube.do_transform(moves_transform);
    println!("{}", cube);
}
