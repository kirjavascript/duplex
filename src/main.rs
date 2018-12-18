#[macro_use]
extern crate lazy_static;
extern crate nom;

mod parser;
mod cube;
mod web;

fn main() {
     use cube::*;
     use parser::parse_moves;

     let moves = parse_moves("rUR'URU2r'").unwrap();

     let moves_transform = combine_transforms(
         moves.iter().map(|s|s.get_transform()).collect()
     );

     let mut cube = Cube::new();

     cube.do_transform(&moves_transform);
     println!("{}", cube);

}
